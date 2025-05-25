use std::hash::Hash;
use std::rc::{Rc, Weak};

use crate::moving::{Castle, Move};
use crate::piece::{Piece, PieceType, Side};
use crate::position::{self, Position};
use crate::search::MovesIter;
use crate::zobrist::*;

use PieceType::*;

pub struct SearchBoard {
    state: BoardState,
    // Attacked by black
    black_attacked: Bitboards,
    // Attacked by white
    white_attacked: Bitboards,

    black_pin_state: Bitboards,
    white_pin_state: Bitboards,
}

impl SearchBoard {
    pub fn side(&self) -> Side {
        self.state.side
    }
    pub fn curr_side_bitboards(&self) -> &Bitboards {
        self.side_bitboards(self.side())
    }

    pub fn side_bitboards(&self, side: Side) -> &Bitboards {
        self.state.side_bitboard(side)
    }

    pub fn side_attacked(&self, side: Side) -> &Bitboards {
        match side {
            Side::White => &self.white_attacked,
            Side::Black => &self.black_attacked,
        }
    }

    pub fn curr_side_attacked(&self) -> &Bitboards {
        self.side_attacked(self.state.side)
    }

    pub fn get_piece_at(&self, pos: Position) -> Option<Piece> {
        self.state.piece_at_position(pos)
    }

    pub fn find_moves_at<T>(&self, pos: Position, side: Side) -> Option<T>
    where
        T: From<Vec<Move>>,
    {
        use crate::search::*;
        use PieceType::*;
        let type_at = self.state.board.get(pos)?.filter_side(side)?.piece_type;
        let allies = self.curr_side_bitboards().combined();
        let enemies = self.side_bitboards(side.opposite()).combined();
        let must_block = self.side_attacked(side.opposite()).combined();
        let castle_rights = self.state.side_castle_rights(side);
        Some(
            match type_at {
                Pawn => find_pawn(side, pos, allies, enemies, must_block),
                Rook => find_rook(pos, allies, allies | enemies),
                Knight => find_knight(pos, allies),
                Bishop => find_bishop(pos, allies, allies | enemies),
                Queen => find_queen(pos, allies, allies | enemies),
                King => find_king(
                    pos,
                    allies,
                    self.side_attacked(self.side().opposite()).combined(),
                    self.side_bitboards(side.opposite()).combined(),
                    castle_rights,
                ),
            }
            .into(),
        )
    }

    pub fn moves_iter(&self) -> MovesIter {
        MovesIter::init(self)
    }

    pub fn make_move(mut self, mov: Move) -> Self {
        // bitboard shenanigans

        // Castle rights

        // en passant square
        self
    }
}

impl Default for SearchBoard {
    fn default() -> Self {
        Self {
            state: BoardState::default(),
            black_attacked: Bitboards {
                state: [0xFF0000000000, 0, 0xA5000000000000, 0, 0, 0],
            },
            white_attacked: Bitboards {
                state: [0xFF0000, 0, 0xA500, 0, 0, 0],
            },
            white_pin_state: Bitboards { state: [0; 6] },
            black_pin_state: Bitboards { state: [0; 6] },
        }
    }
}

#[derive(Clone)]
pub struct Bitboards {
    // Pawn, rook, knight, bishop, queen, king
    pub state: [u64; 6],
}
impl Bitboards {
    pub fn get_containing_bitboard_mut(&mut self, pos: Position) -> Option<&mut u64> {
        let mask = pos.as_mask();
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

    pub fn combined(&self) -> u64 {
        let mut sum = 0;
        for i in self.state {
            sum |= i
        }
        sum
    }

    pub fn pawn_mut(&mut self) -> &mut u64 {
        &mut self.state[0]
    }

    pub fn rook_mut(&mut self) -> &mut u64 {
        &mut self.state[1]
    }
}

#[derive(Clone)]
pub struct BoardRepr {
    board: [u64; 4],
}
impl BoardRepr {
    #[inline]
    pub fn get(&self, index: Position) -> Option<Piece> {
        let index = *index as usize;
        let quadrant = index / 16;
        let index = index % 16;
        let data: u8 = ((self.board[quadrant] >> index * 4) & 0b1111).to_le_bytes()[0];
        if data == 0b1111 {
            None
        } else {
            Some(unsafe { Piece::from_u8_unchecked(data) })
        }
    }
    pub fn from_bitboards(white: Bitboards, black: Bitboards) -> Self {
        let mut board = [0; 4];

        for k in 0..4 {
            for i in (0 + k * 16)..(16 + k * 16) {
                let pos = Position::from_index(i);
                let i = i % 16;
                if let Some(piece) = white.get_role_at_position(pos) {
                    board[k as usize] |= ((Side::White as u64) + (piece as u64)) << (i * 4u8)
                } else if let Some(piece) = black.get_role_at_position(pos) {
                    board[k as usize] |= ((Side::Black as u64) + (piece as u64)) << (i * 4u8)
                } else {
                    board[k as usize] |= 0b1111
                }
            }
        }

        Self { board }
    }
}

#[derive(Clone)]
pub struct BoardState {
    pub black: Bitboards,
    pub white: Bitboards,
    pub board: BoardRepr,
    pub side: Side,
    pub en_passant_square: Option<Position>,
    pub white_castling: (bool, bool), // long, short
    pub black_castling: (bool, bool), // long, short
    pub zobrist: u64,
    pub halfmove_clock: u8,
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
        use Side::*;
        match piece {
            Piece {
                side: White,
                piece_type: piece,
            } => self.white.get_bitboard(piece),
            Piece {
                side: Black,
                piece_type: piece,
            } => self.black.get_bitboard(piece),
        }
    }
    pub fn piece_at_position(&self, pos: Position) -> Option<Piece> {
        self.board.get(pos);

        None
    }

    pub fn side_castle_rights_mut(&mut self, side: Side) -> &mut (bool, bool) {
        match side {
            Side::White => &mut self.white_castling,
            Side::Black => &mut self.black_castling,
        }
    }
    pub fn side_castle_rights(&self, side: Side) -> (bool, bool) {
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

    pub fn side_bitboard(&self, side: Side) -> &Bitboards {
        match side {
            Side::White => &self.white,
            Side::Black => &self.black,
        }
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
            state: [
                0xFF000000000000,
                0x8100000000000000,
                0x4200000000000000,
                0x2400000000000000,
                0x800000000000000,
                0x1000000000000000,
            ],
        };
        let white = Bitboards {
            state: [0xFF00, 0x81, 0x42, 0x24, 0x8, 0x10],
        };
        let mut state = BoardState {
            board: BoardRepr::from_bitboards(white.clone(), black.clone()),
            black,
            white,
            side: Side::White,
            black_castling: (true, true),
            white_castling: (true, true),
            en_passant_square: None,
            zobrist: 0,
            halfmove_clock: 0,
        };
        ZOBRIST_RANDOM.hash_board(&mut state);
        state
    }
}
