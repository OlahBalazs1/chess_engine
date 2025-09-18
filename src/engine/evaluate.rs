use std::ops::Mul;

use crate::{
    board::SearchBoard,
    piece::{Piece, PieceType, Side},
    position::Position,
};
use PieceType::*;
use rand::random;

const PAWN_VALUE: i64 = 10;
const KNIGHT_VALUE: i64 = 30;
const BISHOP_VALUE: i64 = 30;
const ROOK_VALUE: i64 = 50;
const QUEEN_VALUE: i64 = 90;

pub fn evaluate(board: &SearchBoard) -> i64 {
    let mut eval = 0;
    for pos in (0..64).map(Position::from_index) {
        if let Some(piece) = board.board.board[*pos as usize] {
            eval += get_material(piece)
        }
    }
    if board.side() == Side::Black {
        eval *= -1
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
        King => 0,
    }
    .mul(if Side::White == piece.side() { 1 } else { -1 })
}
