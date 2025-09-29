use std::ops::{Index, IndexMut};

use crate::{
    piece::{Piece, PieceType, Side},
    position::Position,
};
use PieceType::*;

pub const PAWN: usize = 0;
pub const ROOK: usize = 1;
pub const KNIGHT: usize = 2;
pub const BISHOP: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;
#[derive(Clone, Debug, PartialEq)]
pub struct Bitboards {
    // Pawn, rook, knight, bishop, queen, king
    pub state: [u64; 6],
}

impl Index<usize> for Bitboards {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.state[index]
    }
}

impl IndexMut<usize> for Bitboards {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.state[index]
    }
}
impl Bitboards {
    pub fn get_containing_bitboard_mut(&mut self, pos: Position) -> Option<&mut u64> {
        for i in self.state.iter_mut() {
            if *i & pos.as_mask() != 0 {
                return Some(i);
            }
        }
        None
    }

    pub fn get_role_at_position(&self, pos: Position) -> Option<PieceType> {
        let mask = pos.as_mask();

        if self.state[0] & mask != 0 {
            return Some(Pawn);
        } else if self.state[1] & mask != 0 {
            return Some(Rook);
        } else if self.state[2] & mask != 0 {
            return Some(Knight);
        } else if self.state[3] & mask != 0 {
            return Some(Bishop);
        } else if self.state[4] & mask != 0 {
            return Some(Queen);
        } else if self.state[5] & mask != 0 {
            return Some(King);
        } else {
            None
        }
    }

    pub fn get_bitboard_mut(&mut self, piece_type: PieceType) -> &mut u64 {
        &mut self.state[piece_type as usize]
    }

    pub fn get_bitboard(&self, piece_type: PieceType) -> u64 {
        self.state[piece_type as usize]
    }

    #[inline]
    pub fn combined(&self) -> u64 {
        self.state[0]
            | self.state[1]
            | self.state[2]
            | self.state[3]
            | self.state[4]
            | self.state[5]
    }

    pub fn pawn_mut(&mut self) -> &mut u64 {
        &mut self.state[0]
    }

    pub fn rook_mut(&mut self) -> &mut u64 {
        &mut self.state[1]
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BoardRepr {
    pub board: [Option<Piece>; 64],
}
impl BoardRepr {
    pub fn get(&self, index: Position) -> Option<Piece> {
        self[*index as usize]
    }

    pub fn from_bitboards(white: Bitboards, black: Bitboards) -> Self {
        let mut board = [None; 64];

        for (index, cell) in board.iter_mut().enumerate() {
            let pos = Position::from_index(index as u8);
            *cell = match (
                white.get_role_at_position(pos),
                black.get_role_at_position(pos),
            ) {
                (Some(i), None) => Some(Piece::white(i)),
                (None, Some(i)) => Some(Piece::black(i)),
                (None, None) => None,
                (Some(_), Some(_)) => panic!("Bitboards not set up correctly"),
            }
        }

        Self { board }
    }

    pub fn find_king(&self, side: Side) -> Position {
        let pieces = self.board.iter().enumerate().map(|(pos, piece)| {
            (
                Position::from_index(pos as u8),
                piece.and_then(|piece| piece.filter_side(side)),
            )
        });
        for (pos, piece) in pieces {
            if let Some(piece) = piece {
                match piece.role() {
                    King => return pos,
                    _ => {}
                }
            }
        }
        panic!("No king found: {:?}", self)
    }
}

impl Index<usize> for BoardRepr {
    type Output = Option<Piece>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}
