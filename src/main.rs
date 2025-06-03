use std::{hint::black_box, time::SystemTime};

use board::SearchBoard;
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
mod zobrist;

fn main() {
    // let mut board = SearchBoard::default();
    // println!("created: {:?}", board.state);
    // let (all_moves, attacked_squares) = board.find_all_moves();
    // println!("found_moves: {:?}", board.state);
    // let _ = board.make(&all_moves[0], attacked_squares);
    // println!("make: {:?}", board.state);
    // // board.unmake(unmake);
    // // println!("unmake: {:?}", board.state);
    // println!("------");
    // let (all_moves, attacked_squares) = board.find_all_moves();
    // let unmake = board.make(&all_moves[0], attacked_squares);
    // println!("make: {:?}", board.state);
    // board.unmake(unmake);
    // println!("unmake: {:?}", board.state);

    const DEPTH: usize = 6;
    println!("{:?}", engine::perft::<DEPTH>());
    // println!("{:?}", engine::perft_copy::<DEPTH>());
}
