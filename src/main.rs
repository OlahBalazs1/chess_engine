use std::{
    hint::black_box,
    io::{stdin, BufRead},
    time::SystemTime,
};

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
mod search_masks;
mod zobrist;

fn main() {
    for _ in 0..1000 {
        let board = SearchBoard::default();
        let (all_moves, check_path) = board.find_all_moves();
        for mov in all_moves {
            let _ = black_box(mov);
        }
    }
}
