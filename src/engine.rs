use std::collections::{HashMap, HashSet};

use crate::{
    board::SearchBoard,
    magic_bitboards::print_bits,
    moving::{Move, MoveType},
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

    // println!("{}", board.state);
    perft_search(&mut board, &mut results, DEPTH);

    return results;
}
use MoveType::*;
use PieceType::*;

fn perft_search<const N: usize>(board: &mut SearchBoard, results: &mut [u32; N], depth: usize) {
    if depth == 0 {
        return;
    }
    let (moves, attacked_squares) = board.find_all_moves();
    results[depth - 1] += moves.len() as u32;
    for (index, mov) in moves.iter().enumerate() {
        let board_clone = board.clone();
        match mov.move_type {
            LongCastle => panic!("Castle detected"),
            ShortCastle => panic!("Castle detected"),
            _ => {}
        }
        if let Some(_) = mov.take {
            println!("{}", mov);
            increment_counter();
        }
        let unmove = board.make(&mov, attacked_squares);
        // if let Some(taken) = mov.take {
        //     println!("{}", board.state);
        //     println!("-----");
        // }

        // match board.check_paths {
        //     CheckPath::Blockable(path) => {
        //         print_bits(path);
        //         increment_counter();
        //     }
        //     CheckPath::Multiple => panic!("Wrong checkpath"),
        //     CheckPath::None => {}
        // }
        perft_search(board, results, depth - 1);
        board.unmake(unmove);

        if board_clone.state != board.state {
            println!("depth: {}", results.len() - depth + 1);
            println!("{:?}", board_clone.state);
            println!("{:?}", board_clone.state.board.get(Position::new(7, 5)));
            println!("{:?}", board.state.board.get(Position::new(7, 5)));
            println!("After: {}", mov);
            // println!("{:?}", board.state); println!("---");
            // println!("{:?}", board_clone.state);
            return;
        }
    }
}

pub fn perft_copy<const DEPTH: usize>() -> [u32; DEPTH] {
    let mut results = [0; DEPTH];
    let board = SearchBoard::default();

    // println!("{}", board.state);
    perft_search_copy(board, &mut results, DEPTH);

    return results;
}

fn perft_search_copy<const N: usize>(board: SearchBoard, results: &mut [u32; N], depth: usize) {
    if depth == 0 {
        return;
    }
    let (moves, attacked_squares) = board.find_all_moves();
    results[depth - 1] += moves.len() as u32;
    for mov in moves {
        let mut board_clone = board.clone();
        let _ = board_clone.make(&mov, attacked_squares);
        // println!("{:?}", board_clone.state);
        // print_bits(board_clone.attacked);
        perft_search_copy(board_clone.clone(), results, depth - 1);
        // if board_clone != board_archive {
        //     println!("Mismatch: {:?}", mov);
        //     return;
        // }
    }
}
