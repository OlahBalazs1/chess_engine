use crate::{
    moving::MoveNotation,
    piece::{Piece, PieceType, Side},
    position::Position,
};
use std::cell::LazyCell;
use PieceType::*;

pub const ZOBRIST_HASHER: LazyCell<ZobristHasher> = LazyCell::new(ZobristHasher::init);

struct ZobristHasher {
    piece_boards: [[u64; 64]; 12],
    en_passant_squares: [u64; 8],
    black: u64,
}

impl ZobristHasher {
    // I'll need a custom pseudorandom generator to generate numbers deterministically (at compile time)
    pub const fn init() -> Self {
        todo!()
    }

    pub fn get_value(&self, piece: Piece, pos: Position) -> u64 {
        match piece {
            Piece::White(piece) => match piece {
                Pawn => self.piece_boards[0][*pos as usize],
                Rook => self.piece_boards[1][*pos as usize],
                Knight => self.piece_boards[2][*pos as usize],
                Bishop => self.piece_boards[3][*pos as usize],
                Queen => self.piece_boards[4][*pos as usize],
                King => self.piece_boards[5][*pos as usize],
            },
            Piece::Black(piece) => match piece {
                Pawn => self.piece_boards[6][*pos as usize],
                Rook => self.piece_boards[7][*pos as usize],
                Knight => self.piece_boards[8][*pos as usize],
                Bishop => self.piece_boards[9][*pos as usize],
                Queen => self.piece_boards[10][*pos as usize],
                King => self.piece_boards[11][*pos as usize],
            },
        }
    }
    pub fn update_hash<M: MoveNotation>(
        &self,
        mut hash: u64,
        mov: M,
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
}

mod pseudorandom {}
