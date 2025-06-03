use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Index;
use std::rc::{Rc, Weak};

use crate::magic_bitboards::print_bits;
use crate::moving::{Castle, Move, MoveType, Unmove};
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

#[derive(Clone, PartialEq)]
pub struct SearchBoard {
    pub state: BoardState,
    pub attacked: u64,
    pub pin_state: PinState,
    pub check_paths: CheckPath,
    pub halfmove_clock: u8,
    pub white_king: Position,
    pub black_king: Position,
}
macro_rules! allies {
    ($side: ident, $state: ident) => {
        $state.side_bitboards_mut($side)
    };
}
macro_rules! enemies {
    ($side: ident, $state: ident) => {
        $state.side_bitboards_mut($side.opposite())
    };
}

macro_rules! side_castle_rights {
    ($side: ident, $state: ident) => {
        $state.state.side_castle_rights_mut($side)
    };
}

impl SearchBoard {
    pub fn side(&self) -> Side {
        self.state.side
    }

    pub fn get_bitboard_mut(&mut self, piece: Piece) -> &mut u64 {
        self.side_bitboards_mut(piece.side())
            .get_bitboard_mut(piece.role())
    }
    pub fn curr_side_bitboards(&self) -> &Bitboards {
        self.side_bitboards(self.side())
    }

    pub fn side_bitboards(&self, side: Side) -> &Bitboards {
        self.state.side_bitboard(side)
    }

    pub fn side_bitboards_mut(&mut self, side: Side) -> &mut Bitboards {
        self.state.side_bitboard_mut(side)
    }

    pub fn get_piece_at(&self, pos: Position) -> Option<Piece> {
        self.state.piece_at_position(pos)
    }

    pub fn side_king(&self, side: Side) -> Position {
        match side {
            Side::White => self.white_king,
            Side::Black => self.black_king,
        }
    }
    pub fn side_king_mut(&mut self, side: Side) -> &mut Position {
        match side {
            Side::White => &mut self.white_king,
            Side::Black => &mut self.black_king,
        }
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

    pub fn make<'a>(&mut self, mov: &'a Move, attacked_squares: u64) -> Unmove<'a> {
        let side = self.state.side;

        let unmake = Unmove {
            mov,
            en_passant_square: self.state.en_passant_square.clone(),
            white_castling: self.state.white_castling.clone(),
            black_castling: self.state.black_castling.clone(),
            zobrist: self.state.zobrist,
            attacked_squares: self.attacked,
            halfmove_clock: self.halfmove_clock,
            pin_state: self.pin_state.clone(),
            check_path: self.check_paths.clone(),
        };

        let mut increment_halfmove = true;

        let mut updated_ep = None;

        self.state
            .zobrist
            .update(mov.piece_type().with_side(side), mov.from());
        self.state
            .zobrist
            .update(mov.piece_type().with_side(side), mov.to());

        let piece = mov.piece_type();
        *allies!(side, self).get_bitboard_mut(piece) ^= mov.from().as_mask() | mov.to().as_mask();
        self.state
            .board
            .board
            .swap(*mov.from() as usize, *mov.to() as usize);

        if let Some(taken) = mov.take {
            *self.get_bitboard_mut(taken) ^= mov.to().as_mask();
            increment_halfmove = false;

            self.state.board.board[*mov.from() as usize] = None;
        }
        match mov.move_type {
            MoveType::Normal(PieceType::Rook) => {
                if mov.from().x() == 0 {
                    side_castle_rights!(side, self).0 = false
                } else if mov.from().x() == 7 {
                    side_castle_rights!(side, self).1 = false
                }
            }
            MoveType::Normal(PieceType::Pawn) if mov.is_pawn_starter() => {
                increment_halfmove = false;
                updated_ep = Some(mov.from().with_y(side.pers_y(2)).unwrap());
            }
            MoveType::Promotion(p) => {
                self.state.board.board[*mov.to() as usize] = Some(p.with_side(side));
                allies!(side, self).state[PAWN] ^= mov.to().as_mask();
                *allies!(side, self).get_bitboard_mut(p) ^= mov.to().as_mask();
            }
            MoveType::LongCastle => {
                self.state.board.board.swap(
                    (5 + side.home_y() * 8) as usize,
                    (7 + side.home_y() * 8) as usize,
                );
                allies!(side, self).state[ROOK] ^=
                    1 << (5 + side.home_y() * 8) | (1 << (7 + side.home_y() * 8));
            }
            MoveType::ShortCastle => {
                self.state.board.board.swap(
                    (0 + side.home_y() * 8) as usize,
                    (2 + side.home_y() * 8) as usize,
                );
                allies!(side, self).state[ROOK] ^=
                    1 << (side.home_y() * 8) | (1 << (2 + side.home_y() * 8));
            }
            MoveType::EnPassant => {
                let ep_pawn = mov.to().with_y(side.pers_y(3)).unwrap();
                increment_halfmove = false;
                enemies!(side, self).state[PAWN] ^= ep_pawn.as_mask();

                // set to taken in unmake
                self.state.board.board[*ep_pawn as usize] = None;
            }
            _ => {}
        }

