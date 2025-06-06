use std::{
    alloc::System,
    hint::{black_box, unreachable_unchecked},
    sync::LazyLock,
    time::SystemTime,
};

use board::{BoardState, SearchBoard};
use engine::COUNTER;
use magic_bitboards::{print_bits, MAGIC_MOVER};
use position::Position;
use search_data::{CheckPath, PinState};

mod board;
mod engine;
mod hashers;
mod magic_bitboards;
mod moving;
mod piece;
mod position;
mod search;
mod search_data;
mod search_masks;
mod utils;
mod zobrist;

fn main() {
    // let board = SearchBoard::from_fen(
    //     "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    // );
    // println!("{}", board.state)
    // let board = SearchBoard::default();
    // let (_, _) = board.find_all_moves();
    //
    // println!("{}", std::mem::size_of::<BoardState>())
    let _ = LazyLock::force(&MAGIC_MOVER);

    const DEPTH: usize = 5;

    let counted;
    unsafe {
        counted = COUNTER;
    }
    for _ in 0..3 {
        let start = SystemTime::now();
        println!("copy_make: {:?}", engine::perft_copy::<DEPTH>());
        println!("elapsed: {}", start.elapsed().unwrap().as_millis());
        let start = SystemTime::now();
        println!("unmake: {:?}", engine::perft::<DEPTH>());
        println!("elapsed: {}", start.elapsed().unwrap().as_millis());
    }
}
