#![allow(dead_code)]
mod board;
mod board_repr;
mod hashers;
mod magic_bitboards;
mod moving;
mod perft;
mod perft_data;
mod piece;
mod position;
mod search;
mod search_data;
mod search_masks;
mod util;
mod zobrist;

use crate::position::Position;
use crate::search_masks::{BLACK_PAWN_TAKE_MASKS, WHITE_PAWN_TAKE_MASKS};
pub use crate::util::pseudo_moving;
use crate::{board::SearchBoard, perft::*};

use crate::magic_bitboards::{slide_blocker_possible_moves, test_rook_indices};

fn main() {
    const DEPTH: usize = 5;
    let board =
        SearchBoard::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ");
    // test::<DEPTH>();
    test_custom::<DEPTH>(board, POS5_TARGETS.to_vec());

    // test::<DEPTH>();
}

fn run_tests() {
    test_rook_indices();
}
