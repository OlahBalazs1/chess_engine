use std::hash::Hash;

use crate::moving::{Castle, Move};
use crate::piece::{Piece, PieceType, Side};
use crate::position::Position;
use crate::zobrist::*;

use PieceType::*;

#[derive(Clone)]
pub struct Bitboards {
    pub pawn: u64,
    pub rook: u64,
    pub knight: u64,
    pub bishop: u64,
    pub queen: u64,
    pub king: u64,
}
struct SearchBoardState {
    state: BoardState,
    // Attacked by black
    black_attacked: Bitboards,
    // Attacked by white
    white_attacked: Bitboards,
}

impl Bitboards {
    pub fn get_containing_bitboard_mut(&mut self, pos: Position) -> Option<&mut u64> {
        let mask = pos.as_mask();
        if self.pawn & mask != 0 {
            return Some(&mut self.pawn);
        } else if self.rook & mask != 0 {
            return Some(&mut self.rook);
        } else if self.knight & mask != 0 {
            return Some(&mut self.knight);
        } else if self.bishop & mask != 0 {
            return Some(&mut self.bishop);
        } else if self.queen & mask != 0 {
            return Some(&mut self.queen);
        } else if self.king & mask != 0 {
            return Some(&mut self.king);
        } else {
            None
        }
    }

    pub fn get_role_at_position(&self, pos: Position) -> Option<PieceType> {
        let mask = pos.as_mask();

        if self.pawn & mask != 0 {
            return Some(Pawn);
        } else if self.rook & mask != 0 {
            return Some(Rook);
        } else if self.knight & mask != 0 {
            return Some(Knight);
        } else if self.bishop & mask != 0 {
            return Some(Bishop);
        } else if self.queen & mask != 0 {
            return Some(Queen);
        } else if self.king & mask != 0 {
            return Some(King);
        } else {
            None
        }
    }

    pub fn get_bitboard_mut(&mut self, piece_type: PieceType) -> &mut u64 {
        match piece_type {
            Pawn => &mut self.pawn,
            Rook => &mut self.rook,
            Knight => &mut self.knight,
            Bishop => &mut self.bishop,
            Queen => &mut self.queen,
            King => &mut self.king,
        }
    }

    pub fn combined(&self) -> u64 {
        self.pawn | self.rook | self.knight | self.bishop | self.queen | self.king
    }
}

#[derive(Clone)]
pub struct BoardState {
    pub black: Bitboards,
    pub white: Bitboards,
    pub side: Side,
    pub en_passant_square: Option<Position>,
    pub white_castling: (bool, bool), // long, short
    pub black_castling: (bool, bool), // long, short
    pub zobrist: u64,
}

impl BoardState {
    pub fn update_zobrist(
        &mut self,
        mov: Move,
        piece: Piece,
        move_side: Side,
        en_passant_from_to: (Option<Position>, Option<Position>),
    ) {
        self.zobrist =
            ZOBRIST_RANDOM.update_hash(self.zobrist, mov, piece, move_side, en_passant_from_to)
    }
    pub fn get_bitboard(&self, piece: Piece) -> u64 {
        match piece {
            Piece::White(piece) => match piece {
                Pawn => self.white.pawn,
                Rook => self.white.rook,
                Knight => self.white.knight,
                Bishop => self.white.bishop,
                Queen => self.white.queen,
                King => self.white.king,
            },
            Piece::Black(piece) => match piece {
                Pawn => self.black.pawn,
                Rook => self.black.rook,
                Knight => self.black.knight,
                Bishop => self.black.bishop,
                Queen => self.black.queen,
                King => self.black.king,
            },
        }
    }
    pub fn piece_at_position(&self, pos: Position) -> Option<Piece> {
        if let Some(role) = self.white.get_role_at_position(pos) {
            return Some(Piece::White(role));
        }

        if let Some(role) = self.black.get_role_at_position(pos) {
            return Some(Piece::Black(role));
        }

        None
    }

    pub fn side_castle_rights_mut(&mut self, side: Side) -> &mut (bool, bool) {
        match side {
            Side::White => &mut self.white_castling,
            Side::Black => &mut self.black_castling,
        }
    }
    pub fn side_castle_rights(&mut self, side: Side) -> (bool, bool) {
        match side {
            Side::White => self.white_castling,
            Side::Black => self.black_castling,
        }
    }

    pub fn side_bitboard_mut(&mut self, side: Side) -> &mut Bitboards {
        match side {
            Side::White => &mut self.white,
            Side::Black => &mut self.black,
        }
    }

