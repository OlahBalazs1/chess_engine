use std::{
    collections::{HashMap, btree_map::OccupiedEntry, hash_map::Entry},
    iter,
    ops::Deref,
};

use crate::engine::{
    self, RepetitionHashmap, add_board_to_repetition, is_draw_repetition,
    transposition_table::{self, NodeType, TTableEntry, TranspositionTable},
};
use nohash_hasher::BuildNoHashHasher;
use owo_colors::OwoColorize;
use rayon::prelude::*;

use crate::{
    board::{BoardState, SearchBoard},
    moving::{Move, Unmove},
    piece::{PieceType, Side},
};
use engine::evaluate::*;

pub fn minimax(
    board: SearchBoard,
    depth: i32,
    repetitions: &RepetitionHashmap,
    transposition_table: &mut TranspositionTable,
) -> MinimaxResult {
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
            let mut transposition_copy = transposition_table.clone();
            board_copy.make(&mov);
            add_board_to_repetition(&mut repetition_copy, &board_copy);
            minimax_eval(
                &mut board_copy,
                depth,
                &repetition_copy,
                i64::MIN,
                i64::MAX,
                &mut transposition_copy,
            )
        })
        .map(|i| i.unwrap())
        .collect::<Vec<_>>();

    filter_best(moves.into_iter(), &evals, board.side())
}

pub fn minimax_single_threaded(
    mut board: SearchBoard,
    depth: i32,
    repetitions: &RepetitionHashmap,
    transposition_table: &mut TranspositionTable,
) -> MinimaxResult {
    if depth == 0 {
        panic!("Don't call minimax() with a depth of 0")
    }

    let (pin_state, check_paths) = board.legal_data();
    let mut moves = board.find_all_moves(pin_state, check_paths);
    moves.sort_by_key(|i| -rate_move(i, board.side()));

    let mut alpha = i64::MIN;
    let mut beta = i64::MAX;
    let mut best = if board.side() == Side::White {
        i64::MIN
    } else {
        i64::MAX
    };

    let mut evals = Vec::with_capacity(moves.len());
    for mov in moves.iter() {
        let mut repetition_copy;
        if !is_permanent(&board, &mov) {
            repetition_copy = repetitions.clone();
        } else {
            repetition_copy = HashMap::with_hasher(BuildNoHashHasher::new());
        }
        let unmove = Unmove::new(&mov, &board);
        board.make(&mov);
        add_board_to_repetition(&mut repetition_copy, &board);
        let score = minimax_eval(
            &mut board,
            depth,
            &repetition_copy,
            alpha,
            beta,
            transposition_table,
        )
        .unwrap();
        board.unmake(unmove);
        if board.side() == Side::White {
            if score > best {
                best = score;
            }
            alpha = alpha.max(score);
            if score >= beta {
                break;
            }
        } else {
            if score < best {
                best = score;
            }
            beta = beta.min(score);
            if score <= alpha {
                break;
            }
        }

        evals.push(score);
    }
    transposition_table.insert(
        board.zobrist,
        TTableEntry {
            score: best,
            depth,
            node_type: NodeType::PV,
        },
    );

    filter_best(moves.into_iter(), &evals, board.side())
}

