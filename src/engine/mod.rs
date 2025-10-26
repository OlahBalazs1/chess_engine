use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

use crate::{board::SearchBoard, piece::Side};

pub mod constants;
#[allow(dead_code)]
pub mod evaluate;
pub mod incremental_rating;
#[allow(dead_code)]
pub mod minimax;
#[allow(dead_code)]
pub mod play;
type ZobristHash = u64;
type RepetitionHashmap = HashMap<ZobristHash, u8, BuildNoHashHasher<u64>>;

fn who2move(side: Side) -> i8 {
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
