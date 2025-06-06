use crate::piece::{Piece, PieceType};
use crate::position::Position;
use crate::search_data::{CheckPath, PinState};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum MoveType {
    Normal(PieceType),
    Promotion(PieceType),
    LongCastle,
    ShortCastle,
    EnPassant,
}
pub struct Unmove<'a> {
    pub mov: &'a Move,
    pub en_passant_square: Option<Position>,
    pub white_castling: (bool, bool),
    pub black_castling: (bool, bool),
    pub zobrist: u64,
    pub attacked_squares: u64,
    pub halfmove_clock: u8,
    pub pin_state: PinState,
    pub check_path: CheckPath,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Move {
    pub move_type: MoveType,
    pub from: Position,
    pub to: Position,
    pub take: Option<Piece>,
}

pub enum Castle {
    Short { from: Position, to: Position },
    Long { from: Position, to: Position },
    Not,
}

impl Move {
    pub fn new(from: Position, to: Position, move_type: MoveType, take: Option<Piece>) -> Self {
        Self {
            from,
            to,
            move_type,
            take,
        }
    }

    pub fn to(&self) -> Position {
        self.to
    }
    pub fn from(&self) -> Position {
        self.from
    }
    pub fn piece_type(&self) -> PieceType {
        match self.move_type {
            MoveType::Normal(role) => role,
            MoveType::Promotion(_) => PieceType::Pawn,
            MoveType::LongCastle => PieceType::King,
            MoveType::ShortCastle => PieceType::King,
            MoveType::EnPassant => PieceType::Pawn,
        }
    }

    pub fn promote_to(&self) -> Option<PieceType> {
        match self.move_type {
            MoveType::Promotion(promote) => Some(promote),
            _ => None,
        }
    }
    pub fn is_pawn_starter(&self) -> bool {
        if self.piece_type() == PieceType::Pawn && self.from().y() > self.to().y() {
            return self.from().y() - self.to().y() == 2;
        }
        self.to().y() - self.from().y() == 2
    }

    pub fn en_passant_square(&self) -> Position {
        Position::from_index((*self.from() + *self.to()) / 2)
    }
}
impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (piece_prefix, promote_suffix) = match self.move_type {
            MoveType::Normal(with) => match with {
                PieceType::Pawn => ("", ""),
                PieceType::Rook => ("R", ""),
                PieceType::Bishop => ("B", ""),
                PieceType::Knight => ("N", ""),
                PieceType::Queen => ("Q", ""),
                PieceType::King => ("K", ""),
            },
            MoveType::Promotion(to) => match to {
                PieceType::Pawn => ("", "=P"),
                PieceType::Rook => ("", "=R"),
                PieceType::Bishop => ("", "=B"),
                PieceType::Knight => ("", "=N"),
                PieceType::Queen => ("", "=Q"),
                PieceType::King => ("", "=K"),
            },
            MoveType::ShortCastle => {
                write!(f, "O-O")?;
                return Ok(());
            }
            MoveType::LongCastle => {
                write!(f, "O-O")?;
                return Ok(());
            }
            MoveType::EnPassant => ("", ""),
        };
        let take = match self.take {
            Some(i) => match i.role() {
                PieceType::Pawn => "xP",
                PieceType::Rook => "xR",
                PieceType::Bishop => "xB",
                PieceType::Knight => "xN",
                PieceType::Queen => "xQ",
                PieceType::King => "xK",
            },
            None => "",
        };
        let from =
            String::from_utf8([self.from.x() + b'a', self.from.y() + b'1'].to_vec()).unwrap();
        let to = String::from_utf8([self.to.x() + b'a', self.to.y() + b'1'].to_vec()).unwrap();
        write!(
            f,
            "{}{}{}{}{}",
            piece_prefix, from, take, to, promote_suffix
        )
    }
}