fn minimax_eval(
    board: &mut SearchBoard,
    depth: i32,
    repetitions: &RepetitionHashmap,
    mut alpha: i64,
    mut beta: i64,
    transposition_table: &mut TranspositionTable,
) -> Option<i64> {
    if depth == 0 {
        return Some(evaluate(&board, repetitions));
    }
    if is_draw_repetition(&board, repetitions) {
        return Some(0);
    }
    let (pin_state, check_paths) = board.legal_data();
    let is_check = check_paths.is_check();
    let mut moves = board.find_all_moves(pin_state, check_paths);
    let are_there_moves = !moves.is_empty();
    // sort_by_key() sorts in ascending order -> rate move needs to be negated
    moves.sort_by_key(|i| -rate_move(i, board.side()));
    // board.side() = player
    // For black, a large negative number is a good evaluation
    // For white, a positive large number is good
    let mut best = if board.side() == Side::White {
        i64::MIN
    } else {
        i64::MAX
    };
    let mut beta_cutoff = false;
    let mut exceeded_alpha = false;

    let ttable_contains = match transposition_table.entry(board.zobrist) {
        Entry::Occupied(entry) => {
            if entry.get().depth < depth {
                entry.remove();
                false
            } else {
                let entry = entry.get();
                match entry.node_type {
                    NodeType::PV => {
                        return Some(entry.score);
                    }
                    NodeType::Cut if entry.score >= beta => {
                        return Some(entry.score);
                    }
                    NodeType::All if entry.score < alpha => {
                        return Some(entry.score);
                    }
                    _ => true,
                }
            }
        }
        Entry::Vacant(_) => false,
    };
    for mov in moves {
        if let Some(PieceType::King) = mov.take.map(|i| i.piece_type) {
            println!(
                "King taken: {} {:?}\n{:?}\n{:?}",
                mov, mov, board.state, pin_state
            );
            return None;
        }
        let mut repetition_copy;
        if !is_permanent(board, &mov) {
            repetition_copy = repetitions.clone();
        } else {
            repetition_copy = HashMap::with_hasher(BuildNoHashHasher::new());
        }
        let unmake = Unmove::new(&mov, &board);
        board.make(&mov);

        add_board_to_repetition(&mut repetition_copy, board);
        let score = minimax_eval(
            board,
            depth - 1,
            &repetition_copy,
            alpha,
            beta,
            transposition_table,
        )
        .expect(&format!(
            "King taken: {} {:?}\n{:?}\n{:?}",
            mov, mov, board.state, pin_state
        ));
        // here board.side == enemy
        board.unmake(unmake);
        if score > alpha {
            exceeded_alpha = true;
        }
        // after unmake, it's the player
        if board.side() == Side::White {
            if score > best {
                best = score;
            }
            alpha = alpha.max(score);
            if score >= beta {
                beta_cutoff = true;
                break;
            }
        } else {
            if score < best {
                best = score;
            }
            beta = beta.min(score);
            if score <= alpha {
                break;
            }
        }
    }
    let node_type = if beta_cutoff {
        NodeType::Cut
    } else if !exceeded_alpha {
        NodeType::All
    } else {
        NodeType::PV
    };
    if !ttable_contains {
        transposition_table.insert(
            board.zobrist,
            TTableEntry {
                score: best,
                depth,
                node_type,
            },
        );
    }
    let outcome = outcome(board, are_there_moves, is_check, repetitions);
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

fn filter_best<MovIter: Iterator<Item = Move>>(
    moves: MovIter,
    evals: &[i64],
    side: Side,
) -> MinimaxResult {
    match side {
        Side::White => filter_best_maximize(moves, evals),
        Side::Black => filter_best_minimize(moves, evals),
    }
}
fn filter_best_minimize<MovIter: Iterator<Item = Move>>(
    moves: MovIter,
    evals: &[i64],
) -> MinimaxResult {
    let mut builder = MinimaxResultBuilder::new();
    let mut best_eval = i64::MAX;
    for (mov, eval) in iter::zip(moves, evals) {
        if *eval < best_eval {
            builder.clear();
            best_eval = *eval;
        }
        if *eval == best_eval {
            builder.add(mov);
        }
    }
    builder.finalize()
}
fn filter_best_maximize<MovIter: Iterator<Item = Move>>(
    moves: MovIter,
    evals: &[i64],
) -> MinimaxResult {
    let mut builder = MinimaxResultBuilder::new();
    let mut best_eval = i64::MIN;

    for (mov, eval) in iter::zip(moves, evals) {
        if *eval > best_eval {
            builder.clear();
            best_eval = *eval;
        }
        if *eval == best_eval {
            builder.add(mov);
        }
    }
    builder.finalize()
}
