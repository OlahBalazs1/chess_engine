use std::hash::Hash;
use std::ops::Index;
use std::rc::{Rc, Weak};

use crate::moving::{Castle, Move};
use crate::piece::{Piece, PieceType, Side};
use crate::position::{self, Position};
use crate::search_data::{CheckPath, PinState};
use crate::zobrist::*;

use PieceType::*;

pub const PAWN: usize = 0;
pub const ROOK: usize = 1;
pub const KNIGHT: usize = 2;
pub const BISHOP: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;

pub struct SearchBoard {
    pub state: BoardState,

    pub attacked: u64,

    pub pin_state: PinState,

    pub check_paths: CheckPath,
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

    pub fn get_piece_at(&self, pos: Position) -> Option<Piece> {
        self.state.piece_at_position(pos)
    }

    // TODO should probably be in search.rs
    pub fn find_moves_at(&self, moves: &mut Vec<Move>, attack_bits: &mut u64, pos: Position) {
        use crate::search::*;
        use PieceType::*;
        let side = self.side();
        let type_at = match self.state.board.get(pos) {
            Some(i) => match i.filter_side(side) {
                Some(i) => i,
                None => return,
            },
            None => return,
        }
        .piece_type;
        match type_at {
            Pawn => find_pawn(moves, attack_bits, pos, self),
            Rook => find_rook(moves, attack_bits, pos, self),
            Knight => find_knight(moves, attack_bits, pos, self),
            Bishop => find_bishop(moves, attack_bits, pos, self),
            Queen => find_queen(moves, attack_bits, pos, self),
            King => find_king(moves, attack_bits, pos, self),
        };
    }

    pub fn find_all_moves(&self) -> (Vec<Move>, u64) {
        let squares = (0..64).map(|i| Position::from_index(i));
        let mut moves = Vec::with_capacity(128);
        let mut attacked_squares = 0;

        for pos in squares {
            self.find_moves_at(&mut moves, &mut attacked_squares, pos);
        }

        (moves, attacked_squares)
    }
}

impl Default for SearchBoard {
    fn default() -> Self {
        Self {
            state: BoardState::default(),
            attacked: 0x7effff0000000000,
            pin_state: PinState::default(),
            check_paths: CheckPath::default(),
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

// TODO TODO TODO TODO INCREMENTAL MOVES (UNMAKE)
#[derive(Clone)]
pub struct BoardRepr {
    board: [Option<Piece>; 64],
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
}

impl Index<usize> for BoardRepr {
    type Output = Option<Piece>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

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