        if increment_halfmove {
            self.halfmove_clock += 1;
        }
        self.pin_state = PinState::find(&self.state, self.side_king(side));
        self.check_paths = CheckPath::find(&self.state, self.side_king(side), side.opposite());
        // self.pin_state = PinState::default();
        // self.check_paths = CheckPath::default();

        self.state
            .zobrist
            .update_ep_square(side, self.state.en_passant_square, updated_ep);

        self.state.zobrist.switch_side();

        if mov.piece_type() == PieceType::King {
            *self.side_king_mut(side) = mov.to;
            if self.state.side_castle_rights(side).0 {
                side_castle_rights!(side, self).0 = false;
                self.state.zobrist.update_long_castle(side);
            }
            if self.state.side_castle_rights(side).1 {
                side_castle_rights!(side, self).1 = false;
                self.state.zobrist.update_short_castle(side);
            }
        }

        self.state.side = self.state.side.opposite();
        self.attacked = attacked_squares;

        unmake
    }

    pub fn unmake(&mut self, unmove: Unmove) {
        self.state.side = self.state.side.opposite();
        let side = self.state.side;
        let mov = unmove.mov;

        let piece = mov.piece_type();
        *allies!(side, self).get_bitboard_mut(piece) ^= mov.from().as_mask() | mov.to().as_mask();
        self.state
            .board
            .board
            .swap(*mov.from() as usize, *mov.to() as usize);

        if let Some(taken) = mov.take {
            *self.get_bitboard_mut(taken) ^= mov.to().as_mask();

            self.state.board.board[*mov.to() as usize] = Some(taken);
        }
        match mov.move_type {
            MoveType::Promotion(p) => {
                self.state.board.board[*mov.to() as usize] = None;
                allies!(side, self).state[PAWN] ^= mov.to().as_mask();
                *allies!(side, self).get_bitboard_mut(p) ^= mov.to().as_mask();
            }
            MoveType::LongCastle => {
                self.state.board.board.swap(
                    (5 + side.home_y() * 8) as usize,
                    (7 + side.home_y() * 8) as usize,
                );
                allies!(side, self).state[ROOK] ^=
                    1 << (5 + side.home_y() * 8) | (1 << (7 + side.home_y() * 8));
            }
            MoveType::ShortCastle => {
                self.state.board.board.swap(
                    (0 + side.home_y() * 8) as usize,
                    (2 + side.home_y() * 8) as usize,
                );
                allies!(side, self).state[ROOK] ^=
                    1 << (side.home_y() * 8) | (1 << (2 + side.home_y() * 8));
            }
            MoveType::EnPassant => {
                let ep_pawn = mov.to().with_y(side.pers_y(3)).unwrap();
                enemies!(side, self).state[PAWN] ^= ep_pawn.as_mask();

                self.state.board.board[*ep_pawn as usize] = Some(PieceType::Pawn.with_side(side));
            }
            _ => {}
        }

        if piece == PieceType::King {
            *self.side_king_mut(side) = mov.from;
        }

        self.state.en_passant_square = unmove.en_passant_square;
        self.state.white_castling = unmove.white_castling;
        self.state.black_castling = unmove.black_castling;
        self.state.zobrist = unmove.zobrist;
        self.attacked = unmove.attacked_squares;
        self.halfmove_clock = unmove.halfmove_clock;
        self.pin_state = unmove.pin_state;
        self.check_paths = unmove.check_path;
    }
}

impl Default for SearchBoard {
    fn default() -> Self {
        Self {
            state: BoardState::default(),
            attacked: 0x7effff0000000000,
            pin_state: PinState::default(),
            check_paths: CheckPath::default(),
            halfmove_clock: 0,
            white_king: Position::new(4, 0),
            black_king: Position::new(4, 7),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, PartialEq)]
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

#[derive(Clone, PartialEq)]
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

impl Debug for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..8).rev() {
            write!(f, "\n")?;
            for x in 0..8 {
                let i = self.board.board[x + y * 8];
                if let Some(i) = i {
                    Debug::fmt(&i, f)?
                } else {
                    write!(f, " ")?
                }
            }
        }
        write!(f, "\nblack: ")?;
        Debug::fmt(&self.black, f)?;
        write!(f, "\nwhite: ")?;
        Debug::fmt(&self.white, f)?;
        write!(f, "\nblack castle rights: ")?;
        Debug::fmt(&self.black_castling, f)?;
        write!(f, "\nwhite castle rights: ")?;
        Debug::fmt(&self.white_castling, f)?;
        write!(f, "\nzobrist: ")?;
        Debug::fmt(&self.zobrist, f)?;
        write!(f, "\nside: ")?;
        Debug::fmt(&self.side, f)?;
        Ok(())
    }
}
impl Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..8).rev() {
            write!(f, "\n")?;
            for x in 0..8 {
                let i = self.board.board[x + y * 8];
                if let Some(i) = i {
                    Debug::fmt(&i, f)?
                } else {
                    write!(f, " ")?
                }
            }
        }
        Ok(())
    }
}
