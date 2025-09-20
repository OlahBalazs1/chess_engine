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
    util::{max_index, min_index},
};
type ZobristHash = u64;
type RepetitionHashmap = HashMap<ZobristHash, u8, BuildNoHashHasher<u64>>;

pub fn minimax(board: SearchBoard, depth: i32, repetitions: &RepetitionHashmap) -> Option<Move> {
    if depth == 0 {
        panic!("Don't call minimax() with a depth of 0")
    }
    let (pin_state, check_paths) = board.legal_data();
    let is_check = check_paths.is_check();
    let moves = board.find_all_moves(pin_state, check_paths);
    let evals = moves
        .par_iter()
        .map(|mov| {
            let mut board_copy = board.clone();
            let mut repetition_copy = repetitions.clone();
            board_copy.make(&mov);
            add_board_to_repetition(&mut repetition_copy, &board_copy);
            minimax_eval(&mut board_copy, depth, &repetition_copy, i64::MIN, i64::MAX)
        })
        .collect::<Vec<_>>();
    let best_move = if board.side() == Side::White {
        max_index(&evals).map(|i| moves[i])
    } else {
        min_index(&evals).map(|i| moves[i])
    };
    best_move
}

fn minimax_eval(
    board: &mut SearchBoard,
    depth: i32,
    repetitions: &RepetitionHashmap,
    mut alpha: i64,
    mut beta: i64,
) -> i64 {
    if depth == 0 {
        return evaluate(&board, repetitions);
    }
    let (pin_state, check_paths) = board.legal_data();
    let is_check = check_paths.is_check();
    let moves = board.find_all_moves(pin_state, check_paths);
    // board.side() = player
    // For black, a large negative number is a good evaluation
    // For white, it's positive
    let mut best = if board.side() == Side::White {
        i64::MIN
    } else {
        i64::MAX
    };
    for mov in moves.iter() {
        let mut repetition_copy = repetitions.clone();
        let unmake = Unmove::new(mov, &board);
        board.make(mov);

        add_board_to_repetition(&mut repetition_copy, board);
        let score = minimax_eval(board, depth - 1, &repetition_copy, alpha, beta);
        // here board.side == enemy
        board.unmake(unmake);
        // after unmake, it's the player
        if board.side() == Side::White {
            best = best.max(score);
            if best >= beta {
                break;
            }
            alpha = alpha.max(best)
        } else {
            best = best.min(score);
            if best <= alpha {
                break;
            }
            beta = beta.min(best)
        }
    }
    let outcome = outcome(board, &moves, is_check, repetitions);
    match outcome {
        Outcome::Stalemate => 0,
        // if it's Ongoing, best is the best eval
        // if it ended in checkmate, "best" is the worst for the current side
        _ => best,
    }
}

// fn get_relevant_eval(evals: &[i64], side: Side) -> Option<(usize, i64)> {
//     match side {
//         Side::White => evals
//             .iter()
//             .copied()
//             .enumerate()
//             .max_by(|(_, x), (_, y)| x.cmp(y))
//             .clone(),
//         Side::Black => evals
//             .iter()
//             .copied()
//             .enumerate()
//             .min_by(|(_, x), (_, y)| x.cmp(y))
//             .clone(),
//     }
// }

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
