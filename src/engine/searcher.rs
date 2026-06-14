use std::{cmp, collections::HashMap};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        evaluate::{Outcome, evaluate, evaluate_outcome, outcome, rate_move},
        negamax::{self, negamax},
        who2move,
    },
    moving::{Move, Unmove},
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
        let eval = self.evaluate_inner(min_depth, i64::MIN + 1, i64::MAX - 1);

        return (
            self.evaluated_move,
            eval * who2move(self.board().side().opposite()),
        );
    }

    fn evaluate_inner(&mut self, depth: i32, mut alpha: i64, beta: i64) -> i64 {
        if depth == 0 {
            return evaluate(self);
            // return self.quiesce(-beta, -alpha, 0);
            // return evaluate(&self.board);
        }
        let (pin_state, check_paths) = self.board().legal_data();
        let is_check = check_paths.is_check();
        let mut moves = self.board().find_all_moves(pin_state, check_paths);
        moves.sort_by_cached_key(|mov| -rate_move(mov, self.board().side()));
        let mut eval = i64::MIN + 1;

        if let Some(eval) = evaluate_outcome(self, !moves.is_empty(), is_check, depth) {
            return eval;
        }

        for mov in moves {
            let unmake = Unmove::new(mov, &self.board());
            self.board.make(&mov);
            let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
            *repetition += 1;

            if *repetition == 2 {
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

    // fn quiesce(&mut self, mut alpha: i64, beta: i64, descended: i32) -> i64 {
    //     if descended == self.quiescence_depth_limit {
    //         return evaluate(self);
    //     }

    //     let mut eval = i64::MIN + 1;
    //     let (pin_state, check_paths) = self.board().legal_data();
    //     let in_check = check_paths.is_check();
    //     let mut moves = self.board().find_all_moves(pin_state, check_paths);
    //     moves.retain(|e| e.take.is_some());
    //     moves.sort_by_cached_key(|mov| -rate_move(mov, self.board().side()));

    //     if moves.is_empty() {
    //         if in_check {
    //             return i64::MIN + 1;
    //         } else {
    //             return 0;
    //         }
    //     }

    //     for mov in moves {
    //         if mov.take.is_none() {
    //             continue;
    //         }
    //         let unmake = Unmove::new(mov, &self.board());
    //         self.board.make(&mov);
    //         let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
    //         *repetition += 1;

    //         if *repetition == 2 {
    //             *repetition -= 1;
    //             self.board.unmake(unmake);
    //             return 0;
    //         }
    //         let score = self.quiesce(-beta, -alpha, descended + 1);
    //         eval = cmp::max(score, eval);
    //         alpha = cmp::max(alpha, eval);

    //         // rebind it because of the borrow checker
    //         let repetition = self.repetitions.entry(self.board().zobrist).or_insert(0);
    //         *repetition -= 1;
    //         self.board.unmake(unmake);

    //         if eval >= beta {
    //             return eval;
    //         }
    //     }

    //     eval
    // }
}
