use std::collections::{HashMap, HashSet};

use crate::{
    board::SearchBoard,
    magic_bitboards::print_bits,
    moving::{Move, MoveType, Unmove},
    piece::PieceType,
    position::Position,
    search_data::CheckPath,
};

pub static mut COUNTER: u32 = 0;

fn increment_counter() {
    unsafe { COUNTER += 1 }
}

pub fn perft<const DEPTH: usize>() -> [u32; DEPTH] {
    let mut results = [0; DEPTH];
    let mut board = SearchBoard::default();
    let attacked_squares = board.state.get_attacked(board.side());

    // println!("{}", board.state);
    perft_search(&mut board, &mut results, DEPTH, attacked_squares);

    return results;
}
use MoveType::*;
use PieceType::*;

fn perft_search<const N: usize>(
    board: &mut SearchBoard,
    results: &mut [u32; N],
    depth: usize,
    attacked_squares: u64,
) {
    if depth == 0 {
        return;
    }
    let (pin_state, check_path) = board.state.legal_data();
    let (moves, attacked_squares) = board.find_all_moves(pin_state, check_path, attacked_squares);
    results[depth - 1] += moves.len() as u32;
    for mov in moves {
        // if depth == 1 {
        //     println!("{}", mov)
        // }
        // let board_clone = board.clone();
        let unmove = Unmove::new(&mov, board);
        board.make(&mov);
        // if let Some(taken) = mov.take {
        //     if mov.piece_type() == Pawn && mov.to().x() == mov.from().x() {
        //         panic!("Bad pawn take")
        //     }
        // }

        // match board.check_paths {
        //     CheckPath::Blockable(path) => {
        //         print_bits(path);
        //         increment_counter();
        //     }
        //     CheckPath::Multiple => panic!("Wrong checkpath"),
        //     CheckPath::None => {}
        // }
        perft_search(board, results, depth - 1, attacked_squares);
        board.unmake(unmove);

        // if board_clone.state != board.state {
        //     println!("depth: {}", results.len() - depth + 1);
        //     println!("{:?}", board_clone.state);
        //     println!("{:?}", board_clone.state.board.get(Position::new(7, 5)));
        //     println!("{:?}", board.state.board.get(Position::new(7, 5)));
        //     println!("After: {}", mov);
        //     // println!("{:?}", board.state); println!("---");
        //     // println!("{:?}", board_clone.state);
        //     return;
        // }
    }
}

pub fn perft_copy<const DEPTH: usize>() -> [u32; DEPTH] {
    let mut results = [0; DEPTH];
    let board = SearchBoard::default();

    let attacked_squares = board.state.get_attacked(board.side());

    // println!("{}", board.state);
    perft_search_copy(board, &mut results, DEPTH, attacked_squares);

    return results;
}

fn perft_search_copy<const N: usize>(
    board: SearchBoard,
    results: &mut [u32; N],
    depth: usize,
    attacked_squares: u64,
) {
    if depth == 0 {
        return;
    }
    let (pin_state, check_path) = board.state.legal_data();
    let (moves, attacked_squares) = board.find_all_moves(pin_state, check_path, attacked_squares);
    results[depth - 1] += moves.len() as u32;
    for mov in moves {
        let mut board_clone = board.clone();
        board_clone.make(&mov);
        // println!("{:?}", board_clone.state);
        // print_bits(board_clone.attacked);
        perft_search_copy(board_clone.clone(), results, depth - 1, attacked_squares);
        // if board_clone != board_archive {
        //     println!("Mismatch: {:?}", mov);
        //     return;
        // }
    }
}
