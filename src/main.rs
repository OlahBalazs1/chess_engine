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

const KIWIPETE_TARGETS: [u64; 6] = [48, 2039, 97862, 4085603, 193690690, 8031647685];
const POS3_TARGETS: [u64; 8] = [
    14, 191, 2812, 43238, 674624, 11030083, 178633661, 3009794393,
];
const POS4_TARGETS: [u64; 6] = [6, 264, 9467, 422333, 15833292, 706045033];

fn main() {
    const DEPTH: usize = 3;
    let board =
        SearchBoard::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    // test::<DEPTH>();
    pseudo_test_custom::<DEPTH>(board, POS4_TARGETS.to_vec());

    // test::<DEPTH>();
}

fn run_tests() {
    test_rook_indices();
}
