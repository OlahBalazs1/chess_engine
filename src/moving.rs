use crate::piece::PieceType;
use crate::position::Position;

pub trait MoveNotation {
    fn new(from: Position, to: Position, move_type: MoveType) -> Self;
    fn from(&self) -> Position;
    fn to(&self) -> Position;
    fn piece_type(&self) -> PieceType;
    fn promote_to(&self) -> Option<PieceType>;

    fn is_pawn_starter(&self) -> bool {
        if self.piece_type() == PieceType::Pawn && self.from().y() > self.to().y() {
            return self.from().y() - self.to().y() == 2;
        }
        self.to().y() - self.from().y() == 2
    }

    fn en_passant_square(&self) -> Position {
        Position::from_index((*self.from() + *self.to()) / 2)
    }

    fn rook_square_after_castle(&self) -> Position {
        Position::from_index((*self.from() + *self.to()) / 2)
    }

    fn castle(&self) -> Castle {
        if self.is_castle() {
            let castle_from = self.castle_from();
            match castle_from.x() {
                0 => Castle::Long {
                    from: castle_from,
                    to: self.rook_square_after_castle(),
                },
                7 => Castle::Short {
                    from: castle_from,
                    to: self.rook_square_after_castle(),
                },
                _ => panic!(
                    "MoveNotation::castle_from() returned a position with an x value other than 0 or 7"
                ),
            }
        } else {
            Castle::Not
        }
    }

    fn castle_from(&self) -> Position {
        if self.to().x() < 3 {
            self.from().with_x(0)
        } else {
            self.from().with_x(7)
        }
    }

    fn is_castle(&self) -> bool {
        self.piece_type() == PieceType::King
            && ((self.from().x() as i8) - (self.to().x() as i8)).abs() == 2
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Normal(PieceType),
    Promotion(PieceType),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub move_type: MoveType,
    pub from: Position,
    pub to: Position,
}

pub enum Castle {
    Short { from: Position, to: Position },
    Long { from: Position, to: Position },
    Not,
}

impl Move {
    pub fn new_normal(from: Position, to: Position, with: PieceType) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Normal(with),
        }
    }
    pub fn new_promotion(from: Position, to: Position, promote_to: PieceType) -> Self {
        Self {
            from,
            to,
            move_type: MoveType::Promotion(promote_to),
        }
    }
}

impl MoveNotation for Move {
    fn new(from: Position, to: Position, move_type: MoveType) -> Self {
        Self {
            from,
            to,
            move_type,
        }
    }
    fn to(&self) -> Position {
        self.to
    }
    fn from(&self) -> Position {
        self.from
    }
    fn piece_type(&self) -> PieceType {
        match self.move_type {
            MoveType::Normal(role) => role,
            MoveType::Promotion(_) => PieceType::Pawn,
        }
    }

    fn promote_to(&self) -> Option<PieceType> {
        match self.move_type {
            MoveType::Normal(_) => None,
            MoveType::Promotion(promote) => Some(promote),
        }
    }
}
