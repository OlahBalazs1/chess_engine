use std::cell::LazyCell;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::mem;
use std::ops::{Index, IndexMut};
use std::pin::Pin;

use crate::magic_bitboards::{print_bits, MAGIC_MOVER};
use crate::moving::{Castle, Move, MoveType, Unmove};
use crate::piece::{self, Piece, PieceType, Side};
use crate::position::{self, Position};
use crate::search_data::{CheckPath, PinState};
use crate::search_masks::{KING_MASKS, KNIGHT_MASKS, PAWN_TAKE_MASKS};
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
    pub halfmove_clock: u8,
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
        self.state.side_king(side)
    }
    pub fn side_king_mut(&mut self, side: Side) -> &mut Position {
        self.state.side_king_mut(side)
    }

    pub fn find_all_moves(
        &self,
        pin_state: PinState,
        check_paths: CheckPath,
        attacked_squares: u64,
    ) -> Vec<Move> {
        use crate::search::*;
        let mut moves = Vec::with_capacity(128);

        let pieces = self
            .state
            .board
            .board
            .iter()
            .enumerate()
            .map(|(pos, piece)| {
                (
                    Position::from_index(pos as u8),
                    piece.and_then(|piece| piece.filter_side(self.state.side)),
                )
            });

        for (pos, piece) in pieces {
            if let Some(piece) = piece {
                match piece.piece_type {
                    Pawn => find_pawn(&mut moves, pos, self, &pin_state, &check_paths),
                    Rook => find_rook(&mut moves, pos, self, &pin_state, &check_paths),
                    Knight => find_knight(&mut moves, pos, self, &pin_state, &check_paths),
                    Bishop => find_bishop(&mut moves, pos, self, &pin_state, &check_paths),
                    Queen => find_queen(&mut moves, pos, self, &pin_state, &check_paths),
                    King => find_king(&mut moves, pos, self, &check_paths, attacked_squares),
                };
            }
        }

        // for pos in squares {
        //     self.find_moves_at(&mut moves, &mut attacked_squares, pos);
        // }

        moves
    }

    pub fn make<'a>(&mut self, mov: &'a Move) {
        let side = self.state.side;

        let mut increment_halfmove = true;
        let mut updated_ep = None;

        let piece = mov.piece_type();

        self.state.zobrist.update(piece.with_side(side), mov.from());
        self.state.zobrist.update(piece.with_side(side), mov.to());

        *allies!(side, self).get_bitboard_mut(piece) ^= mov.from().as_mask() | mov.to().as_mask();
        self.state.board.board[*mov.to() as usize] =
            mem::replace(&mut self.state.board.board[*mov.from() as usize], None);

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
                self.state.board.board[(5 + side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(7 + side.home_y() * 8) as usize],
                    None,
                );
                allies!(side, self).state[ROOK] ^=
                    1 << (5 + side.home_y() * 8) | (1 << (7 + side.home_y() * 8));
            }
            MoveType::ShortCastle => {
                self.state.board.board[(2 + side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(side.home_y() * 8) as usize],
                    None,
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
        // match self.check_paths {
        //     CheckPath::Blockable(path) => print_bits(path),
        //     _ => {}
        // }
        // self.pin_state = PinState::default();
        // self.check_paths = CheckPath::default();

        self.state
            .zobrist
            .update_ep_square(side, self.state.en_passant_square, updated_ep);

        self.state.zobrist.switch_side();

        if piece == PieceType::King {
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
    }

    pub fn unmake(&mut self, unmove: Unmove) {
        self.state.side = self.state.side.opposite();
        let side = self.state.side;
        let mov = unmove.mov;

        let piece = mov.piece_type();
        *allies!(side, self).get_bitboard_mut(piece) ^= mov.from().as_mask() | mov.to().as_mask();

        if let Some(taken) = mov.take {
            *self.get_bitboard_mut(taken) ^= mov.to().as_mask();
            self.state.board.board[*mov.from() as usize] =
                mem::replace(&mut self.state.board.board[*mov.to() as usize], Some(taken));
        } else {
            self.state.board.board[*mov.from() as usize] =
                mem::replace(&mut self.state.board.board[*mov.to() as usize], None)
        }
        match mov.move_type {
            MoveType::Promotion(p) => {
                self.state.board.board[*mov.to() as usize] = None;
                allies!(side, self).state[PAWN] ^= mov.to().as_mask();
                *allies!(side, self).get_bitboard_mut(p) ^= mov.to().as_mask();
            }
            MoveType::LongCastle => {
                self.state.board.board[(7 + side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(5 + side.home_y() * 8) as usize],
                    None,
                );
                allies!(side, self).state[ROOK] ^=
                    1 << (5 + side.home_y() * 8) | (1 << (7 + side.home_y() * 8));
            }
            MoveType::ShortCastle => {
                self.state.board.board[(side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(2 + side.home_y() * 8) as usize],
                    None,
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
        self.halfmove_clock = unmove.halfmove_clock;
    }

    pub fn from_fen(fen: &str) -> Self {
        let state = BoardState::from_fen(fen);
        let halfmove_clock = fen
            .split(" ")
            .nth(4)
            .expect("Invalid FEN")
            .parse()
            .expect("Invalid FEN");
        let attacked = state.get_attacked(state.side);

        let white_king = state.find_king(Side::White);
        let black_king = state.find_king(Side::Black);
        Self {
            halfmove_clock,
            state,
        }
    }
}

impl Default for SearchBoard {
    fn default() -> Self {
        Self {
            state: BoardState::default(),
            halfmove_clock: 0,
        }
    }
}

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
        panic!("No king found")
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
    pub white_king: Position,
    pub black_king: Position,
}

impl BoardState {
    pub fn can_be_taken(&self, pos: Position, enemy: Side) -> bool {
        let enemies = self.side_bitboard(enemy);
        let allies = self.side_bitboard(enemy.opposite()).combined();

        let pawn_lookup_pos = match self.side {
            Side::White => pos.add_x(1).unwrap(),
            Side::Black => pos.sub_x(1).unwrap(),
        };

        if KNIGHT_MASKS[*pos as usize].sum & enemies[KNIGHT] != 0
            || PAWN_TAKE_MASKS[*pawn_lookup_pos as usize].sum & enemies[PAWN] != 0
            || KING_MASKS[*pos as usize].sum & enemies[KING] != 0
        {
            return true;
        } else if MAGIC_MOVER
            .get_rook(pos, enemies.combined() | allies)
            .bitboard
            & (enemies[ROOK] | enemies[QUEEN])
            != 0
            || MAGIC_MOVER
                .get_bishop(pos, enemies.combined() | allies)
                .bitboard
                & (enemies[BISHOP] | enemies[QUEEN])
                != 0
        {
            return true;
        }
        false
    }
    pub fn legal_data(&self) -> (PinState, CheckPath) {
        let side = self.side;
        (
            PinState::find(self, self.side_king(side)),
            CheckPath::find(self, self.side_king(side), side.opposite()),
        )
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
    pub fn find_king(&self, side: Side) -> Position {
        let pieces = self.board.board.iter().enumerate().map(|(pos, piece)| {
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
        panic!("No king found")
    }
    pub fn get_attacked(&self, enemy: Side) -> u64 {
        let mut attacked_squares = 0;

        let all = self.white.combined() | self.black.combined();

        let pieces = self
            .board
            .board
            .iter()
            .enumerate()
            .map(|(pos, piece)| {
                (
                    Position::from_index(pos as u8),
                    piece.and_then(|p| p.filter_side(enemy)),
                )
            })
            .filter(|(_, piece)| piece.is_some())
            .map(|(pos, piece)| (pos, piece.unwrap()));

        for (pos, piece) in pieces {
            attacked_squares |= match piece.piece_type {
                Pawn => {
                    PAWN_TAKE_MASKS[match enemy {
                        Side::White => *pos + 8,
                        Side::Black => *pos - 8,
                    } as usize]
                        .sum
                }
                Rook => MAGIC_MOVER.get_rook(pos, all).bitboard,
                Knight => KNIGHT_MASKS[*pos as usize].sum,
                Bishop => MAGIC_MOVER.get_bishop(pos, all).bitboard,
                Queen => {
                    MAGIC_MOVER.get_rook(pos, all).bitboard
                        | MAGIC_MOVER.get_bishop(pos, all).bitboard
                }
                King => KING_MASKS[*pos as usize].sum,
            };
        }

        attacked_squares
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

    pub fn from_fen(fen: &str) -> Self {
        let [piece_data, active, rights, ep, _, _] = fen.split(" ").collect::<Vec<_>>()[..] else {
            panic!("Invalid FEN")
        };
        let mut white_bits = Bitboards { state: [0; 6] };
        let mut black_bits = Bitboards { state: [0; 6] };
        let piecewise: BoardRepr = {
            let mut temp = [None; 64];
            let mut square = 0;
            for i in piece_data.chars() {
                match i {
                    '1'..='8' => {
                        square += (i as usize) - (b'0' as usize);
                        continue;
                    }
                    '/' => continue,
                    'p' => {
                        temp[square] = Some(Piece::black(Pawn));
                        black_bits.state[PAWN] |= 1 << square
                    }
                    'r' => {
                        temp[square] = Some(Piece::black(Rook));
                        black_bits.state[ROOK] |= 1 << square
                    }
                    'n' => {
                        temp[square] = Some(Piece::black(Knight));
                        black_bits.state[KNIGHT] |= 1 << square
                    }
                    'b' => {
                        temp[square] = Some(Piece::black(Bishop));
                        black_bits.state[BISHOP] |= 1 << square
                    }
                    'q' => {
                        temp[square] = Some(Piece::black(Queen));
                        black_bits.state[QUEEN] |= 1 << square
                    }
                    'k' => {
                        temp[square] = Some(Piece::black(King));
                        black_bits.state[KING] |= 1 << square
                    }
                    'P' => {
                        temp[square] = Some(Piece::white(Pawn));
                        white_bits.state[PAWN] |= 1 << square
                    }
                    'R' => {
                        temp[square] = Some(Piece::white(Rook));
                        white_bits.state[ROOK] |= 1 << square
                    }
                    'N' => {
                        temp[square] = Some(Piece::white(Knight));
                        white_bits.state[KNIGHT] |= 1 << square
                    }
                    'B' => {
                        temp[square] = Some(Piece::white(Bishop));
                        white_bits.state[BISHOP] |= 1 << square
                    }
                    'Q' => {
                        temp[square] = Some(Piece::white(Queen));
                        white_bits.state[QUEEN] |= 1 << square
                    }
                    'K' => {
                        temp[square] = Some(Piece::white(King));
                        white_bits.state[KING] |= 1 << square
                    }
                    _ => panic!("Invalid FEN"),
                }
                square += 1;
            }
            BoardRepr { board: temp }
        };

        let active = match active {
            "w" => Side::White,
            "b" => Side::Black,
            _ => panic!("Invalid FEN"),
        };
        let mut white_rights = (false, false);
        let mut black_rights = (false, false);

        for i in rights.chars() {
            match i {
                'K' | 'H' => white_rights.1 = true,
                'Q' | 'A' => white_rights.0 = true,
                'k' | 'h' => black_rights.1 = true,
                'q' | 'a' => black_rights.0 = true,
                '-' => break,
                _ => panic!("Invalid FEN"),
            }
        }
        let en_passant_square = match ep {
            "-" => None,
            ep => {
                let mut chars = ep.chars();
                let ep_square = Position::new(
                    (chars.next().expect("Invalid FEN") as u8) - b'a',
                    (chars.next().expect("Invalid FEN") as u8) - b'1',
                );
                Some(ep_square)
            }
        };

        let mut temp_board = BoardState {
            black: black_bits,
            white: white_bits,
            white_king: piecewise.find_king(Side::White),
            black_king: piecewise.find_king(Side::Black),
            board: piecewise,
            side: active,
            en_passant_square,
            white_castling: white_rights,
            black_castling: black_rights,
            zobrist: 0,
        };
        ZOBRIST_RANDOM.hash_board(&mut temp_board);
        temp_board
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
            white_king: Position::new(4, 0),
            black_king: Position::new(4, 7),
            board: BoardRepr::from_bitboards(white.clone(), black.clone()),
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
