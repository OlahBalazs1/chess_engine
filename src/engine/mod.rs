use std::{collections::HashMap, ops::Deref};

use nohash_hasher::BuildNoHashHasher;

use crate::{board::SearchBoard, moving::Move, piece::Side};

pub mod constants;
#[allow(dead_code)]
pub mod evaluate;
// pub mod negamax;
#[allow(dead_code)]
pub mod play;
pub mod searcher;
pub mod transposition_table;
type ZobristHash = u64;
type RepetitionHashmap = HashMap<ZobristHash, u8, BuildNoHashHasher<u64>>;

fn who2move(side: Side) -> i64 {
    match side {
        Side::White => 1,
        Side::Black => -1,
    }
}

pub fn is_draw_repetition(board: &SearchBoard, repetitions: &RepetitionHashmap) -> bool {
    if let Some(repetition) = repetitions.get(&board.zobrist)
        && *repetition == 3
    {
        true
    } else {
        false
    }
}

pub fn add_board_to_repetition(repetitions: &mut RepetitionHashmap, board: &SearchBoard) {
    let repetition_entry = repetitions.entry(board.zobrist).or_insert(0);
    *repetition_entry += 1;
}

pub struct MinimaxResult {
    best_moves: Vec<Move>,
}

impl Deref for MinimaxResult {
    type Target = Vec<Move>;
    fn deref(&self) -> &Self::Target {
        &self.best_moves
    }
}

impl PartialEq for MinimaxResult {
    fn eq(&self, other: &Self) -> bool {
        // only compare that the other result contains the same moves as the first one, order
        // doesn't matter
        if self.len() != other.len() {
            return false;
        }
        // this has a REALLY bad time complexity, but this is not performance critical
        // Also, n <= 219
        // so it will never have a high cost anyway
        for i in self.iter() {
            if !other.contains(i) {
                return false;
            }
        }
        for i in other.iter() {
            if !self.contains(i) {
                return false;
            }
        }
        true
    }
}
