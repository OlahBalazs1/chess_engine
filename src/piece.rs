use crate::position::Position;
use std::{
    fmt::{write, Debug, Display},
    ops::Deref,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}
impl PieceType {
    pub fn with_side(self, side: Side) -> Piece {
        Piece::new(self, side)
    }
}
use PieceType::*;
use Side::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub side: Side,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_type: PieceType, side: Side) -> Self {
        Self { side, piece_type }
    }

    pub fn white(piece_type: PieceType) -> Self {
        Self {
            side: White,
            piece_type,
        }
    }
    pub fn black(piece_type: PieceType) -> Self {
        Self {
            side: Black,
            piece_type,
        }
    }

    pub const fn role(self) -> PieceType {
        self.piece_type
    }

    pub const fn side(self) -> Side {
        self.side
    }
    pub fn as_u8(self) -> u8 {
        (self.side as u8) + (self.piece_type as u8)
    }

    pub fn from_u8(repr: u8) -> Self {
        match repr {
            0b1000 => Self::black(Pawn),
            0b1001 => Self::black(Rook),
            0b1010 => Self::black(Knight),
            0b1011 => Self::black(Bishop),
            0b1100 => Self::black(Queen),
            0b1101 => Self::black(King),
            0b0000 => Self::white(Pawn),
            0b0001 => Self::white(Rook),
            0b0010 => Self::white(Knight),
            0b0011 => Self::white(Bishop),
            0b0100 => Self::white(Queen),
            0b0101 => Self::white(King),
            _ => panic!("Invalid representation!"),
        }
    }

    pub unsafe fn from_u8_unchecked(repr: u8) -> Self {
        let side: Side = unsafe { std::mem::transmute(repr & 0b1000) };
        let piece_type = unsafe { std::mem::transmute(repr & 0b0111) };

        Self { side, piece_type }
    }

    pub fn filter_side(self, side: Side) -> Option<Self> {
        if self.side == side {
            return Some(self);
        }
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    White = 0,
    Black = 8,
}

impl Side {
    pub const fn opposite(self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }

    pub const fn home_y(self) -> u8 {
        match self {
            Side::White => 0,
            Side::Black => 7,
        }
    }

    pub const fn pers_y(self, rank: u8) -> u8 {
        match self {
            Side::White => rank,
            Side::Black => 7 - rank,
        }
    }
}

#[inline]
pub fn occupied(bitboard: u64, pos: Position) -> bool {
    bitboard & (1 << pos.index()) != 0
}

use PieceType::*;
impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece {
                side: White,
                piece_type: Pawn,
            } => write!(f, "p"),
            Piece {
                side: White,
                piece_type: Rook,
            } => write!(f, "r"),
            Piece {
                side: White,
                piece_type: Knight,
            } => write!(f, "n"),
            Piece {
                side: White,
                piece_type: Bishop,
            } => write!(f, "b"),
            Piece {
                side: White,
                piece_type: Queen,
            } => write!(f, "q"),
            Piece {
                side: White,
                piece_type: King,
            } => write!(f, "k"),
            Piece {
                side: Black,
                piece_type: Pawn,
            } => write!(f, "P"),
            Piece {
                side: Black,
                piece_type: Rook,
            } => write!(f, "R"),
            Piece {
                side: Black,
                piece_type: Knight,
            } => write!(f, "N"),
            Piece {
                side: Black,
                piece_type: Bishop,
            } => write!(f, "B"),
            Piece {
                side: Black,
                piece_type: Queen,
            } => write!(f, "Q"),
            Piece {
                side: Black,
                piece_type: King,
            } => write!(f, "K"),
        }
    }
}
