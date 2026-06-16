use crate::{
    board::SearchBoard,
    board_repr::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK},
    engine::{
        RepetitionHashmap,
        constants::{
            BISHOP_POSITIONAL, BISHOP_VALUE, CHECK_WEIGHT, KING_POSITIONAL, KING_VALUE,
            KNIGHT_POSITIONAL, KNIGHT_VALUE, MATERIAL_WEIGHT, MOBILITY_WEIGHT, PAWN_POSITIONAL,
            PAWN_VALUE, POSITIONAL_WEIGHT, QUEEN_POSITIONAL, QUEEN_VALUE, ROOK_POSITIONAL,
            ROOK_VALUE,
        },
        is_draw_repetition,
        searcher::SearchContext,
        who2move,
    },
    moving::{Move, MoveType},
    piece::{Piece, PieceType, Side},
    position::Position,
};
use PieceType::*;

pub fn evaluate(board: &SearchBoard, repetitions: &RepetitionHashmap, depth: i32) -> i64 {
    let (pin_state, check_paths) = board.legal_data();
    let is_check = check_paths.is_check();
    let moves = board.find_all_moves(pin_state, check_paths, false);

    let mut side_dependent = eval_score(board);
    side_dependent += eval_material(board);

    let mut side_agnostic = moves.len() as i64;
    side_agnostic -= if is_check { 10 } else { 0 };

    if let Some(outcome) = evaluate_outcome(board, repetitions, !moves.is_empty(), is_check, depth)
    {
        return -outcome.abs();
    }

    return side_dependent * if board.side() == Side::White { 1 } else { -1 } + side_agnostic;
}

pub fn evaluate_outcome(
    board: &SearchBoard,
    repetitions: &RepetitionHashmap,
    are_there_moves: bool,
    is_check: bool,
    depth: i32,
) -> Option<i64> {
    Some(
        match outcome(&board, are_there_moves, is_check, &repetitions) {
            Outcome::Ongoing => return None,
            Outcome::WhiteWon => i64::MAX - 10000 + (100 * depth) as i64,
            Outcome::BlackWon => i64::MIN + 10000 - (100 * depth) as i64,
            Outcome::Stalemate => -1000,
        },
    )
}
pub fn eval_material(board: &SearchBoard) -> i64 {
    let mut eval = 0;
    eval += board.side_bitboards(Side::White)[PAWN].count_ones() as i64 * PAWN_VALUE;
    eval += board.side_bitboards(Side::White)[ROOK].count_ones() as i64 * ROOK_VALUE;
    eval += board.side_bitboards(Side::White)[KNIGHT].count_ones() as i64 * KNIGHT_VALUE;
    eval += board.side_bitboards(Side::White)[BISHOP].count_ones() as i64 * BISHOP_VALUE;
    eval += board.side_bitboards(Side::White)[QUEEN].count_ones() as i64 * QUEEN_VALUE;
    eval -= board.side_bitboards(Side::Black)[PAWN].count_ones() as i64 * PAWN_VALUE;
    eval -= board.side_bitboards(Side::Black)[ROOK].count_ones() as i64 * ROOK_VALUE;
    eval -= board.side_bitboards(Side::Black)[KNIGHT].count_ones() as i64 * KNIGHT_VALUE;
    eval -= board.side_bitboards(Side::Black)[BISHOP].count_ones() as i64 * BISHOP_VALUE;
    eval -= board.side_bitboards(Side::Black)[QUEEN].count_ones() as i64 * QUEEN_VALUE;

    eval * MATERIAL_WEIGHT
}

pub fn eval_score(board: &SearchBoard) -> i64 {
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
    }
    positional * POSITIONAL_WEIGHT
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
pub(super) fn rate_move(mov: &Move, who_to_move: Side) -> i64 {
    let piece = mov.piece_type().with_side(who_to_move);
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
        MoveType::ShortCastle => 20000,
        MoveType::LongCastle => 20000,
        MoveType::EnPassant => 1_000_000,
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
