use std::hash::Hash;

#[path ="./utils.rs"]
mod utils;

use crate::utils::{Piece, Move, Position, PieceType, Side};
use PieceType::*;

const ZOBRIST_HASHER: ZobristHasher = ZobristHasher::init();

struct ZobristHasher {
    piece_boards: [[u64; 64]; 12],
    black: u64,
}

impl ZobristHasher {

    // I'll need a custom pseudorandom generator to generate numbers deterministically (at compile time)
    pub const fn init() -> Self {
        todo!()
    }
    pub fn get_value(&self, piece: Piece, pos: Position) -> u64{
        match piece{
            Piece::White(piece) => match piece{
                Pawn => self.piece_boards[0][*pos as usize],
                Rook => self.piece_boards[1][*pos as usize],
                Knight => self.piece_boards[2][*pos as usize],
                Bishop => self.piece_boards[3][*pos as usize],
                Queen => self.piece_boards[4][*pos as usize],
                King => self.piece_boards[5][*pos as usize],
            }
            Piece::Black(piece) => match piece{
                Pawn => self.piece_boards[6][*pos as usize],
                Rook => self.piece_boards[7][*pos as usize],
                Knight => self.piece_boards[8][*pos as usize],
                Bishop => self.piece_boards[9][*pos as usize],
                Queen => self.piece_boards[10][*pos as usize],
                King => self.piece_boards[11][*pos as usize],
            }
        }
    }
    pub fn update_hash(&self, mut hash: u64, mov: Move, piece: Piece, move_side: Side, side_changed: bool) -> u64{
        hash ^= self.get_value(piece, mov.from);

        if let Some(promoted_to) = mov.promote_to{
            hash ^= self.get_value(promoted_to.with_side(move_side), mov.to)
        }
        else{
            hash ^= self.get_value(piece, mov.to);
        }
        
        if side_changed{
            hash ^= self.black
        }

        hash
    }
}

#[derive(Clone)]
pub struct Bitboards{
    pub pawn: u64,
    pub rook: u64,
    pub knight: u64,
    pub bishop: u64,
    pub queen: u64,
    pub king: u64
}

impl Bitboards{
    pub fn get_containing_bitboard_mut(&mut self, pos: Position) -> Option<&mut u64>{
        if self.pawn & pos.as_mask() != 0 {
            return Some(&mut self.pawn)
        }
        else if self.rook & pos.as_mask() != 0{
            return Some(&mut self.rook)
        }
        else if self.knight & pos.as_mask() != 0{
            return Some(&mut self.knight)
        }
        else if self.bishop & pos.as_mask() != 0{
            return Some(&mut self.bishop)
        }
        else if self.queen & pos.as_mask() != 0{
            return Some(&mut self.queen)
        }
        else if self.king & pos.as_mask() != 0{
            return Some(&mut self.king)
        }
        else{
            None
        }
    }

    pub fn get_bitboard_mut(&mut self, piece_type: PieceType) -> &mut u64{
        match piece_type {
            Pawn => &mut self.pawn,
            Rook => &mut self.rook,
            Knight => &mut self.knight,
            Bishop => &mut self.bishop,
            Queen => &mut self.queen,
            King => &mut self.king
        }
    }

    pub fn combined(&self) -> u64{
        self.pawn | self.rook | self.knight | self.bishop | self.queen | self.king
    }
}

#[derive(Clone)]
struct BoardState {
    pub black: Bitboards,
    pub white: Bitboards,
    pub side: Side,
    pub zobrist: u64,
}

impl BoardState{
    pub fn get_bitboard(&self, piece: Piece) -> u64{
        match piece{
            Piece::White(piece) => match piece{
                Pawn => self.white.pawn,
                Rook => self.white.rook,
                Knight => self.white.knight,
                Bishop => self.white.bishop,
                Queen => self.white.queen,
                King => self.white.king,
            }
            Piece::Black(piece) => match piece{
                Pawn => self.black.pawn,
                Rook => self.black.rook,
                Knight => self.black.knight,
                Bishop => self.black.bishop,
                Queen => self.black.queen,
                King => self.black.king,
            }
        }

    }

    pub fn side_bitboard_mut(&mut self, side: Side) -> &mut Bitboards{
        match side{
            Side::White => &mut self.white,
            Side::Black => &mut self.black
        }
    }

    pub fn make_move(&self, mov: Move, piece: Piece) -> BoardState{
        let mut after_move = self.clone();

        if let Some(this_side) = after_move.side_bitboard_mut(self.side).get_containing_bitboard_mut(mov.from) {
            *this_side ^= mov.from.as_mask();
            if let Some(promote_to) = mov.promote_to {
                // if the move includes a promotion, update the bitboard of that type
                // else, update the bitboard of the piece that makes the move
                *after_move.side_bitboard_mut(self.side).get_bitboard_mut(promote_to) |= mov.from.as_mask()
            }
            else{
                *this_side |= mov.to.as_mask();
            }
        }

        // if the other side has a piece at the target position, delete it from there as the move is a take
        if let Some(other_side) = after_move.side_bitboard_mut(self.side.opposite()).get_containing_bitboard_mut(mov.to) {
            *other_side ^= mov.to.as_mask();
        }

        after_move

    }
}

impl Hash for BoardState{
    // a hasher should only care about the zobrist hash
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zobrist.hash(state);
    }
}