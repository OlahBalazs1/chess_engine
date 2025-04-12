use crate::{
    moving::{MoveNotation, MoveType},
    piece::PieceType,
    position::{Offset, Position},
};
use std::iter;

#[derive(Clone, Copy, PartialEq)]
pub struct RookMove {
    from: Position,
    to: Position,
}

#[derive(Clone, Copy, PartialEq)]
pub struct BishopMove {
    from: Position,
    to: Position,
}

impl MoveNotation for RookMove {
    fn new(from: Position, to: Position, _move_type: crate::moving::MoveType) -> Self {
        Self { from, to }
    }
    fn from(&self) -> Position {
        self.from
    }
    fn to(&self) -> Position {
        self.to
    }
    fn piece_type(&self) -> PieceType {
        PieceType::Rook
    }
    fn promote_to(&self) -> Option<PieceType> {
        None
    }
}

impl MoveNotation for BishopMove {
    fn new(from: Position, to: Position, _move_type: crate::moving::MoveType) -> Self {
        Self { from, to }
    }
    fn from(&self) -> Position {
        self.from
    }
    fn to(&self) -> Position {
        self.to
    }
    fn piece_type(&self) -> PieceType {
        PieceType::Bishop
    }
    fn promote_to(&self) -> Option<PieceType> {
        None
    }
}

struct MagicMover {
    rook_magics: [SquareMagic<RookMove>; 64],
    bishop_magics: [SquareMagic<BishopMove>; 64],
}

impl MagicMover {
    fn get_rook(&self, pos: Position, blockers: u64) -> &[RookMove] {
        self.rook_magics[*pos as usize].get(blockers)
    }

    fn get_bishop(&self, pos: Position, blockers: u64) -> &[BishopMove] {
        self.bishop_magics[*pos as usize].get(blockers)
    }
}

struct MagicHasher {
    premask: u64,
    magic: u64,
    shift: u8,
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

struct SquareMagic<M: MoveNotation> {
    moves: Box<[Box<[M]>]>,
    hasher: MagicHasher,
}

impl<M: MoveNotation> SquareMagic<M> {
    fn new_rook(
        pos: Position,
        piece: PieceType,
        magic: u64,
        premask: u64,
        shift: u8,
    ) -> SquareMagic<RookMove> {
        let hasher = MagicHasher::new(premask, magic, shift);
        let blocker_configs = generate_rook_blockers(pos);
        let possible_moves: Vec<Box<[RookMove]>> = blocker_configs
            .iter()
            .map(|block| {
                slide_blocker_possible_moves(
                    *block,
                    pos,
                    piece,
                    [
                        Offset::new(1, 0),
                        Offset::new(-1, 0),
                        Offset::new(0, -1),
                        Offset::new(0, 1),
                    ],
                )
            })
            .collect();
        let mut magic_moves = vec![None; blocker_configs.len()];
        for (blocker, possible_move) in iter::zip(blocker_configs, possible_moves) {
            if let Some(collided) = &magic_moves[hasher.hash(blocker) as usize] {
                assert!(*collided == possible_move, "Magic number is not magic")
            } else {
                magic_moves[hasher.hash(blocker) as usize] = Some(possible_move)
            }
        }

        SquareMagic::<RookMove> {
            moves: magic_moves
                .iter_mut()
                .map(|i| i.take().unwrap_or_else(|| Box::new([])))
                .collect(),
            hasher,
        }
    }
    fn new_bishop(
        pos: Position,
        piece: PieceType,
        magic: u64,
        premask: u64,
        shift: u8,
    ) -> SquareMagic<BishopMove> {
        let hasher = MagicHasher::new(premask, magic, shift);
        let blocker_configs = generate_rook_blockers(pos);
        let possible_moves: Vec<Box<[BishopMove]>> = blocker_configs
            .iter()
            .map(|block| {
                slide_blocker_possible_moves(
                    *block,
                    pos,
                    piece,
                    [
                        Offset::new(1, 1),
                        Offset::new(1, -1),
                        Offset::new(-1, 1),
                        Offset::new(-1, -1),
                    ],
                )
            })
            .collect();
        let mut magic_moves = vec![None; blocker_configs.len()];
        for (blocker, possible_move) in iter::zip(blocker_configs, possible_moves) {
            if let Some(collided) = &magic_moves[hasher.hash(blocker) as usize] {
                assert!(*collided == possible_move, "Magic number is not magic")
            } else {
                magic_moves[hasher.hash(blocker) as usize] = Some(possible_move)
            }
        }

        SquareMagic::<BishopMove> {
            moves: magic_moves
                .iter_mut()
                // CORRECT BEHAVIOUR: all possible blocker configurations point to a valid move array
                .map(|i| i.take().unwrap_or_else(|| Box::new([])))
                .collect(),
            hasher,
        }
    }

    const fn hash(&self, blockers: u64) -> u64 {
        self.hasher.hash(blockers)
    }

    fn get(&self, blockers: u64) -> &[M] {
        self.moves[self.hash(blockers) as usize].as_ref()
    }
}

fn slide_blocker_possible_moves<const N: usize, M>(
    blocker_config: u64,
    start_pos: Position,
    piece: PieceType,
    offsets: [Offset; N],
) -> Box<[M]>
where
    M: MoveNotation,
{
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

                moves.push(M::new(start_pos, position, MoveType::Normal(piece)));
            } else {
                directions[index] = false
            }
        }
    }
    moves.into_boxed_slice()
}

fn generate_rook_blockers(pos: Position) -> Box<[u64]> {
    let indices = rook_indices(pos);
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

fn rook_indices(pos: Position) -> Vec<u8> {
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
    indices
}

fn generate_bishop_blockers(pos: Position) -> Box<[u64]> {
    let indices = bishop_indices(pos);
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
        if y2 > 0 && y2 < 7 {
            indices.push((y2 * 8 + i) as u8)
        }
    }

    indices
}