    pub fn make_move(&self, mov: Move) -> BoardState {
        let mut after_move = self.clone();
        let taken_piece = self.piece_at_position(mov.to());
        let piece = mov.piece_type().with_side(self.side);

        if let Some(this_side) = after_move
            .side_bitboard_mut(self.side)
            .get_containing_bitboard_mut(mov.from())
        {
            *this_side ^= mov.from().as_mask();
            if let Some(promote_to) = mov.promote_to() {
                // if the move includes a promotion, update the bitboard of that type
                // else, update the bitboard of the piece that makes the move
                *after_move
                    .side_bitboard_mut(self.side)
                    .get_bitboard_mut(promote_to) |= mov.to().as_mask()
            } else {
                *this_side |= mov.to().as_mask();
            }
        }

        // if the other side has a piece at the target position, delete it from there as the move is a take
        if let Some(other_side) = after_move
            .side_bitboard_mut(self.side.opposite())
            .get_containing_bitboard_mut(mov.to())
        {
            *other_side ^= mov.to().as_mask();
        }

        // en passant capture
        if let Some(en_passant_square) = self.en_passant_square {
            if mov.to() == en_passant_square {
                *after_move
                    .side_bitboard_mut(self.side.opposite())
                    .get_bitboard_mut(Pawn) ^= mov.from().with_x(mov.to().x()).as_mask()
            }
        }

        // google en passant
        // set the en passant square
        if mov.is_pawn_starter() {
            // the en passant square is the average
            after_move.en_passant_square = Some(mov.en_passant_square())
        } else {
            after_move.en_passant_square = None
        }

        // handle castling
        // castling is notated as a king move that moves 2 squares at once
        // The king's move has already been handled
        match (mov.castle(), after_move.side_castle_rights(after_move.side)) {
            (Castle::Long { from, to }, (true, _)) | (Castle::Short { from, to }, (_, true)) => {
                after_move.side_bitboard_mut(after_move.side).rook ^= from.as_mask() | to.as_mask();
                after_move.zobrist =
                    ZOBRIST_RANDOM.castle_update(after_move.zobrist, after_move.side, from, to)
            }
            _ => {}
        }

        // castling rights
        let mut castled_hash = after_move.zobrist;
        let side = after_move.side;
        let castle_rights = after_move.side_castle_rights_mut(after_move.side);

        if piece.role() == King {
            castle_rights.0 = false;
            castle_rights.1 = false;
            castled_hash = ZOBRIST_RANDOM.update_long_castle_right(castled_hash, side);
            castled_hash = ZOBRIST_RANDOM.update_short_castle_right(castled_hash, side);
        }

        if piece.role() == Rook {
            if mov.from().y() == 0 {
                castle_rights.0 = false;
                castled_hash = ZOBRIST_RANDOM.update_long_castle_right(castled_hash, side);
            } else if mov.from().y() == 7 {
                castle_rights.1 = false;
                castled_hash = ZOBRIST_RANDOM.update_short_castle_right(castled_hash, side);
            }
        }
        if let Some(taken_piece) = taken_piece {
            if taken_piece.role() == Rook {
                if mov.to().y() == 0 {
                    castle_rights.0 = false;
                    castled_hash = ZOBRIST_RANDOM.update_short_castle_right(castled_hash, side);
                } else if mov.to().y() == 7 {
                    castled_hash = ZOBRIST_RANDOM.update_short_castle_right(castled_hash, side);
                    castle_rights.1 = false;
                }
            }
        }
        after_move.zobrist = castled_hash;

        after_move.update_zobrist(
            mov,
            piece,
            after_move.side,
            (self.en_passant_square, after_move.en_passant_square),
        );
        after_move.side = after_move.side.opposite();

        after_move
    }
}

impl Hash for BoardState {
    // a hasher should only care about the zobrist hash
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zobrist.hash(state);
    }
}

impl Default for BoardState {
    fn default() -> Self {
        let black = Bitboards {
            pawn: 0xFF000000000000,
            rook: 0x8100000000000000,
            knight: 0x4200000000000000,
            bishop: 0x2400000000000000,
            king: 0x1000000000000000,
            queen: 0x800000000000000,
        };
        let white = Bitboards {
            pawn: 0xFF00,
            rook: 0x81,
            knight: 0x42,
            bishop: 0x24,
            king: 0x10,
            queen: 0x8,
        };
        let mut state = BoardState {
            black,
            white,
            side: Side::White,
            black_castling: (true, true),
            white_castling: (true, true),
            en_passant_square: None,
            zobrist: 0,
        };
        ZOBRIST_RANDOM.hash_board(&mut state);
        state
    }
}
