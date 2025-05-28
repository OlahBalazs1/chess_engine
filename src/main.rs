use std::{hint::black_box, time::SystemTime};

use board::SearchBoard;
use piece::Piece;

mod board;
mod hashers;
mod magic_bitboards;
mod moving;
mod piece;
mod position;
mod search;
mod search_data;
mod zobrist;

fn main() {
    for _ in 0..3 {
        let start = SystemTime::now();
        let board = SearchBoard::default();
        let (all_moves, check_path) = board.find_all_moves();
        for mov in all_moves {
            let _ = println!("{:?}", mov);
        }
        println!("{}", start.elapsed().unwrap().as_micros());
        println!("------")
    }
}
