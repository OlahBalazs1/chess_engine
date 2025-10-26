use std::ops::{Add, Mul};

use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        constants::{
            BISHOP_POSITIONAL, BISHOP_VALUE, KING_POSITIONAL, KING_VALUE, KNIGHT_POSITIONAL,
            KNIGHT_VALUE, PAWN_POSITIONAL, PAWN_VALUE, QUEEN_POSITIONAL, QUEEN_VALUE,
            ROOK_POSITIONAL, ROOK_VALUE,
        },
        incremental_rating::IncrementalRating,
        is_draw_repetition, who2move,
    },
    moving::{Move, MoveType},
    piece::{Piece, PieceType, Side},
    position::Position,
};
use PieceType::*;

pub fn evaluate(board: &SearchBoard, repetitions: &RepetitionHashmap) -> i64 {
    let (pin_state, check_paths) = board.legal_data();
    let is_check = check_paths.is_check();
    let moves = board.find_all_moves(pin_state, check_paths.clone());

    match outcome(board, !moves.is_empty(), is_check, repetitions) {
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
            eval += get_material(piece) * 100;
            eval += get_positional(piece, pos)
        }
    }
    eval
}

pub fn side_dependent_eval(board: &SearchBoard, is_check: bool, moves: &[Move]) -> i64 {
    let mut eval = 0;
    eval += moves.len().isqrt() as i64;
    if is_check {
        eval -= 10;
    }

    eval * who2move(board.side()) as i64
}

pub fn outcome(
    board: &SearchBoard,
    are_there_moves: bool,
    is_check: bool,
    repetitions: &RepetitionHashmap,
) -> Outcome {
    if is_draw_repetition(board, repetitions) {
        return Outcome::Stalemate;
    }
    if are_there_moves {
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

pub(crate) fn get_material(piece: Piece) -> i64 {
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

pub(crate) fn get_positional(piece: Piece, pos: Position) -> i64 {
    let lookup_pos = pos.with_y(piece.side().pers_y(pos.y())).unwrap();
    (match piece.role() {
        Pawn => PAWN_POSITIONAL,
        Rook => ROOK_POSITIONAL,
        Knight => KNIGHT_POSITIONAL,
        Bishop => BISHOP_POSITIONAL,
        Queen => QUEEN_POSITIONAL,
        King => KING_POSITIONAL,
    }[*lookup_pos as usize])
        .mul(if Side::White == piece.side() { 1 } else { -1 })
}
pub(crate) fn rate_move(mov: &Move, who_to_move: Side) -> IncrementalRating {
    let mut eval = IncrementalRating::default();

    let piece = mov.piece_type().with_side(who_to_move);
    eval.set_positional(who_to_move, -(get_positional(piece, mov.from)));
    match mov.move_type {
        MoveType::Promotion(promoted_to) => {
            eval.set_material(
                who_to_move,
                get_material(promoted_to.with_side(who_to_move)),
            );
            eval.set_positional(
                who_to_move,
                eval.positional(who_to_move)
                    + get_positional(promoted_to.with_side(who_to_move), mov.to),
            );
        }
        _ => eval.set_positional(
            who_to_move,
            eval.positional(who_to_move) + get_positional(piece, mov.to),
        ),
    }

    if let Some(taken) = mov.take {
        eval.set_material(who_to_move.opposite(), -get_material(taken));
        eval.set_positional(who_to_move.opposite(), -get_positional(taken, mov.to));
    }

    eval
}

#[repr(u8)]
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
