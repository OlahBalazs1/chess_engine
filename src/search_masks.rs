use crate::{position::Offset, utils::StaticData};
use array_init::array_init;
use std::{cell::LazyCell, sync::LazyLock};

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

    fn pawn_takes(index: usize) -> Self {
        let positions: Vec<_> = [Offset::new(1, 0), Offset::new(-1, 0)]
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
pub static PAWN_TAKE_MASKS: LazyLock<[SingularData; 64]> =
    LazyLock::new(|| array_init(SingularData::pawn_takes));
