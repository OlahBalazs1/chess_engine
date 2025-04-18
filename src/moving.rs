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
mod move_search {
    use crate::{
        magic_bitboards::{BishopMove, MagicMover, RookMove, MAGIC_MOVER},
        moving::{MoveNotation, MoveType},
        piece::{PieceType, Side},
        position::{Offset, Position},
    };

    pub fn find_pawn<M, T>(
        side: Side,
        pos: Position,
        friendlies: u64,
        enemies: u64,
        must_block: u64,
    ) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        let mut moves: Vec<M> = Vec::with_capacity(4);
        let yo = match side {
            Side::White => 1,
            Side::Black => -1,
        };

        // Takes
        use PieceType::*;
        [Offset::new(-1, yo), Offset::new(1, yo)]
            .iter()
            .filter_map(|&off| pos.with_offset(off))
            .filter(|to| {
                enemies & to.as_mask() != 0
                    && friendlies & to.as_mask() == 0
                    && (must_block == 0 || must_block & to.as_mask() < must_block)
            })
            .for_each(|to| match (side, to.y()) {
                (Side::White, 7) | (Side::Black, 0) => {
                    for promote_to in [Rook, Knight, Bishop, Queen] {
                        moves.push(M::new(pos, to, MoveType::Promotion(promote_to)))
                    }
                }
                _ => moves.push(M::new(pos, to, MoveType::Normal(PieceType::Pawn))),
            });

        let forward = [Offset::new(0, yo), Offset::new(0, 2 * yo)];
        let valid_forward = match pos.y() {
            1 | 6 => &forward[..],
            _ => &forward[..1],
        }
        .iter()
        .filter_map(|&off| pos.with_offset(off));

        for to in valid_forward {
            if (friendlies & enemies) & to.as_mask() != 0 {
                break;
            }
            moves.push(M::new(pos, to, MoveType::Normal(PieceType::Pawn)))
        }

        moves.into()
    }

    pub fn find_knight<M, T>(pos: Position, friendlies: u64) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        [
            Offset::new(-2, -1),
            Offset::new(-2, 1),
            Offset::new(-1, 2),
            Offset::new(1, 2),
            Offset::new(2, 1),
            Offset::new(2, -1),
            Offset::new(1, -2),
            Offset::new(-1, -2),
        ]
        .iter()
        .filter_map(|&off| pos.with_offset(off))
        .filter(|p| friendlies & p.as_mask() != 0)
        .map(|i| M::new(pos, i, MoveType::Normal(PieceType::Knight)))
        .collect::<Vec<M>>()
        .into()
    }

    pub fn find_king<M, T>(
        pos: Position,
        friendlies: u64,
        attacked_squares: u64,
        castle_rights: (bool, bool),
    ) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        let mut moves: Vec<M> = Vec::with_capacity(10);
        let must_avoid = friendlies & attacked_squares;

        // normal moving
        moves.extend(
            [
                Offset::new(0, 1),
                Offset::new(0, -1),
                Offset::new(1, 0),
                Offset::new(-1, 0),
                Offset::new(1, 1),
                Offset::new(-1, -1),
                Offset::new(-1, 1),
                Offset::new(1, -1),
            ]
            .iter()
            .filter_map(|i| pos.with_offset(*i))
            .filter(|i| must_avoid & i.as_mask() == 0)
            .map(|i| M::new(pos, i, MoveType::Normal(PieceType::King))),
        );
        let long = 0b11 << (1 + pos.y() * 8);
        let short = 0b11 << (5 + pos.y() * 8);

        if castle_rights.0 && long & must_avoid != 0 {
            moves.push(M::new(
                pos,
                pos.with_x(2),
                MoveType::Normal(PieceType::King),
            ));
        }

        if castle_rights.1 && short & must_avoid != 0 {
            moves.push(M::new(
                pos,
                pos.with_x(6),
                MoveType::Normal(PieceType::King),
            ));
        }

        moves.into()
    }

    pub fn find_rook(pos: Position, friendlies: u64, all_pieces: u64) -> Vec<RookMove> {
        find_rook_with_magic(pos, friendlies, all_pieces, &*MAGIC_MOVER)
    }

    pub fn find_rook_with_magic(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
        magic_mover: &MagicMover,
    ) -> Vec<RookMove> {
        magic_mover
            .get_rook(pos, all_pieces)
            .iter()
            .cloned()
            .filter(|i| friendlies & (1 << *i.to()) == 0)
            .collect()
    }

    pub fn find_bishop(pos: Position, friendlies: u64, all_pieces: u64) -> Vec<BishopMove> {
        find_bishop_with_magic(pos, friendlies, all_pieces, &*MAGIC_MOVER)
    }

    pub fn find_bishop_with_magic(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
        magic_mover: &MagicMover,
    ) -> Vec<BishopMove> {
        magic_mover
            .get_bishop(pos, all_pieces)
            .iter()
            .cloned()
            .filter(|i| friendlies & (1 << *i.to()) == 0)
            .collect()
    }

    pub fn find_queen<M, T>(pos: Position, friendlies: u64, all_pieces: u64) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        find_queen_with_magic(pos, friendlies, all_pieces, &*MAGIC_MOVER)
    }

    pub fn find_queen_with_magic<M, T>(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
        magic_mover: &MagicMover,
    ) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        let mut bishop_moves: Vec<M> =
            find_bishop_with_magic(pos, friendlies, all_pieces, magic_mover)
                .iter()
                .map(|&i| M::new(i.from(), i.to(), MoveType::Normal(PieceType::Bishop)))
                .collect();
        let mut rook_moves: Vec<M> = find_rook_with_magic(pos, friendlies, all_pieces, magic_mover)
            .iter()
            .map(|&i| M::new(i.from(), i.to(), MoveType::Normal(PieceType::Rook)))
            .collect();

        bishop_moves.append(&mut rook_moves);

        bishop_moves.into()
    }
}
