use std::{
    alloc::System,
    collections::HashSet,
    hint::{black_box, unreachable_unchecked},
    ops::Index,
    sync::LazyLock,
    time::SystemTime,
};

use board::{BoardState, SearchBoard};
use magic_bitboards::{MAGIC_MOVER, print_bits};
use position::Position;
use search_data::{CheckPath, PinState};

mod board;
mod board_repr;
mod engine;
mod hashers;
mod magic_bitboards;
mod moving;
mod perft_data;
mod piece;
mod position;
mod pseudo_moving;
mod search;
mod search_data;
mod search_masks;
mod zobrist;

use engine::perft::*;

use crate::{
    magic_bitboards::{slide_blocker_possible_moves, test_rook_indices},
    pseudo_moving::ROOK_DIRECTIONS,
};

fn main() {
    const DEPTH: usize = 5;

    test::<DEPTH>();
    //
    // let data =
    //     slide_blocker_possible_moves(0xfffe00010101fefe, Position::new(0, 2), ROOK_DIRECTIONS);
    // print!("{}", data.bitboard);
}

fn run_tests() {
    test_rook_indices();
}
