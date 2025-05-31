use crate::hashers::*;
use crate::{
    moving::{Move, MoveType},
    piece::PieceType,
    position::{Offset, Position},
};
use std::fmt::Debug;
use std::sync::Arc;
use std::{iter, sync::LazyLock};

struct MagicDataBuilder {
    normal: Vec<Position>,
    takes: Vec<Position>,
    ends: Vec<Position>,
    bitboard: u64,
}
impl MagicDataBuilder {
    fn new() -> Self {
        Self {
            normal: Vec::with_capacity(16),
            takes: Vec::with_capacity(4),
            ends: Vec::with_capacity(4),
            bitboard: 0,
        }
    }
    fn add_normal(&mut self, data: Position) {
        self.normal.push(data);
        self.bitboard |= data.as_mask()
    }

    fn add_take(&mut self, data: Position) {
        self.takes.push(data);
        self.bitboard |= data.as_mask()
    }

    fn add_end(&mut self, data: Position) {
        self.ends.push(data);
        self.bitboard |= data.as_mask()
    }

    fn finalize(self) -> MagicData {
        MagicData {
            normal: self.normal.into_boxed_slice(),
            takes: self.takes.into_boxed_slice(),
            ends: self.ends.into_boxed_slice(),
            bitboard: self.bitboard,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MagicData {
    pub normal: Box<[Position]>,
    pub takes: Box<[Position]>,
    pub ends: Box<[Position]>,
    pub bitboard: u64,
}
use std::iter::{Chain, Copied};
use std::slice::Iter;
impl MagicData {
    pub fn possible_takes(&self) -> Chain<Copied<Iter<'_, Position>>, Copied<Iter<'_, Position>>> {
        self.takes.iter().copied().chain(self.ends.iter().copied())
    }
}

impl PartialEq for MagicData {
    fn eq(&self, other: &Self) -> bool {
        return self.bitboard == other.bitboard;
    }
}

pub static MAGIC_MOVER: LazyLock<MagicMover> =
    LazyLock::new(|| MagicMover::init(ROOK_MAGIC_HASHERS, BISHOP_MAGIC_HASHERS));

pub struct MagicMover {
    rook_magics: Box<[SquareMagic]>,
    bishop_magics: Box<[SquareMagic]>,
}

impl MagicMover {
    fn init(rook_magics: [MagicHasher; 64], bishop_magics: [MagicHasher; 64]) -> Self {
        Self {
            rook_magics: rook_magics
                .iter()
                .enumerate()
                .map(|(index, &magic)| {
                    SquareMagic::new_rook(Position::from_index(index as u8), magic)
                })
                .collect(),

            bishop_magics: bishop_magics
                .iter()
                .enumerate()
                .map(|(index, &magic)| {
                    SquareMagic::new_bishop(Position::from_index(index as u8), magic)
                })
                .collect(),
        }
    }

    pub fn get_rook(&self, pos: Position, blockers: u64) -> &MagicData {
        self.rook_magics[*pos as usize].get(blockers)
    }

    pub fn get_bishop(&self, pos: Position, blockers: u64) -> &MagicData {
        self.bishop_magics[*pos as usize].get(blockers)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MagicHasher {
    pub premask: u64,
    pub magic: u64,
    pub shift: u8,
}

impl MagicHasher {
    const fn new(premask: u64, magic: u64, shift: u8) -> Self {
        Self {
            premask,
            magic,
            shift,
        }
    }
    const fn hash(&self, mut blockers: u64) -> u64 {
        blockers &= self.premask;
        blockers = blockers.wrapping_mul(self.magic);
        blockers >>= self.shift;
        blockers
    }
}

#[derive(Debug)]
struct SquareMagic {
    moves: Box<[Option<MagicData>]>,
    hasher: MagicHasher,
}

impl SquareMagic {
    fn new_rook(pos: Position, hasher: MagicHasher) -> Self {
        let blocker_configs = generate_rook_blockers(pos);
        let all_hashed: Vec<_> = blocker_configs
            .iter()
            .map(|i| hasher.hash(*i) as usize)
            .collect();

        let highest = *all_hashed.iter().max().unwrap();
        let possible_moves: Vec<_> = blocker_configs
            .iter()
            .map(|block| {
                slide_blocker_possible_moves(
                    *block,
                    pos,
                    [
                        Offset::new(1, 0),
                        Offset::new(-1, 0),
                        Offset::new(0, -1),
                        Offset::new(0, 1),
                    ],
                )
            })
            .collect();
        let mut magic_moves = vec![None; highest + 1];
        for (blocker, possible_move) in iter::zip(blocker_configs, possible_moves) {
            if let Some(collided) = &magic_moves[hasher.hash(blocker) as usize] {
                assert!(*collided == possible_move, "Magic number is not magic")
            } else {
                magic_moves[hasher.hash(blocker) as usize] = Some(possible_move)
            }
        }

        SquareMagic {
            moves: magic_moves.into_boxed_slice(),
            hasher,
        }
    }
    fn new_bishop(pos: Position, hasher: MagicHasher) -> Self {
        let blocker_configs = generate_bishop_blockers(pos);
        let all_hashed: Vec<_> = blocker_configs
            .iter()
            .map(|i| hasher.hash(*i) as usize)
            .collect();

        let highest = *all_hashed.iter().max().unwrap();

        let possible_moves: Vec<_> = blocker_configs
            .iter()
            .map(|block| {
                slide_blocker_possible_moves(
                    *block,
                    pos,
                    [
                        Offset::new(1, 1),
                        Offset::new(1, -1),
                        Offset::new(-1, 1),
                        Offset::new(-1, -1),
                    ],
                )
            })
            .collect();

        let mut magic_moves = vec![None; highest + 1];
        for (blocker, possible_move) in iter::zip(blocker_configs, possible_moves) {
            if let Some(collided) = &magic_moves[hasher.hash(blocker) as usize] {
                assert!(*collided == possible_move, "Magic number is not magic")
            } else {
                magic_moves[hasher.hash(blocker) as usize] = Some(possible_move)
            }
        }

        SquareMagic {
            moves: magic_moves.into(),
            hasher,
        }
    }

    const fn hash(&self, blockers: u64) -> u64 {
        self.hasher.hash(blockers)
    }
    fn get(&self, blockers: u64) -> &MagicData {
        self.moves[self.hash(blockers) as usize]
            .as_ref()
            .expect("Magic number should be validated.")
    }
}

fn slide_blocker_possible_moves<const N: usize>(
    blocker_config: u64,
    start_pos: Position,
    offsets: [Offset; N],
) -> MagicData {
    let mut moves = MagicDataBuilder::new();

    let mut directions = [true; N];
    for i in 1..6 {
        let directions_clone = directions.clone();

        let offsets = offsets
            .iter()
            .enumerate()
            .filter(|(index, _)| directions_clone[*index])
            .map(|(index, offset)| (index, offset.mul(i).unwrap()));

        for (index, offset) in offsets {
            if let Some(position) = start_pos.with_offset(offset) {
                if blocker_config & (1 << *position) != 0 {
                    directions[index] = false;
                    moves.add_take(position);
                } else {
                    moves.add_normal(position);
                }
            } else {
                directions[index] = false
            }
        }
    }

    // ends
    let offsets = offsets
        .iter()
        .enumerate()
        .filter(|(index, _)| directions[*index])
        .map(|(_, offset)| offset.mul(7).unwrap());

    for offset in offsets {
        if let Some(position) = start_pos.with_offset(offset) {
            moves.add_end(position);
        }
    }

    moves.finalize()
}

fn generate_rook_blockers(pos: Position) -> Box<[u64]> {
    let indices = rook_indices(pos);
    let mut blockers = vec![];
    for combination in 0..(1 << indices.len()) {
        let bitboard = {
            let mut bitboard = 0u64;
            for (index, i) in indices.iter().enumerate() {
                if combination & (1 << index) != 0 {
                    bitboard |= 1 << i
                }
            }
            bitboard
        };
        blockers.push(bitboard);
    }

    blockers.into_boxed_slice()
}

fn rook_indices(pos: Position) -> Vec<u8> {
    let x = pos.x();
    let y = pos.y();
    let mut indices = Vec::with_capacity(14);
    for i in 1..7 {
        if i != y {
            indices.push(pos.with_y(i).unwrap().index());
        }
        if i != x {
            indices.push(pos.with_x(i).unwrap().index());
        }
    }
    indices
}

fn generate_bishop_blockers(pos: Position) -> Box<[u64]> {
    let indices = bishop_indices(pos);
    let mut blockers = vec![];
    for combination in 0..(1 << indices.len()) {
        let bitboard = {
            let mut bitboard = 0u64;
            for (index, i) in indices.iter().enumerate() {
                if combination & (1 << index) != 0 {
                    bitboard |= 1 << i
                }
            }
            bitboard
        };
        blockers.push(bitboard);
    }

    blockers.into_boxed_slice()
}

fn bishop_indices(pos: Position) -> Vec<u8> {
    let mut indices = Vec::with_capacity(10);

    let x = pos.x();
    let y = pos.y();

    for i in 1..7 {
        let yo = (x as i8) - i;

        let y1 = (y as i8) - yo;
        let y2 = (y as i8) + yo;

        if y1 > 0 && y1 < 7 && y1 != y2 {
            indices.push((y1 * 8 + i) as u8)
        }
        if y2 > 0 && y2 < 7 && y1 != y2 {
            indices.push((y2 * 8 + i) as u8)
        }
    }

    indices
}

pub fn print_bits(i: u64) {
    for y in (0..8).rev() {
        for x in 0..8 {
            if i & (1 << (x + y * 8)) != 0 {
                print!("1")
            } else {
                print!(".")
            }
        }
        println!("")
    }
    println!("")
}
