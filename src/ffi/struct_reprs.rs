use crate::{
    moving::{Move, MoveType},
    piece::{Piece, PieceType, Side},
    position::Position,
};
#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum SimplePieceRepr {
    Nothing = 0,

    WhitePawn = 0b1,
    WhiteRook = 0b10,
    WhiteKnight = 0b11,
    WhiteBishop = 0b100,
    WhiteQueen = 0b101,
    WhiteKing = 0b110,

    BlackPawn = 0b1001,
    BlackRook = 0b1010,
    BlackKnight = 0b1011,
    BlackBishop = 0b1100,
    BlackQueen = 0b1101,
    BlackKing = 0b1110,
}

impl From<Option<Piece>> for SimplePieceRepr {
    fn from(value: Option<Piece>) -> Self {
        use PieceType::*;
        let Some(piece) = value else {
            return Self::Nothing;
        };
        match piece {
            Piece {
                side: Side::White,
                piece_type: typ,
            } => match typ {
                Pawn => Self::WhitePawn,
                Rook => Self::WhiteRook,
                Knight => Self::WhiteKnight,
                Bishop => Self::WhiteBishop,
                Queen => Self::WhiteQueen,
                King => Self::WhiteKing,
            },
            Piece {
                side: Side::Black,
                piece_type: typ,
            } => match typ {
                Pawn => Self::BlackPawn,
                Rook => Self::BlackRook,
                Knight => Self::BlackKnight,
                Bishop => Self::BlackBishop,
                Queen => Self::BlackQueen,
                King => Self::BlackKing,
            },
        }
    }
}

impl From<SimplePieceRepr> for Option<Piece> {
    fn from(value: SimplePieceRepr) -> Self {
        use PieceType::*;
        use Side::*;
        use SimplePieceRepr::*;
        match value {
            Nothing => None,
            WhitePawn => Some(Piece {
                side: White,
                piece_type: Pawn,
            }),
            WhiteRook => Some(Piece {
                side: White,
                piece_type: Rook,
            }),
            WhiteKnight => Some(Piece {
                side: White,
                piece_type: Knight,
            }),
            WhiteBishop => Some(Piece {
                side: White,
                piece_type: Bishop,
            }),
            WhiteQueen => Some(Piece {
                side: White,
                piece_type: Queen,
            }),
            WhiteKing => Some(Piece {
                side: White,
                piece_type: King,
            }),
            BlackPawn => Some(Piece {
                side: Black,
                piece_type: Pawn,
            }),
            BlackRook => Some(Piece {
                side: Black,
                piece_type: Rook,
            }),
            BlackKnight => Some(Piece {
                side: Black,
                piece_type: Knight,
            }),
            BlackBishop => Some(Piece {
                side: Black,
                piece_type: Bishop,
            }),
            BlackQueen => Some(Piece {
                side: Black,
                piece_type: Queen,
            }),
            BlackKing => Some(Piece {
                side: Black,
                piece_type: King,
            }),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FFIMove {
    move_type: MoveType,
    from: Position,
    to: Position,
    take: SimplePieceRepr,
}

impl From<Move> for FFIMove {
    fn from(value: Move) -> Self {
        Self {
            move_type: value.move_type,
            from: value.from,
            to: value.to,
            take: value.take.into(),
        }
    }
}
impl From<FFIMove> for Move {
    fn from(value: FFIMove) -> Self {
        Self {
            move_type: value.move_type,
            from: value.from,
            to: value.to,
            take: value.take.into(),
        }
    }
}
