use crate::{piece::Side, position::Offset};
use array_init::array_init;
use std::sync::LazyLock;

use crate::position::Position;

pub struct SingularData {
    pub positions: Vec<Position>,
    pub parts: Vec<u64>,
    pub sum: u64,
}

impl SingularData {
    fn horse_at_index(index: usize) -> Self {
        let positions: Vec<_> = [
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
        .filter_map(|&off| Position::from_index(index as u8).with_offset(off))
        .collect();

        let parts: Vec<_> = positions.iter().map(|i| i.as_mask()).collect();

        let sum = parts.iter().fold(0, |acc, i| acc | i);

        Self {
            positions,
            parts,
            sum,
        }
    }

    fn king_at_index(index: usize) -> Self {
        let positions: Vec<_> = [
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
        .filter_map(|&off| Position::from_index(index as u8).with_offset(off))
        .collect();

        let parts: Vec<_> = positions.iter().map(|i| i.as_mask()).collect();

        let sum = parts.iter().fold(0, |acc, i| acc | i);

        Self {
            positions,
            parts,
            sum,
        }
    }

    fn pawn_takes(index: usize, side: Side) -> Self {
        const WHITE_PAWN_DIRECTIONS: [Offset; 2] = [Offset::new(-1, 1), Offset::new(1, 1)];
        const BLACK_PAWN_DIRECTIONS: [Offset; 2] = [Offset::new(-1, -1), Offset::new(1, -1)];
        let positions: Vec<_> = if let Side::White = side {
            WHITE_PAWN_DIRECTIONS
        } else {
            BLACK_PAWN_DIRECTIONS
        }
        .iter()
        .filter_map(|&off| Position::from_index(index as u8).with_offset(off))
        .collect();

        let parts: Vec<_> = positions.iter().map(|i| i.as_mask()).collect();

        let sum = parts.iter().fold(0, |acc, i| acc | i);

        Self {
            positions,
            parts,
            sum,
        }
    }
}

pub static KNIGHT_MASKS: LazyLock<[SingularData; 64]> =
    LazyLock::new(|| array_init(SingularData::horse_at_index));
pub static KING_MASKS: LazyLock<[SingularData; 64]> =
    LazyLock::new(|| array_init(SingularData::king_at_index));
pub static WHITE_PAWN_TAKE_MASKS: LazyLock<[SingularData; 64]> =
    LazyLock::new(|| array_init(|i| SingularData::pawn_takes(i, Side::White)));
pub static BLACK_PAWN_TAKE_MASKS: LazyLock<[SingularData; 64]> =
    LazyLock::new(|| array_init(|i| SingularData::pawn_takes(i, Side::Black)));

pub fn init_masks() {
    let _ = LazyLock::force(&KNIGHT_MASKS);
    let _ = LazyLock::force(&KING_MASKS);
    let _ = LazyLock::force(&WHITE_PAWN_TAKE_MASKS);
    let _ = LazyLock::force(&BLACK_PAWN_TAKE_MASKS);
}

pub fn choose_pawn_take_mask(side: Side) -> &'static [SingularData; 64] {
    match side {
        Side::White => &*WHITE_PAWN_TAKE_MASKS,
        Side::Black => &*BLACK_PAWN_TAKE_MASKS,
    }
}

pub const WHITE_HOME_ROOKS: [Position; 2] = [Position::new(0, 0), Position::new(7, 0)];
pub const BLACK_HOME_ROOKS: [Position; 2] = [Position::new(0, 7), Position::new(7, 7)];

pub fn choose_home_rook(side: Side) -> [Position; 2] {
    match side {
        Side::White => WHITE_HOME_ROOKS,
        Side::Black => BLACK_HOME_ROOKS,
    }
}
