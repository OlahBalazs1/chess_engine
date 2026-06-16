use std::{cmp, collections::HashMap, hint::black_box};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        evaluate::{Outcome, evaluate, evaluate_outcome, outcome, rate_move},
        who2move,
    },
    moving::{Move, MoveType, Unmove},
};

#[derive(Clone)]
pub struct SearchContext {
    // general
    pub board: SearchBoard,
    pub repetitions: RepetitionHashmap,
    pub evaluated_move: Move,

    // quiescence
    pub(super) quiescence_depth_limit: i32,
}

impl SearchContext {
    pub fn new(
        mut board: SearchBoard,
        repetitions: RepetitionHashmap,
        evaluated_move: Move,
    ) -> Self {
        board.make(&evaluated_move);
        Self {
            board,
            repetitions,
            evaluated_move,
            quiescence_depth_limit: 2,
        }
    }

    pub fn board(&self) -> &SearchBoard {
        return &self.board;
    }

    pub fn evaluate(&mut self, min_depth: i32, max_depth: i32) -> (Move, i64) {
        let eval = self.evaluate_inner(min_depth, i64::MIN + 1, i64::MAX);

        return (self.evaluated_move, -eval);
    }

    fn evaluate_inner(&mut self, depth: i32, mut alpha: i64, beta: i64) -> i64 {
        if depth == 0 {
            return self.quiesce(alpha, beta, 0);
        }
        let (pin_state, check_paths) = self.board().legal_data();
        let is_check = check_paths.is_check();
        let mut moves = self.board().find_all_moves(pin_state, check_paths, false);
        moves.sort_by_cached_key(|mov| -rate_move(mov, self.board().side()));
        let mut eval = i64::MIN + 1;

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
            alpha = cmp::max(alpha, eval);

            // rebind it because of the borrow checker
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition -= 1;
            self.board.unmake(unmake);

            if eval >= beta {
                return eval;
            }
        }
        eval
    }

    fn quiesce(&mut self, mut alpha: i64, beta: i64, descended: i32) -> i64 {
        if descended == self.quiescence_depth_limit {
            return evaluate(self.board(), &self.repetitions, 0);
        }
        let (pin_state, check_paths) = self.board().legal_data();
        let is_check = check_paths.is_check();
        let mut moves = self.board().find_all_moves(pin_state, check_paths, true);
        moves.sort_by_cached_key(|mov| -rate_move(mov, self.board().side()));
        let mut eval = i64::MIN + 1;

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
            let score = -self.quiesce(-beta, -alpha, descended + 1);
            eval = cmp::max(score, eval);
            alpha = cmp::max(alpha, eval);

            // rebind it because of the borrow checker
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition -= 1;
            self.board.unmake(unmake);

            if eval >= beta {
                return eval;
            }
        }
        eval
    }
}
