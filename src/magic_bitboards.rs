#[path = "./utils.rs"]
mod utils;
use crate::utils::{Move, Offset, Position};

struct MagicMover {
    rook_magics: [[SquareMagic; 8]; 8],
    bishop_magics: [[SquareMagic; 8]; 8],
}

struct SquareMagic {
    moves: Box<[Move]>,
    premask: u64,
    magic: u64,
    shift: u8,
}

impl SquareMagic {
    fn find_rook_magic(pos: Position) -> Self {
        unimplemented!()
    }

    fn rook_from_magic(pos: Position, magic: u64) -> Self {
        unimplemented!()
    }

    fn bishop_from_magic(pos: Position, magic: u64) -> Self {
        unimplemented!()
    }
}

fn slide_blocker_possible_moves<const N: usize>(
    blocker_config: u64,
    start_pos: Position,
    offsets: [Offset; N],
) -> Box<[Move]> {
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
    moves.into_boxed_slice()
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
