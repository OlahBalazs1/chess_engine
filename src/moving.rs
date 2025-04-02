use crate::utils::{PieceType, Position};

pub trait MoveNotation {
    fn new(from: Position, to: Position, move_type: MoveType) -> Self;
    fn from(&self) -> Position;
    fn to(&self) -> Position;
    fn piece_type(&self) -> PieceType;
    fn promote_to(&self) -> Option<PieceType>;

    fn is_pawn_starter(&self) -> bool {
        if self.from().y() > self.to().y() {
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
