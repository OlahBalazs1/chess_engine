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

use crate::perft::*;
pub use crate::util::pseudo_moving;

use crate::magic_bitboards::{slide_blocker_possible_moves, test_rook_indices};

fn main() {
    const DEPTH: usize = 5;

    test::<DEPTH>();
}

fn run_tests() {
    test_rook_indices();
}
