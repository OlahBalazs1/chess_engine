use crate::position::Position;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
impl PieceType {
    pub fn with_side(self, side: Side) -> Piece {
        Piece::new(self, side)
    }
}
use PieceType::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}
use Piece::*;

impl Piece {
    pub fn new(piece_type: PieceType, side: Side) -> Self {
        if side == Side::White {
            White(piece_type)
        } else {
            Black(piece_type)
        }
    }

    pub const fn role(self) -> PieceType {
        match self {
            White(role) => role,
            Black(role) => role,
        }
    }

    pub const fn side(self) -> Side {
        match self {
            White(_) => Side::White,
            Black(_) => Side::Black,
        }
    }

    pub fn filter_side(self, side: Side) -> Option<Self> {
        match (self, side) {
            (White(_), Side::White) => Some(self),
            (Black(_), Side::Black) => Some(self),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub const fn opposite(self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[inline]
pub fn occupied(bitboard: u64, pos: Position) -> bool {
    bitboard & (1 << pos.index()) != 0
}
