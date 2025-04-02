use crate::{
    moving::{MoveNotation, MoveType},
    utils::{Offset, PieceType, Position},
};

#[derive(Clone, Copy)]
pub struct RookMove {
    from: Position,
    to: Position,
}

#[derive(Clone, Copy)]
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
    rook_magics: [[SquareMagic<RookMove>; 8]; 8],
    bishop_magics: [[SquareMagic<BishopMove>; 8]; 8],
}

struct SquareMagic<M: MoveNotation> {
    moves: Box<[Box<[M]>]>,
    premask: u64,
    magic: u64,
    shift: u8,
}

impl<M: MoveNotation> SquareMagic<M> {
    const fn rook_from_magic(pos: Position, magic: u64) -> SquareMagic<RookMove> {
        unimplemented!()
    }

    const fn bishop_from_magic(pos: Position, magic: u64) -> SquareMagic<BishopMove> {
        unimplemented!()
    }

    fn get(&self, blockers: u64) -> &[M] {
        &self.moves[self.hash(blockers) as usize][..]
    }

    fn hash(&self, mut blockers: u64) -> u64 {
        blockers &= self.premask;
        blockers *= self.magic;
        blockers >>= self.shift;
        blockers
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
