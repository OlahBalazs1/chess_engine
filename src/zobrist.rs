use crate::{
    board::BoardState,
    moving::Move,
    piece::{Piece, PieceType, Side},
    position::Position,
};
use rand::prelude::*;
use std::sync::LazyLock;
use PieceType::*;

pub static ZOBRIST_RANDOM: LazyLock<ZobristRandom> =
    LazyLock::new(|| ZobristRandom::seeded_init(b"Lorem ipsum dolor sit amet nisi."));

pub struct ZobristRandom {
    piece_boards: [[u64; 64]; 12],
    en_passant_squares: [u64; 64],
    black_castle_rights: [u64; 2],
    white_castle_rights: [u64; 2],
    black: u64,
}

impl ZobristRandom {
    // I'll need a custom pseudorandom generator to generate numbers deterministically at compile time
    pub const fn const_init() -> Self {
        todo!()
    }

    pub fn seeded_init(seed: &[u8; 32]) -> Self {
        let mut rng = SmallRng::from_seed(*seed);
        Self {
            piece_boards: rng.random(),
            en_passant_squares: rng.random(),
            black_castle_rights: rng.random(),
            white_castle_rights: rng.random(),
            black: rng.random(),
        }
    }

    pub fn hash_board(&self, state: &mut BoardState) {
        // order of hashing bitboards
        // wPawn, wRook, wKnight, wBishop, wQueen, wKing
        // bPawn, bRook, bKnight, bBishop, bQueen, bKing
        let pieces = state.white.state.iter().chain(state.black.state.iter());
        let mut hash = 0;
        for (random, board) in std::iter::zip(self.piece_boards, pieces) {
            for square in 0..64 {
                if board & (1 << square) != 0 {
                    hash ^= random[square]
                }
            }
        }
        if let Some(en_passant) = state.en_passant_square {
            hash ^= self.en_passant_squares[*en_passant as usize]
        }
        if state.white_castling.0 {
            hash ^= self.white_castle_rights[0]
        }
        if state.white_castling.1 {
            hash ^= self.white_castle_rights[1]
        }
        if state.black_castling.0 {
            hash ^= self.black_castle_rights[1]
        }
        if state.black_castling.1 {
            hash ^= self.black_castle_rights[1]
        }

        if state.side == Side::Black {
            hash ^= self.black
        }

        state.zobrist = hash
    }

    pub fn get_value(&self, piece: Piece, pos: Position) -> u64 {
        use Side::*;
        let index = (piece.role() as u8)
            + match piece.side() {
                White => 0,
                Black => 6,
            };
        self.piece_boards[index as usize][*pos as usize]
    }
    pub fn get_castle_right<const N: usize>(&self, side: Side) -> u64 {
        match side {
            Side::White => self.white_castle_rights[N],
            Side::Black => self.black_castle_rights[N],
        }
    }
}

pub trait ZobristHash {
    fn update(&mut self, piece: Piece, pos: Position);
    fn update_short_castle(&mut self, side: Side);
    fn update_long_castle(&mut self, side: Side);
    fn switch_side(&mut self);
    fn update_ep_square(&mut self, side: Side, before: Option<Position>, after: Option<Position>);
}

impl ZobristHash for u64 {
    fn update(&mut self, piece: Piece, pos: Position) {
        *self ^= ZOBRIST_RANDOM.get_value(piece, pos);
    }
    fn update_short_castle(&mut self, side: Side) {
        *self ^= ZOBRIST_RANDOM.get_castle_right::<1>(side)
    }
    fn update_long_castle(&mut self, side: Side) {
        *self ^= ZOBRIST_RANDOM.get_castle_right::<0>(side)
    }
    fn switch_side(&mut self) {
        *self ^= ZOBRIST_RANDOM.black;
    }
    fn update_ep_square(&mut self, side: Side, before: Option<Position>, after: Option<Position>) {
        if let Some(before) = before {
            self.update(
                Piece {
                    side: side,
                    piece_type: PieceType::Pawn,
                },
                before,
            );
        }
        if let Some(after) = after {
            self.update(
                Piece {
                    side: side,
                    piece_type: PieceType::Pawn,
                },
                after,
            );
        }
    }
}
