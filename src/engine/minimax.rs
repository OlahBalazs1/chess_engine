use std::{collections::HashMap, iter, ops::Deref};

use crate::{
    board,
    engine::{self, RepetitionHashmap, add_board_to_repetition},
};
use nohash_hasher::BuildNoHashHasher;
use rayon::prelude::*;

use crate::{
    board::{BoardState, SearchBoard},
    moving::{Move, Unmove},
    piece::{PieceType, Side},
};
use engine::evaluate::*;

pub fn minimax(board: SearchBoard, depth: i32, repetitions: &RepetitionHashmap) -> MinimaxResult {
    if depth == 0 {
        panic!("Don't call minimax() with a depth of 0")
    }
    let (pin_state, check_paths) = board.legal_data();
    // let is_check = check_paths.is_check();
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
        .map(|i| i.unwrap())
        .collect::<Vec<_>>();

    filter_best(&moves, &evals, board.side())
}

fn minimax_eval(
    board: &mut SearchBoard,
    depth: i32,
    repetitions: &RepetitionHashmap,
    mut alpha: i64,
    mut beta: i64,
) -> Option<i64> {
    if depth == 0 {
        return Some(evaluate(&board, repetitions));
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
        if let Some(PieceType::King) = mov.take.map(|i| i.piece_type) {
            println!(
                "King taken: {} {:?}\n{:?}\n{:?}",
                mov, mov, board.state, pin_state
            );
            return None;
        }
        let mut repetition_copy;
        if !is_permanent(board, mov) {
            repetition_copy = repetitions.clone();
        } else {
            repetition_copy = HashMap::with_hasher(BuildNoHashHasher::new());
        }
        let unmake = Unmove::new(mov, &board);
        board.make(mov);

        add_board_to_repetition(&mut repetition_copy, board);
        let Some(score) = minimax_eval(board, depth - 1, &repetition_copy, alpha, beta) else {
            println!(
                "King taken: {} {:?}\n{:?}\n{:?}",
                mov, mov, board.state, pin_state
            );
            return None;
        };
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
        Outcome::Stalemate => Some(0),
        // if it's Ongoing, best is the best eval
        // if it ended in checkmate, "best" is the worst for the current side
        _ => Some(best),
    }
}

fn is_permanent(_board: &BoardState, mov: &Move) -> bool {
    return PieceType::Pawn == mov.piece_type() || mov.take.is_some();
}

pub struct MinimaxResultBuilder {
    best_moves: Vec<Move>,
}
impl MinimaxResultBuilder {
    pub fn new() -> Self {
        Self {
            best_moves: Vec::new(),
        }
    }

    pub fn add(&mut self, mov: Move) {
        self.best_moves.push(mov);
    }

    pub fn clear(&mut self) {
        self.best_moves.clear();
    }

    pub fn finalize(self) -> MinimaxResult {
        MinimaxResult {
            best_moves: self.best_moves,
        }
    }
}
pub struct MinimaxResult {
    best_moves: Vec<Move>,
}

impl Deref for MinimaxResult {
    type Target = Vec<Move>;
    fn deref(&self) -> &Self::Target {
        &self.best_moves
    }
}

impl PartialEq for MinimaxResult {
    fn eq(&self, other: &Self) -> bool {
        // only compare that the other result contains the same moves as the first one, order
        // doesn't matter
        if self.len() != other.len() {
            return false;
        }
        // this has a REALLY bad time complexity, but this is not performance critical
        // Also, n <= 219
        // so it will never have a high cost anyway
        for i in self.iter() {
            if !other.contains(i) {
                return false;
            }
        }
        for i in other.iter() {
            if !self.contains(i) {
                return false;
            }
        }
        true
    }
}

fn filter_best(moves: &[Move], evals: &[i64], side: Side) -> MinimaxResult {
    match side {
        Side::White => filter_best_maximize(moves, evals),
        Side::Black => filter_best_minimize(moves, evals),
    }
}
fn filter_best_minimize(moves: &[Move], evals: &[i64]) -> MinimaxResult {
    let mut builder = MinimaxResultBuilder::new();
    let mut best_eval = i64::MAX;
    for (mov, eval) in iter::zip(moves, evals) {
        if *eval < best_eval {
            builder.clear();
            best_eval = *eval;
        }
        if *eval == best_eval {
            builder.add(*mov);
        }
    }
    builder.finalize()
}
fn filter_best_maximize(moves: &[Move], evals: &[i64]) -> MinimaxResult {
    let mut builder = MinimaxResultBuilder::new();
    let mut best_eval = i64::MIN;

    for (mov, eval) in iter::zip(moves, evals) {
        if *eval > best_eval {
            builder.clear();
            best_eval = *eval;
        }
        if *eval == best_eval {
            builder.add(*mov);
        }
    }
    builder.finalize()
}
