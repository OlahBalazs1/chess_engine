use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        negamax::{self, negamax},
    },
    moving::Move,
};

#[derive(Clone)]
pub struct SearchContext {
    // general
    pub(super) board: SearchBoard,
    pub(super) repetitions: RepetitionHashmap,

    // quiescence
    pub(super) quiescence_depth_limit: i32,
}

impl SearchContext {
    pub fn new(board: SearchBoard) -> Self {
        Self {
            board,
            repetitions: HashMap::with_hasher(BuildNoHashHasher::new()),
            quiescence_depth_limit: 2,
        }
    }
    pub fn find_move(&mut self, min_depth: i32, max_depth: i32) -> Option<Move> {
        negamax(self, min_depth).get(0).copied()
    }
}
