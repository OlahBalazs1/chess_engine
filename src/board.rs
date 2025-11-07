use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::mem;
use std::ops::{Deref, DerefMut};

use crate::board_repr::*;
use crate::magic_bitboards::MAGIC_MOVER;
use crate::moving::{Move, MoveType, Unmove};
use crate::piece::{Piece, PieceType, Side};
use crate::position::Position;
use crate::search_data::{CheckPath, PinState};
use crate::search_masks::{KING_MASKS, KNIGHT_MASKS, choose_home_rook, choose_pawn_take_mask};
use crate::zobrist::*;

use PieceType::*;

// TODO: Avoid deref polymorphism
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

impl SearchBoard {
    pub fn find_all_moves(&self, pin_state: PinState, check_paths: CheckPath) -> Vec<Move> {
        use crate::search::*;
        let mut moves = Vec::with_capacity(219);
        let enemy = self.side().opposite();
        let mut attacked_squares = 0;
        let all = (self.white.combined() | self.black.combined())
            & !(self.side_bitboards(enemy.opposite())[KING]);

        for pos in (0..64).map(Position::from_index) {
            if let Some(piece) = self.board.board[*pos as usize] {
                if self.side() == piece.side() {
                    match piece.piece_type {
                        Pawn => find_pawn(&mut moves, pos, self, &pin_state, &check_paths),
                        Rook => find_rook(&mut moves, pos, self, &pin_state, &check_paths),
                        Knight => find_knight(&mut moves, pos, self, &pin_state, &check_paths),
                        Bishop => find_bishop(&mut moves, pos, self, &pin_state, &check_paths),
                        Queen => find_queen(&mut moves, pos, self, &pin_state, &check_paths),
                        _ => {}
                    };
                } else {
                    attacked_squares |= match piece.piece_type {
                        Pawn => choose_pawn_take_mask(enemy)[*pos as usize].sum,
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
            }
        }
        find_king(
            &mut moves,
            self.find_king(self.side()),
            self,
            &check_paths,
            attacked_squares,
        );
        moves
    }

    pub fn make<'a>(&mut self, mov: &'a Move) {
        let ally_side = self.state.side;
        let enemy_side = ally_side.opposite();

        let mut increment_halfmove = true;
        let mut updated_ep = None;

        let piece = mov.piece_type();

        self.state
            .zobrist
            .update(piece.with_side(ally_side), mov.from());
        self.state
            .zobrist
            .update(piece.with_side(ally_side), mov.to());
        self.state.board.board[*mov.to() as usize] =
            mem::replace(&mut self.state.board.board[*mov.from() as usize], None);

        *allies!(ally_side, self).get_bitboard_mut(piece) ^=
            mov.from().as_mask() | mov.to().as_mask();

        if let Some(taken) = mov.take {
            *self.get_bitboard_mut(taken) ^= mov.to().as_mask();
            increment_halfmove = false;
        }
        match mov.move_type {
            MoveType::Normal(PieceType::Pawn) if mov.is_pawn_starter() => {
                increment_halfmove = false;
                updated_ep = Some(mov.from().with_y(ally_side.pers_y(2)).unwrap());
            }
            MoveType::Promotion(p) => {
                self.state.board.board[*mov.to() as usize] = Some(p.with_side(ally_side));
                allies!(ally_side, self).state[PAWN] ^= mov.to().as_mask();
                *allies!(ally_side, self).get_bitboard_mut(p) ^= mov.to().as_mask();
            }
            MoveType::LongCastle => {
                self.state.board.board[(3 + ally_side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(0 + ally_side.home_y() * 8) as usize],
                    None,
                );
                allies!(ally_side, self).state[ROOK] ^= 0x9 << (ally_side.home_y() * 8);
            }
            MoveType::ShortCastle => {
                self.state.board.board[(5 + ally_side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(7 + ally_side.home_y() * 8) as usize],
                    None,
                );
                allies!(ally_side, self).state[ROOK] ^= 0xa0 << (ally_side.home_y() * 8);
            }
            MoveType::EnPassant => {
                let ep_pawn = mov.to().with_y(ally_side.pers_y(4)).unwrap();
                increment_halfmove = false;
                enemies!(ally_side, self).state[PAWN] ^= ep_pawn.as_mask();

                // set to taken in unmake
                self.state.board.board[*ep_pawn as usize] = None;
            }
            _ => {}
        }

        if increment_halfmove {
            self.halfmove_clock += 1;
        } else {
            self.halfmove_clock = 0;
        }

        self.state
            .zobrist
            .update_ep_square(ally_side, self.state.en_passant_square, updated_ep);
        self.state.en_passant_square = updated_ep;

        self.state.zobrist.switch_side();

        let ally_home_rook = choose_home_rook(ally_side);
        if mov.from() == ally_home_rook[0] {
            if self.state.side_castle_rights(ally_side).0 {
                self.state.zobrist.update_long_castle(ally_side);
            }
            self.side_castle_rights_mut(ally_side).0 = false;
        } else if mov.from() == ally_home_rook[1] {
            if self.state.side_castle_rights(ally_side).1 {
                self.state.zobrist.update_short_castle(ally_side);
            }
            self.side_castle_rights_mut(ally_side).1 = false;
        }
        let enemy_home_rook = choose_home_rook(enemy_side);
        if mov.to() == enemy_home_rook[0] {
            if self.state.side_castle_rights(enemy_side).0 {
                self.state.zobrist.update_long_castle(enemy_side);
            }
            self.side_castle_rights_mut(enemy_side).0 = false;
        } else if mov.to() == enemy_home_rook[1] {
            if self.state.side_castle_rights(enemy_side).1 {
                self.state.zobrist.update_short_castle(enemy_side);
            }
            self.side_castle_rights_mut(enemy_side).1 = false;
        }

        if piece == PieceType::King {
            *self.side_king_mut(ally_side) = mov.to;
            if self.state.side_castle_rights(ally_side).0 {
                self.side_castle_rights_mut(ally_side).0 = false;
                self.state.zobrist.update_long_castle(ally_side);
            }
            if self.state.side_castle_rights(ally_side).1 {
                self.side_castle_rights_mut(ally_side).1 = false;
                self.state.zobrist.update_short_castle(ally_side);
            }
        }

        self.state.side = self.state.side.opposite();
    }

    pub fn unmake(&mut self, unmove: Unmove) {
        self.state.side = self.state.side.opposite();
        let ally_side = self.state.side;
        let mov = unmove.mov;

        let piece = mov.piece_type();
        *allies!(ally_side, self).get_bitboard_mut(piece) ^=
            mov.from().as_mask() | mov.to().as_mask();

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
                self.state.board.board[*mov.from() as usize] = Some(Pawn.with_side(ally_side));
                allies!(ally_side, self).state[PAWN] ^= mov.to().as_mask();
                *allies!(ally_side, self).get_bitboard_mut(p) ^= mov.to().as_mask();
            }
            MoveType::LongCastle => {
                self.state.board.board[(0 + ally_side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(3 + ally_side.home_y() * 8) as usize],
                    None,
                );
                allies!(ally_side, self).state[ROOK] ^= 0x9 << (ally_side.home_y() * 8);
            }
            MoveType::ShortCastle => {
                self.state.board.board[(7 + ally_side.home_y() * 8) as usize] = mem::replace(
                    &mut self.state.board.board[(5 + ally_side.home_y() * 8) as usize],
                    None,
                );
                allies!(ally_side, self).state[ROOK] ^= 0xa0 << (ally_side.home_y() * 8);
            }
            MoveType::EnPassant => {
                // the pawn that is taken
                let ep_pawn = mov.to().with_y(ally_side.pers_y(4)).unwrap();
                // restore taken pawn
                enemies!(ally_side, self).state[PAWN] |= ep_pawn.as_mask();

                self.state.board.board[*ep_pawn as usize] =
                    Some(PieceType::Pawn.with_side(ally_side.opposite()));
            }
            _ => {}
        }

