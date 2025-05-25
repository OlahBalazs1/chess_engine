use crate::{
    board::BoardState,
    moving::Move,
    piece::{Piece, PieceType, Side},
    position::Position,
};
use rand::prelude::*;
use std::cell::LazyCell;
use PieceType::*;

pub const ZOBRIST_RANDOM: LazyCell<ZobristRandom> =
    LazyCell::new(|| ZobristRandom::seeded_init(b"Lorem ipsum dolor sit amet nisi."));

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
        match piece {
            Piece {
                side: White,
                piece_type: piece,
            } => match piece {
                Pawn => self.piece_boards[0][*pos as usize],
                Rook => self.piece_boards[1][*pos as usize],
                Knight => self.piece_boards[2][*pos as usize],
                Bishop => self.piece_boards[3][*pos as usize],
                Queen => self.piece_boards[4][*pos as usize],
                King => self.piece_boards[5][*pos as usize],
            },
            Piece {
                side: Black,
                piece_type: piece,
            } => match piece {
                Pawn => self.piece_boards[6][*pos as usize],
                Rook => self.piece_boards[7][*pos as usize],
                Knight => self.piece_boards[8][*pos as usize],
                Bishop => self.piece_boards[9][*pos as usize],
                Queen => self.piece_boards[10][*pos as usize],
                King => self.piece_boards[11][*pos as usize],
            },
        }
    }
    pub fn update_hash(
        &self,
        mut hash: u64,
        mov: Move,
        piece: Piece,
        move_side: Side,
        en_passant_from_to: (Option<Position>, Option<Position>),
    ) -> u64 {
        hash ^= self.get_value(piece, mov.from());

        if let Some(promoted_to) = mov.promote_to() {
            hash ^= self.get_value(promoted_to.with_side(move_side), mov.to())
        } else {
            hash ^= self.get_value(piece, mov.to());
        }

        if let Some(en_passant) = en_passant_from_to.0 {
            hash ^= self.en_passant_squares[*en_passant as usize]
        }
        if let Some(en_passant) = en_passant_from_to.1 {
            hash ^= self.en_passant_squares[*en_passant as usize]
        }

        hash ^= self.black;

        hash
    }

    pub fn castle_update(&self, mut hash: u64, side: Side, from: Position, to: Position) -> u64 {
        hash ^= self.get_value(Rook.with_side(side), from);
        hash ^= self.get_value(Rook.with_side(side), to);
        hash
    }
    pub fn update_long_castle_right(&self, hash: u64, side: Side) -> u64 {
        hash ^ match side {
            Side::White => self.white_castle_rights[0],
            Side::Black => self.black_castle_rights[0],
        }
    }
    pub fn update_short_castle_right(&self, hash: u64, side: Side) -> u64 {
        hash ^ match side {
            Side::White => self.white_castle_rights[1],
            Side::Black => self.black_castle_rights[1],
        }
    }
}
