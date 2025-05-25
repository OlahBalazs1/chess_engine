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
mod zobrist;

fn main() {
    for _ in 0..3 {
        let start = SystemTime::now();
        let board = SearchBoard::default();
        for mov in board.moves_iter() {
            let _ = black_box(mov);
        }
        println!("{}", start.elapsed().unwrap().as_micros());
        println!("------")
    }
}
