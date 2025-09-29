use std::ops::Mul;

use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        constants::{
            BISHOP_POSITIONAL, BISHOP_VALUE, KING_POSITIONAL, KING_VALUE, KNIGHT_POSITIONAL,
            KNIGHT_VALUE, PAWN_POSITIONAL, PAWN_VALUE, QUEEN_POSITIONAL, QUEEN_VALUE,
            ROOK_POSITIONAL, ROOK_VALUE,
        },
        is_draw_repetition, who2move,
    },
    moving::{Move, MoveType},
    piece::{self, Piece, PieceType, Side},
    position::Position,
};
use PieceType::*;

pub fn evaluate(board: &SearchBoard, repetitions: &RepetitionHashmap) -> i64 {
    let (pin_state, check_paths) = board.legal_data();
    let is_check = check_paths.is_check();
    let moves = board.find_all_moves(pin_state, check_paths.clone());

    match outcome(board, &moves, is_check, repetitions) {
        Outcome::Ongoing => eval_score(board) + side_dependent_eval(board, is_check, &moves),
        Outcome::WhiteWon => i64::MAX,
        Outcome::BlackWon => i64::MIN,
        Outcome::Stalemate => 0,
    }
}

pub fn eval_score(board: &SearchBoard) -> i64 {
    let mut eval = 0;
    for pos in (0..64).map(Position::from_index) {
        if let Some(piece) = board.board.board[*pos as usize] {
            eval += get_material(piece);
            eval += get_positional(piece, pos)
        }
    }
    eval
}

pub fn side_dependent_eval(board: &SearchBoard, is_check: bool, moves: &[Move]) -> i64 {
    let mut eval = 0;
    eval += moves.len().isqrt() as i64;
    if is_check {
        eval -= 10
    }

    eval * who2move(board.side()) as i64
}

pub fn outcome(
    board: &SearchBoard,
    moves: &[Move],
    is_check: bool,
    repetitions: &RepetitionHashmap,
) -> Outcome {
    if is_draw_repetition(board, repetitions) {
        return Outcome::Stalemate;
    }
    if !moves.is_empty() {
        return Outcome::Ongoing;
    }
    if is_check {
        match board.side() {
            Side::White => Outcome::BlackWon,
            Side::Black => Outcome::WhiteWon,
        }
    } else {
        Outcome::Stalemate
    }
}

fn get_material(piece: Piece) -> i64 {
    match piece.role() {
        Pawn => PAWN_VALUE,
        Rook => ROOK_VALUE,
        Knight => KNIGHT_VALUE,
        Bishop => BISHOP_VALUE,
        Queen => QUEEN_VALUE,
        King => KING_VALUE,
    }
    .mul(if Side::White == piece.side() { 1 } else { -1 })
}

fn get_positional(piece: Piece, pos: Position) -> i64 {
    (match piece.role() {
        Pawn => PAWN_POSITIONAL,
        Rook => ROOK_POSITIONAL,
        Knight => KNIGHT_POSITIONAL,
        Bishop => BISHOP_POSITIONAL,
        Queen => QUEEN_POSITIONAL,
        King => KING_POSITIONAL,
    }[*pos as usize])
        .mul(if Side::White == piece.side() { 1 } else { -1 })
}
fn rate_move(mov: &Move) -> i64 {
    match mov.move_type {
        // en passant is clearly the best
        MoveType::EnPassant => i64::MAX,
        _ => i64::MIN,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Ongoing,
    WhiteWon,
    BlackWon,
    Stalemate,
}

impl Outcome {
    pub fn is_game_over(self) -> bool {
        matches!(
            self,
            Outcome::WhiteWon | Outcome::BlackWon | Outcome::Stalemate
        )
    }
}
