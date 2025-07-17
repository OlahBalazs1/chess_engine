use std::{
    alloc::System,
    collections::HashSet,
    hint::{black_box, unreachable_unchecked},
    ops::Index,
    sync::LazyLock,
    time::SystemTime,
};

use board::{BoardState, SearchBoard};
use magic_bitboards::{print_bits, MAGIC_MOVER};
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

fn main() {
    //     "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    // );
    // println!("{}", board.state)
    // let board = SearchBoard::default();
    // let (_, _) = board.find_all_moves();
    //
    // println!("{}", std::mem::size_of::<BoardState>())
    // let _ = LazyLock::force(&MAGIC_MOVER);
    //
    //
    const DEPTH: usize = 5;

    pseudo_test::<DEPTH>();

    // for _ in 0..1 {
    //     let start = SystemTime::now();
    //     println!("copy_make: {:?}", perft_copy::<DEPTH>());
    //     println!("elapsed: {}", start.elapsed().unwrap().as_millis());
    //     let start = SystemTime::now();
    //     println!("unmake: {:?}", perft::<DEPTH>());
    //     println!("elapsed: {}", start.elapsed().unwrap().as_millis());
    // }
    //
    // let board = BoardState::from_fen("7k/8/8/8/3B4/8/8/4K3 w - - 0 1");
    // println!("{}", board);
    // print_bits(board.get_attacked(board.side.opposite()));
    //
    // print_bits(
    //     MAGIC_MOVER
    //         .get_bishop(Position::new(0, 4), 0x7effff0500000000)
    //         .bitboard,
    // );
}
