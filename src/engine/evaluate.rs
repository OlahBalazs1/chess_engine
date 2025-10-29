use crate::{
    board::SearchBoard,
    engine::{
        RepetitionHashmap,
        constants::{
            BISHOP_POSITIONAL, BISHOP_VALUE, CHECK_WEIGHT, KING_POSITIONAL, KING_VALUE,
            KNIGHT_POSITIONAL, KNIGHT_VALUE, MATERIAL_WEIGHT, MOBILITY_WEIGHT, PAWN_POSITIONAL,
            PAWN_VALUE, POSITIONAL_WEIGHT, QUEEN_POSITIONAL, QUEEN_VALUE, ROOK_POSITIONAL,
            ROOK_VALUE,
        },
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
    let mut positional = 0;
    for (index, piece) in board
        .board
        .board
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, i)| i.map(|i| (index, i)))
    {
        let pos = Position::from_index(index as u8);
        positional += get_raw_positional(piece, pos) * who2move(piece.side());

        eval += get_material(piece);
    }
    eval + (positional * POSITIONAL_WEIGHT)
}

pub fn side_dependent_eval(board: &SearchBoard, is_check: bool, moves: &[Move]) -> i64 {
    let mut eval = 0;
    eval += moves.len().isqrt() as i64 * MOBILITY_WEIGHT;
    if is_check {
        eval -= 10 * CHECK_WEIGHT;
    }

    eval * who2move(board.side()) as i64
}

pub fn outcome(
    board: &SearchBoard,
    are_there_moves: bool,
    is_check: bool,
    repetitions: &RepetitionHashmap,
) -> Outcome {
    if board.halfmove_clock >= 50 || is_draw_repetition(board, repetitions) {
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

pub(crate) const fn get_material(piece: Piece) -> i64 {
    let fundamental_value = match piece.role() {
        Pawn => PAWN_VALUE,
        Rook => ROOK_VALUE,
        Knight => KNIGHT_VALUE,
        Bishop => BISHOP_VALUE,
        Queen => QUEEN_VALUE,
        King => KING_VALUE,
    };
    fundamental_value
        * (if let Side::White = piece.side() {
            1
        } else {
            -1
        })
        * MATERIAL_WEIGHT
}
pub(crate) const fn get_raw_material(piece: PieceType) -> i64 {
    match piece {
        Pawn => PAWN_VALUE,
        Rook => ROOK_VALUE,
        Knight => KNIGHT_VALUE,
        Bishop => BISHOP_VALUE,
        Queen => QUEEN_VALUE,
        King => KING_VALUE,
    }
}

pub(crate) const fn get_positional(piece: Piece, pos: Position) -> i64 {
    let lookup_pos = pos.with_y(piece.side().pers_y(pos.y())).unwrap();
    let value = match piece.role() {
        Pawn => PAWN_POSITIONAL,
        Rook => ROOK_POSITIONAL,
        Knight => KNIGHT_POSITIONAL,
        Bishop => BISHOP_POSITIONAL,
        Queen => QUEEN_POSITIONAL,
        King => KING_POSITIONAL,
    }[lookup_pos.index() as usize];
    value
        * (if let Side::White = piece.side() {
            1
        } else {
            -1
        })
        * POSITIONAL_WEIGHT
}
pub(crate) const fn get_raw_positional(piece: Piece, pos: Position) -> i64 {
    let lookup_pos = pos.with_y(piece.side().pers_y(pos.y())).unwrap();
    (match piece.role() {
        Pawn => PAWN_POSITIONAL,
        Rook => ROOK_POSITIONAL,
        Knight => KNIGHT_POSITIONAL,
        Bishop => BISHOP_POSITIONAL,
        Queen => QUEEN_POSITIONAL,
        King => KING_POSITIONAL,
    })[lookup_pos.index() as usize]
}
pub(crate) fn rate_move(mov: &Move, who_to_move: Side) -> i64 {
    let piece = mov.piece_type().with_side(who_to_move);
    // accounts for color
    // good for black -> negative
    // good for white -> positive
    match mov.move_type {
        MoveType::Normal(_) => {
            let mut eval = 0;
            eval -= get_raw_positional(piece, mov.from);
            eval += get_raw_positional(piece, mov.to);
            eval *= POSITIONAL_WEIGHT;
            if let Some(taken) = mov.take {
                eval += get_raw_material(taken.role()) * MATERIAL_WEIGHT;
            }
            eval
        }
        MoveType::Promotion(promoted_to) => get_raw_material(promoted_to).pow(2),
        MoveType::ShortCastle => 20000 * who2move(who_to_move) as i64,
        MoveType::LongCastle => 20000 * who2move(who_to_move) as i64,
        MoveType::EnPassant => 1_000_000 * who2move(who_to_move) as i64,
    }
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
