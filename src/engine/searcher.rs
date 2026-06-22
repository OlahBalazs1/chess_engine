use std::{
    cmp,
    collections::{HashMap, hash_map::Entry},
    sync::{Arc, Mutex},
};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        evaluate::{evaluate, evaluate_outcome, rate_move},
        transposition_table::{NodeType, TTableEntry, TranspositionTable},
    },
    moving::{Move, Unmove},
};

pub struct SearchContext {
    // general
    pub board: SearchBoard,
    pub repetitions: RepetitionHashmap,
    pub evaluated_move: Move,

    pub ttable: Arc<Mutex<TranspositionTable>>,
    pub nodes_searched: u32,

    // quiescence
    pub(super) quiescence_depth_limit: i32,
}

impl SearchContext {
    pub fn new(
        mut board: SearchBoard,
        repetitions: RepetitionHashmap,
        evaluated_move: Move,
        ttable: Arc<Mutex<TranspositionTable>>,
    ) -> Self {
        board.make(&evaluated_move);
        Self {
            board,
            repetitions,
            evaluated_move,
            nodes_searched: 0,
            ttable,
            quiescence_depth_limit: 2,
        }
    }

    pub fn board(&self) -> &SearchBoard {
        return &self.board;
    }

    pub fn evaluate(&mut self, min_depth: i32, _max_depth: i32) -> (Move, i64) {
        let eval = self.evaluate_inner(min_depth, i64::MIN + 1, i64::MAX);

        return (self.evaluated_move, -eval);
    }

    fn evaluate_inner(&mut self, depth: i32, mut alpha: i64, beta: i64) -> i64 {
        if let Some(transposition_score) =
            self.ttable
                .lock()
                .unwrap()
                .get(self.board().zobrist, depth, alpha, beta)
        {
            return transposition_score;
        }
        if depth == 0 {
            return self.quiesce(0, alpha, beta);
        }

        let (pin_state, check_paths) = self.board().legal_data();
        let is_check = check_paths.is_check();
        let mut moves = self.board().find_all_moves(pin_state, check_paths, false);
        // outcome
        if let Some(eval) = evaluate_outcome(
            self.board(),
            &self.repetitions,
            !moves.is_empty(),
            is_check,
            depth,
        ) {
            let eval = eval;
            // this code runs when the side to play is checkmated -> negative
            return -eval.abs();
        }
        moves.sort_by_cached_key(|mov| -rate_move(mov, self.board().side()));
        let mut eval = i64::MIN + 1;
        let mut node_type = NodeType::UpperBound;

        self.nodes_searched += 1;

        for mov in moves {
            let unmake = Unmove::new(mov, &self.board());
            self.board.make(&mov);
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition += 1;

            if *repetition > 1 {
                *repetition -= 1;
                self.board.unmake(unmake);
                return 0;
            }
            let score = -self.evaluate_inner(depth - 1, -beta, -alpha);
            eval = cmp::max(score, eval);

            // rebind it because of the borrow checker
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition -= 1;
            self.board.unmake(unmake);

            // fail high
            if eval >= beta {
                node_type = NodeType::LowerBound;
                self.ttable
                    .lock()
                    .unwrap()
                    .insert(self.board().zobrist, eval, depth, node_type);
                return beta;
            }

            if alpha < eval {
                node_type = NodeType::PV;
                alpha = eval
            }
        }
        self.ttable
            .lock()
            .unwrap()
            .insert(self.board().zobrist, eval, depth, node_type);
        alpha
    }

    fn quiesce(&mut self, descended: i32, mut alpha: i64, beta: i64) -> i64 {
        self.nodes_searched += 1;

        if let Some(transposition_score) =
            self.ttable
                .lock()
                .unwrap()
                .get(self.board().zobrist, -descended, alpha, beta)
        {
            return transposition_score;
        }
        if descended == self.quiescence_depth_limit {
            return evaluate(self.board(), &self.repetitions, -descended);
        }

        let (pin_state, check_paths) = self.board().legal_data();
        let is_check = check_paths.is_check();
        let mut moves = self.board().find_all_moves(pin_state, check_paths, true);
        // outcome
        if let Some(eval) = evaluate_outcome(
            self.board(),
            &self.repetitions,
            !moves.is_empty(),
            is_check,
            -descended,
        ) {
            let eval = eval;
            // this code runs when the side to play is checkmated -> negative
            return -eval.abs();
        }
        moves.sort_by_cached_key(|mov| -rate_move(mov, self.board().side()));
        let mut eval = i64::MIN + 1;
        let mut node_type = NodeType::UpperBound;

        for mov in moves {
            let unmake = Unmove::new(mov, &self.board());
            self.board.make(&mov);
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition += 1;

            if *repetition > 1 {
                *repetition -= 1;
                self.board.unmake(unmake);
                return 0;
            }
            let score = -self.quiesce(descended + 1, -beta, -alpha);
            eval = cmp::max(score, eval);

            // rebind it because of the borrow checker
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition -= 1;
            self.board.unmake(unmake);

            // fail high
            if eval >= beta {
                node_type = NodeType::LowerBound;
                self.ttable.lock().unwrap().insert(
                    self.board().zobrist,
                    eval,
                    -descended,
                    node_type,
                );
                return beta;
            }

            if alpha < eval {
                node_type = NodeType::PV;
                alpha = eval
            }
        }
        self.ttable
            .lock()
            .unwrap()
            .insert(self.board().zobrist, eval, -descended, node_type);
        alpha
    }
}
