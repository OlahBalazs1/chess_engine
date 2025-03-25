#[path = "./utils.rs"]
mod utils;
use crate::utils::{Move, Offset, Position};

use rayon::prelude::*;
use std::sync::Arc;

const ROOK_OFFSETS: [Offset; 4] = [
    Offset::new(1, 0),
    Offset::new(-1, 0),
    Offset::new(0, 1),
    Offset::new(0, -1),
];
const BISHOP_OFFSETS: [Offset; 4] = [
    Offset::new(1, 1),
    Offset::new(-1, 1),
    Offset::new(-1, -1),
    Offset::new(1, -1),
];

struct MagicMover {
    rook_magics: [[SquareMagic; 8]; 8],
    bishop_magics: [[SquareMagic; 8]; 8],
}

struct SquareMagic {
    moves: Box<[Arc<[Move]>]>,
    premask: u64,
    magic: u64,
    shift: u8,
}

impl SquareMagic {
    fn new(square: Position, magic: u64, shift: u8, offsets: [Offset; 4]) -> Self {
        let indices = rook_indices(square);
        let premask = indices_to_premask(indices.clone());

        let blockers = generate_blockers(indices);

        let raw_moves = blockers
            .par_iter()
            .map(|blocker| possible_moves(*blocker, square, offsets))
            .collect::<Vec<Arc<[Move]>>>();

        let magicized_blockers = blockers
            .par_iter()
            .map(|blocker_config| hash_blockers(*blocker_config, premask, magic, shift))
            .collect::<Vec<u64>>();

        let highest = magicized_blockers
            .iter()
            .fold(0, |acc, i| std::cmp::max(acc, *i));

        let moves = {
            let mut moves = vec![None; highest as usize];
            for (index, blocker) in magicized_blockers.iter().enumerate() {
                if let Some(collided_move) = moves[*blocker as usize].clone() {
                    assert!(
                        collided_move == raw_moves[index],
                        "Magic number is not magic!"
                    );
                    continue;
                }
                moves[*blocker as usize] = Some(raw_moves[index].clone())
            }

            moves
                .par_iter()
                .map(|i| i.clone().unwrap_or(vec![].into()))
                .collect::<Vec<Arc<[Move]>>>()
                .into_boxed_slice()
        };
        Self {
            moves,
            premask,
            magic,
            shift,
        }
    }

    fn new_rook(square: Position, magic: u64, shift: u8, offsets: [Offset; 4]) -> Self {
        SquareMagic::new(square, magic, shift, ROOK_OFFSETS)
    }

    fn new_bishop(square: Position, magic: u64, shift: u8, offsets: [Offset; 4]) -> Self {
        SquareMagic::new(square, magic, shift, BISHOP_OFFSETS)
    }

    fn get_moves(&self, blocker_config: u64) -> Arc<[Move]> {
        (*self.moves)[self.hash(blocker_config) as usize].clone()
    }

    fn hash(&self, blocker_config: u64) -> u64 {
        ((blocker_config & self.premask) * self.magic) >> self.shift
    }
}

const fn hash_blockers(blockers: u64, premask: u64, magic: u64, shift: u8) -> u64 {
    ((blockers & premask) * magic) >> shift
}

fn possible_moves<const N: usize>(
    blocker_config: u64,
    start_pos: Position,
    offsets: [Offset; N],
) -> Arc<[Move]> {
    let mut moves = vec![];

    let mut directions = [true; N];
    for i in 0..7 {
        let directions_clone = directions.clone();

        let offsets = offsets
            .iter()
            .enumerate()
            .filter(|(index, _)| directions_clone[*index])
            .map(|(index, offset)| (index, offset.mul(i).unwrap()));

        for (index, offset) in offsets {
            if let Some(position) = start_pos.with_offset(offset) {
                directions[index] = blocker_config & (1 << *position) == 0;
                moves.push(Move::new(start_pos, position));
            } else {
                directions[index] = false
            }
        }
    }
    moves.into()
}

fn generate_blockers(indices: Box<[u8]>) -> Box<[u64]> {
    let mut blockers = vec![];
    for combination in 0..(1 << indices.len()) {
        let bitboard = {
            let mut bitboard = 0u64;
            for (index, i) in indices.iter().enumerate() {
                bitboard |= (combination & (1 << index) >> index) << i;
            }
            bitboard
        };
        blockers.push(bitboard);
    }

    blockers.into_boxed_slice()
}

fn rook_indices(pos: Position) -> Box<[u8]> {
    let x = pos.x();
    let y = pos.y();
    let mut indices = Vec::with_capacity(14);
    for i in 1..7 {
        if i != y {
            indices.push(*Position::new(x, i));
        }
        if i != x {
            indices.push(*Position::new(i, y));
        }
    }
    indices.into_boxed_slice()
}

fn bishop_indices(pos: Position) -> Box<[u8]> {
    let x = pos.x();
    let y = pos.y();
    let mut indices = Vec::with_capacity(9);
    for i in 1..7_u8 {
        if let Some(y_offset) = i.checked_sub(x) {
            let y1 = y + y_offset;

            if y1 < 8 {
                indices.push(*Position::new(x, y1));
            }

            if let Some(y2) = y.checked_sub(y_offset) {
                if y2 < 8 {
                    indices.push(*Position::new(x, y2));
                }
            }
        }
    }
    indices.into_boxed_slice()
}

fn indices_to_premask(indices: Box<[u8]>) -> u64 {
    indices.iter().fold(0_u64, |acc, index| acc | (1 << index))
}
