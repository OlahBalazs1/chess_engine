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

fn main() {
    const DEPTH: usize = 2;
    // let board = SearchBoard::default();
    let board = SearchBoard::from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    );
    println!("{}", board.state);
    pseudo_test_custom::<DEPTH>(board, KIWIPETE_TARGETS.to_vec());
}

fn run_tests() {
    test_rook_indices();
}
