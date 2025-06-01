use std::{hint::black_box, time::SystemTime};

use board::SearchBoard;
use magic_bitboards::print_bits;
use position::Position;
use search_data::{CheckPath, PinState};

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
    let mut board = SearchBoard::default();
    for _ in 0..2 {
        let start = SystemTime::now();
        // print_bits(all_moves.1);
        let pin_state = black_box(PinState::find(&board.state, Position::new(4, 0)));
        let check_path = black_box(CheckPath::find(
            &board.state,
            Position::new(4, 0),
            piece::Side::Black,
        ));

        let (all_moves, attacked_squares) = board.find_all_moves();
        for mov in all_moves {
            let mut board = board.clone();
            let _ = board.make(&mov, attacked_squares);
            board.pin_state = PinState::find(&board.state, Position::new(4, 0));
            board.check_paths =
                CheckPath::find(&board.state, Position::new(4, 0), piece::Side::Black);
            println!("{}", board.state);
            let (all_moves, attacked_squares) = board.find_all_moves();
            for mov in all_moves {
                let mut board = board.clone();
                let _ = board.make(&mov, attacked_squares);
                board.pin_state = PinState::find(&board.state, Position::new(4, 0));
                board.check_paths =
                    CheckPath::find(&board.state, Position::new(4, 0), piece::Side::Black);

                println!("{}", board.state);
            }
        }
        println!("{}", start.elapsed().unwrap().as_nanos());
        println!("------")
    }
    println!("{}", board.state);
    let (all_moves, attacked_squares) = board.find_all_moves();
    println!("{:?}", all_moves);
}
