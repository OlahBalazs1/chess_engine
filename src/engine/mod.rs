pub mod evaluate;
pub mod play;
use rayon::prelude::*;
use std::{cmp, collections::HashMap, iter::zip};

use nohash_hasher::{BuildNoHashHasher, NoHashHasher};

use self::evaluate::*;
use crate::{
    board::{self, BoardState, SearchBoard},
    moving::{Move, Unmove},
    piece::{PieceType, Side},
    search_data::CheckPath,
};
type ZobristHash = u64;
type RepetitionHashmap = HashMap<ZobristHash, u8, BuildNoHashHasher<u64>>;
pub fn negamax(board: SearchBoard, depth: i32, board_repetition: &RepetitionHashmap) -> Move {
    if depth == 0 {
        panic!("Don't call minimax() with a depth of 0")
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);
    let evals = moves
        .par_iter()
        .map(|mov| {
            let mut board_copy = board.clone();
            let mut repetition_copy = board_repetition.clone();
            board_copy.make(&mov);
            add_board_to_repetition(&mut repetition_copy, &board_copy);
            -negamax_search(&mut board_copy, depth - 1, &repetition_copy)
        })
        .collect::<Vec<_>>();

    moves[max_eval_index(&evals).unwrap()]
}
fn negamax_search(
    board: &mut SearchBoard,
    depth: i32,
    board_repetition: &RepetitionHashmap,
) -> i64 {
    if depth == 0 {
        return evaluate(&board);
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);
    let mut max = i64::MIN;
    for mov in moves.iter() {
        let mut repetition_copy = board_repetition.clone();
        let unmake = Unmove::new(mov, &board);
        board.make(mov);

        add_board_to_repetition(&mut repetition_copy, board);
        let score = -negamax_search(board, depth - 1, &repetition_copy);
        max = cmp::max(score, max);
        board.unmake(unmake);
    }
    max
}

pub fn minimax(
    board: SearchBoard,
    depth: i32,
    board_repetition: &RepetitionHashmap,
    who_to_play: Side,
) -> Move {
    if depth == 0 {
        panic!("Don't call minimax() with a depth of 0")
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);

    let evals = moves
        .par_iter()
        .map(|mov| {
            let mut board_copy = board.clone();
            let mut repetition_copy = board_repetition.clone();
            board_copy.make(&mov);
            add_board_to_repetition(&mut repetition_copy, &board_copy);
            if who_to_play == Side::White {
                maxi(&mut board_copy, depth - 1, &repetition_copy)
            } else {
                mini(&mut board_copy, depth - 1, &repetition_copy)
            }
        })
        .collect::<Vec<_>>();

    moves[max_eval_index(&evals).unwrap()]
}

fn max_eval_index(evals: &[i64]) -> Option<usize> {
    if evals.is_empty() {
        return None;
    }
    if evals.len() == 1 {
        return Some(0);
    }

    let mut max = evals[0];
    let mut max_index = 0;
    for (index, i) in evals[1..].iter().enumerate() {
        println!("{} {} {}", max, index, i);
        if *i > max {
            max = *i;
            max_index = index;
        }
    }
    return Some(max_index + 1);
}

fn maxi(board: &mut SearchBoard, depth: i32, board_repetition: &RepetitionHashmap) -> i64 {
    if depth == 0 {
        return evaluate(&board);
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);
    let mut max = i64::MIN;
    for mov in moves.iter() {
        let mut repetition_copy = board_repetition.clone();
        let unmake = Unmove::new(mov, &board);
        board.make(mov);

        add_board_to_repetition(&mut repetition_copy, board);
        let score = mini(board, depth - 1, &repetition_copy);
        if score > max {
            max = score;
        }

        board.unmake(unmake);
    }
    max
}
fn mini(board: &mut SearchBoard, depth: i32, board_repetition: &RepetitionHashmap) -> i64 {
    if is_draw_repetition(board, &board_repetition) {
        return 0;
    }
    if depth == 0 {
        return evaluate(&board);
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);
    let mut min = i64::MAX;
    for mov in moves.iter() {
        let mut repetition_copy = board_repetition.clone();
        let unmake = Unmove::new(mov, &board);
        board.make(mov);

        add_board_to_repetition(&mut repetition_copy, board);
        let score = maxi(board, depth - 1, &repetition_copy);
        if score < min {
            min = score;
        }

        board.unmake(unmake);
    }
    min
}

fn get_relevant_eval(evals: &[i64], side: Side) -> Option<(usize, i64)> {
    match side {
        Side::White => evals
            .iter()
            .copied()
            .enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .clone(),
        Side::Black => evals
            .iter()
            .copied()
            .enumerate()
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .clone(),
    }
}

fn is_permanent(_board: &BoardState, mov: Move) -> bool {
    return PieceType::Pawn == mov.piece_type() || mov.take.is_some();
}

pub fn is_draw_repetition(board: &SearchBoard, repetitions: &RepetitionHashmap) -> bool {
    if let Some(repetition) = repetitions.get(&board.zobrist)
        && *repetition == 3
    {
        true
    } else {
        false
    }
}

pub fn add_board_to_repetition(repetitions: &mut RepetitionHashmap, board: &SearchBoard) {
    let repetition_entry = repetitions.entry(board.zobrist).or_insert(0);
    *repetition_entry += 1;
}

fn who2move(side: Side) -> i8 {
    match side {
        Side::White => 1,
        Side::Black => -1,
    }
}