        if piece == PieceType::King {
            *self.side_king_mut(ally_side) = mov.from;
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
        Self {
            halfmove_clock,
            state,
        }
    }
}

impl Deref for SearchBoard {
    type Target = BoardState;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl DerefMut for SearchBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Default for SearchBoard {
    fn default() -> Self {
        let state = BoardState::default();
        Self {
            state,
            halfmove_clock: 0,
        }
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
    pub fn side(&self) -> Side {
        self.side
    }
    pub fn can_be_taken(&self, pos: Position, enemy: Side) -> bool {
        let enemies = self.side_bitboards(enemy);
        let allies = self.side_bitboards(enemy.opposite()).combined();

        let pawn_lookup_pos = match self.side {
            Side::White => pos.add_x(1).unwrap(),
            Side::Black => pos.sub_x(1).unwrap(),
        };

        if KNIGHT_MASKS[*pos as usize].sum & enemies[KNIGHT] != 0
            || choose_pawn_take_mask(enemy)[*pawn_lookup_pos as usize].sum & enemies[PAWN] != 0
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
        self.board.find_king(side)
    }
    pub fn get_attacked(&self, enemy: Side) -> u64 {
        let mut attacked_squares = 0;
        let enemies = self.side_bitboards(self.side().opposite()).combined();

        let all = (self.white.combined() | self.black.combined())
            & !(self.side_bitboards(enemy.opposite())[KING]);

        for pos in (0..64).map(Position::from_index) {
            if enemies & pos.as_mask() == 0 {
                continue;
            }
            unsafe {
                attacked_squares |= match self.board.board[*pos as usize]
                    .unwrap_unchecked()
                    .piece_type
                {
                    Pawn => choose_pawn_take_mask(enemy)[*pos as usize].sum,
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
    pub fn get_bitboard_mut(&mut self, piece: Piece) -> &mut u64 {
        use Side::*;
        match piece {
            Piece {
                side: White,
                piece_type: piece,
            } => self.white.get_bitboard_mut(piece),
            Piece {
                side: Black,
                piece_type: piece,
            } => self.black.get_bitboard_mut(piece),
        }
    }
    pub fn get_piece_at(&self, pos: Position) -> Option<Piece> {
        self.board.get(pos)
    }

    pub fn from_fen(fen: &str) -> Self {
        let split = fen.split(" ").collect::<Vec<_>>();
        let [piece_data, active, rights, ep, _, _] = split[..6] else {
            panic!("Invalid FEN")
        };
        let mut white_bits = Bitboards { state: [0; 6] };
        let mut black_bits = Bitboards { state: [0; 6] };
        let piecewise: BoardRepr = {
            let mut temp = [None; 64];
            let mut x = 0;
            let mut y = 7;
            for i in piece_data.chars() {
                let square = x + y * 8;
                match i {
                    '1'..='8' => {
                        x += (i as usize) - (b'0' as usize);
                        continue;
                    }
                    '/' => {
                        x = 0;
                        y -= 1;
                        continue;
                    }
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
                x += 1;
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

    pub fn side_bitboards_mut(&mut self, side: Side) -> &mut Bitboards {
        match side {
            Side::White => &mut self.white,
            Side::Black => &mut self.black,
        }
    }

    pub fn side_bitboards(&self, side: Side) -> &Bitboards {
        match side {
            Side::White => &self.white,
            Side::Black => &self.black,
        }
    }

    pub fn is_in_check(&self) -> bool {
        self.can_be_taken(self.find_king(self.side()), self.side().opposite())
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
