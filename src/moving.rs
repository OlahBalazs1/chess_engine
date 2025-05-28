use crate::piece::PieceType;
use crate::position::Position;
use crate::search_data::CheckPath;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal(PieceType),
    Promotion(PieceType),
    Castle { rook_pos_after: Position },
}
#[derive(Clone)]
pub struct Unmove {
    mov: Move,
    en_passant_square: Option<Position>,
    white_castling: (bool, bool),
    black_castling: (bool, bool),
    zobrist: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub move_type: MoveType,
    pub from: Position,
    pub to: Position,
    pub take: Option<PieceType>,
}

pub enum Castle {
    Short { from: Position, to: Position },
    Long { from: Position, to: Position },
    Not,
}

impl Move {
    pub fn new(from: Position, to: Position, move_type: MoveType, take: Option<PieceType>) -> Self {
        Self {
            from,
            to,
            move_type,
            take,
        }
    }

    pub fn new_with_checkpath_singular(
        from: Position,
        to: Position,
        move_type: MoveType,
        take: Option<PieceType>,
        checkpath: &mut CheckPath,
    ) -> Self {
        match take {
            Some(PieceType::King) => checkpath.set_singular(from),
            _ => {}
        };
        Self::new(from, to, move_type, take)
    }
    pub fn new_with_checkpath_diagonal(
        from: Position,
        to: Position,
        move_type: MoveType,
        take: Option<PieceType>,
        checkpath: &mut CheckPath,
    ) -> Self {
        match take {
            Some(PieceType::King) => checkpath.set_diagonal(from, to),
            _ => {}
        };
        Self::new(from, to, move_type, take)
    }
    pub fn new_with_checkpath_parallel(
        from: Position,
        to: Position,
        move_type: MoveType,
        take: Option<PieceType>,
        checkpath: &mut CheckPath,
    ) -> Self {
        match take {
            Some(PieceType::King) => checkpath.set_parallel(from, to),
            _ => {}
        };
        Self::new(from, to, move_type, take)
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
            MoveType::Castle { .. } => PieceType::King,
        }
    }

    pub fn promote_to(&self) -> Option<PieceType> {
        match self.move_type {
            MoveType::Normal(_) => None,
            MoveType::Promotion(promote) => Some(promote),
            MoveType::Castle { .. } => None,
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
impl Debug for Move {
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
            MoveType::Castle {
                rook_pos_after: pos,
            } => match pos.snap_to_side().x() {
                0 => {
                    write!(f, "O-O-O")?;
                    return Ok(());
                }
                7 => {
                    write!(f, "O-O")?;
                    return Ok(());
                }
                _ => {
                    unreachable!()
                }
            },
        };
        let take = match self.take {
            Some(i) => match i {
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
