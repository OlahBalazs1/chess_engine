use std::ops::Mul;

use crate::{
    board::SearchBoard,
    engine::who2move,
    piece::{Piece, PieceType, Side},
    position::Position,
};
use PieceType::*;
use rand::random;

const PAWN_VALUE: i64 = 1;
const KNIGHT_VALUE: i64 = 3;
const BISHOP_VALUE: i64 = 3;
const ROOK_VALUE: i64 = 5;
const QUEEN_VALUE: i64 = 9;
const KING_VALUE: i64 = 200;
pub fn evaluate(board: &SearchBoard) -> i64 {
    eval_score(board) * who2move(board.side()) as i64
}
pub fn eval_score(board: &SearchBoard) -> i64 {
    let mut eval = 0;
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths.clone());

    eval += (moves.len()) as i64;

    if check_paths.is_check() {
        eval += 10
    }
    for pos in (0..64).map(Position::from_index) {
        if let Some(piece) = board.board.board[*pos as usize] {
            eval += get_material(piece);
        }
    }
    eval
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
