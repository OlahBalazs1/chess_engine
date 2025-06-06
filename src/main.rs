use std::{
    alloc::System,
    hint::{black_box, unreachable_unchecked},
    time::SystemTime,
};

use board::SearchBoard;
use engine::COUNTER;
use magic_bitboards::print_bits;
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
    const DEPTH: usize = 4;

    // let start = SystemTime::now();
    println!("{:?}", engine::perft::<DEPTH>());
    // println!("elapsed: {}", start.elapsed().unwrap().as_millis());
    let counted;
    unsafe {
        counted = COUNTER;
    }
    println!("{}", counted)
    // println!("{:?}", engine::perft_copy::<DEPTH>());
}
