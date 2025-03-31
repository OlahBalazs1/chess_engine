use std::hash::Hash;

#[path = "./utils.rs"]
mod utils;

use utils::{Move, Piece, PieceType, Position, Side};
use PieceType::*;

const ZOBRIST_HASHER: ZobristHasher = ZobristHasher::init();

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
    fn update_hash(
        &self,
        mut hash: u64,
        mov: Move,
        piece: Piece,
        move_side: Side,
        en_passant_from_to: (Option<Position>, Option<Position>),
    ) -> u64 {
        hash ^= self.get_value(piece, mov.from);

        if let Some(promoted_to) = mov.promote_to {
            hash ^= self.get_value(promoted_to.with_side(move_side), mov.to)
        } else {
            hash ^= self.get_value(piece, mov.to);
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

    fn castle_update(&self, mut hash: u64, side: Side, from: Position, to: Position) -> u64 {
        hash ^= self.get_value(Rook.with_side(side), from);
        hash ^= self.get_value(Rook.with_side(side), to);
        hash
    }
}

#[derive(Clone)]
pub struct Bitboards {
    pub pawn: u64,
    pub rook: u64,
    pub knight: u64,
    pub bishop: u64,
    pub queen: u64,
    pub king: u64,
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
struct BoardState {
    pub black: Bitboards,
    pub white: Bitboards,
    pub side: Side,
    pub en_passant_square: Option<Position>,
    pub white_castling: (bool, bool),
    pub black_castling: (bool, bool),
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
            ZOBRIST_HASHER.update_hash(self.zobrist, mov, piece, move_side, en_passant_from_to)
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

    pub fn side_bitboard_mut(&mut self, side: Side) -> &mut Bitboards {
        match side {
            Side::White => &mut self.white,
            Side::Black => &mut self.black,
        }
    }

    pub fn make_move(&self, mov: Move, piece: Piece) -> BoardState {
        let mut after_move = self.clone();
        let taken_piece = self.piece_at_position(mov.to);

        if let Some(this_side) = after_move
            .side_bitboard_mut(self.side)
            .get_containing_bitboard_mut(mov.from)
        {
            *this_side ^= mov.from.as_mask();
            if let Some(promote_to) = mov.promote_to {
                // if the move includes a promotion, update the bitboard of that type
                // else, update the bitboard of the piece that makes the move
                *after_move
                    .side_bitboard_mut(self.side)
                    .get_bitboard_mut(promote_to) |= mov.to.as_mask()
            } else {
                *this_side |= mov.to.as_mask();
            }
        }

        // if the other side has a piece at the target position, delete it from there as the move is a take
        if let Some(other_side) = after_move
            .side_bitboard_mut(self.side.opposite())
            .get_containing_bitboard_mut(mov.to)
        {
            *other_side ^= mov.to.as_mask();
        }

        if piece.role() == Pawn && mov.is_pawn_double() {
            // the en passant square is the average
            after_move.en_passant_square = Some(Position::from_index((*mov.from + *mov.to) / 2))
        } else {
            after_move.en_passant_square = None
        }

        // castling rights
        let castle_rights = after_move.side_castle_rights_mut(after_move.side);

        if piece.role() == King {
            castle_rights.0 = false;
            castle_rights.1 = false;
        }

        if piece.role() == Rook {
            if mov.from.y() == 0 {
                castle_rights.0 = false;
            } else if mov.from.y() == 7 {
                castle_rights.1 = false;
            }
        }
        if let Some(taken_piece) = taken_piece {
            if taken_piece.role() == Rook {
                let castle_rights = after_move.side_castle_rights_mut(taken_piece.side());
                if mov.to.y() == 0 {
                    castle_rights.0 = false;
                } else if mov.to.y() == 7 {
                    castle_rights.1 = false;
                }
            }
        }

        // handle castling
        // castling is notated as a king move that moves 2 squares at once
        if piece.role() == King && ((mov.from.x() as i8) - (mov.to.x() as i8)).abs() == 2 {
            // handle the King
            after_move.side_bitboard_mut(after_move.side).king ^= mov.as_mask();

            let castle_from = if mov.to.x() < 3 {
                mov.from.with_x(0)
            } else {
                mov.from.with_x(7)
            };

            let castle_to = Position::from_index((*mov.from + *mov.to) / 2);

            after_move.side_bitboard_mut(after_move.side).rook ^=
                castle_from.as_mask() | castle_to.as_mask();

            after_move.zobrist = ZOBRIST_HASHER.castle_update(
                after_move.zobrist,
                after_move.side,
                castle_from,
                castle_to,
            )
        }

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
