use std::pin::Pin;

use crate::{
    board::{BoardState, SearchBoard, BISHOP, KNIGHT, QUEEN, ROOK},
    magic_bitboards::{MagicData, MagicMover, MAGIC_MOVER},
    piece::Side,
    position::Position,
    search_masks::{KNIGHT_MASKS, PAWN_TAKE_MASKS},
};

pub struct PinState {
    pub diagonal: u64,
    pub x_aligned: u64,
    pub y_aligned: u64,
}

impl PinState {
    pub fn choose_relevant(&self, pos: Position) -> u64 {
        let mask = pos.as_mask();
        if self.diagonal & mask != 0 {
            return self.diagonal;
        } else if self.x_aligned & mask != 0 {
            return self.x_aligned;
        } else if self.y_aligned & mask != 0 {
            return self.y_aligned;
        }

        0
    }
}

impl Default for PinState {
    fn default() -> Self {
        Self {
            diagonal: 0,
            x_aligned: 0,
            y_aligned: 0,
        }
    }
}

pub enum CheckPath {
    Multiple,
    Blockable(u64),
    None,
}

macro_rules! update_checkpath {
    ($i:ident, $to:expr) => {
        match $i {
            crate::search_data::CheckPath::None => {
                $i = crate::search_data::CheckPath::Blockable($to)
            }
            crate::search_data::CheckPath::Blockable(_) => {
                $i = Self::Multiple;
                return $i;
            }
            crate::search_data::CheckPath::Multiple => {
                return $i;
            }
        }
    };
    ($i:ident, $to:expr, $ret:expr) => {
        match $i {
            crate::search_data::CheckPath::None => {
                $i = crate::search_data::CheckPath::Blockable($to)
            }
            crate::search_data::CheckPath::Blockable(_) => {
                $i = Self::Multiple;
                return $ret;
            }
            crate::search_data::CheckPath::Multiple => {
                return $ret;
            }
        }
    };
}
impl CheckPath {
    pub fn find(board: &BoardState, king_pos: Position, side: Side) -> Self {
        MAGIC_MOVER.with(|m| CheckPath::find_with(board, king_pos, side, m))
    }

    fn find_with(
        board: &BoardState,
        king_pos: Position,
        side: Side,
        magic_mover: &MagicMover,
    ) -> Self {
        let mut path = CheckPath::None;
        let bitboards = board.side_bitboard(side);
        let enemies = bitboards.combined();

        if KNIGHT_MASKS.with(|m| {
            for i in m[*king_pos as usize].parts.iter().copied() {
                if bitboards.state[KNIGHT] & i != 0 {
                    update_checkpath!(path, i, true)
                }
            }
            false
        }) {
            return path;
        }
        let yo = match side {
            Side::White => |i| i << 8,
            Side::Black => |i| i >> 8,
        };

        if PAWN_TAKE_MASKS.with(|m| {
            for i in m[*king_pos as usize].parts.iter().copied() {
                if bitboards.state[KNIGHT] & yo(i) != 0 {
                    update_checkpath!(path, yo(i), true)
                }
            }
            false
        }) {
            return path;
        }
        let diagonal_attackers = bitboards.state[BISHOP] | bitboards.state[QUEEN];

        let diagonal_data = magic_mover.get_bishop(king_pos, diagonal_attackers);
        let diagonals = diagonal_data
            .possible_takes()
            .filter(|i| enemies & i.as_mask() != 0);
        for i in diagonals {
            update_checkpath!(path, {
                (magic_mover.get_bishop(i, king_pos.as_mask()).bitboard & diagonal_data.bitboard)
                    | i.as_mask()
            })
        }

        let parallel_attackers = bitboards.state[ROOK] | bitboards.state[QUEEN];
        let parallel_data = magic_mover.get_rook(king_pos, parallel_attackers);
        let parallels = parallel_data
            .possible_takes()
            .filter(|i| enemies & i.as_mask() != 0);
        for i in parallels {
            update_checkpath!(path, {
                (magic_mover.get_rook(i, king_pos.as_mask()).bitboard & parallel_data.bitboard)
                    | i.as_mask()
            })
        }
        path
    }
}
impl PinState {
    fn find(state: &mut BoardState, king_pos: Position) -> Self {
        MAGIC_MOVER.with(|m| PinState::find_with(state, king_pos, m))
    }
    fn find_with(state: &mut BoardState, king_pos: Position, magic_mover: &MagicMover) -> Self {
        let ally_bitboards = state.side_bitboard(state.side);
        let enemy_bitboards = state.side_bitboard(state.side.opposite());
        let king_pos: Position = king_pos;
        let king_mask = king_pos.as_mask();

        let friendlies = ally_bitboards.combined();
        let diagonal_attackers = enemy_bitboards.state[BISHOP] | enemy_bitboards.state[QUEEN];
        let parallel_attackers = enemy_bitboards.state[ROOK] | enemy_bitboards.state[QUEEN];

        let surrounding_friendlies = magic_mover.get_rook(king_pos, friendlies).bitboard
            | magic_mover.get_bishop(king_pos, friendlies).bitboard;

        let diagonal = {
            let cast = magic_mover.get_bishop(
                king_pos,
                friendlies & !(surrounding_friendlies) | diagonal_attackers,
            );
            let mut all_casts: u64 = 0;

            for i in cast.takes.iter().copied() {
                if friendlies & i.as_mask() == 0 {
                    all_casts |= magic_mover.get_bishop(i, king_mask).bitboard | i.as_mask();
                }
            }
            cast.bitboard & all_casts
        };
        let (x_aligned, y_aligned) = {
            let cast = magic_mover.get_rook(
                king_pos,
                friendlies & !(surrounding_friendlies) | parallel_attackers,
            );
            let mut x_casts: u64 = 0;
            let mut y_casts: u64 = 0;

            for i in cast.takes.iter().copied() {
                if friendlies & i.as_mask() == 0 {
                    if i.x() == king_pos.x() {
                        x_casts |= magic_mover.get_rook(i, king_mask).bitboard | i.as_mask();
                    } else {
                        y_casts |= magic_mover.get_rook(i, king_mask).bitboard | i.as_mask();
                    }
                }
            }
            (cast.bitboard & x_casts, cast.bitboard & y_casts)
        };

        PinState {
            diagonal,
            x_aligned,
            y_aligned,
        }
    }
}

impl Default for CheckPath {
    fn default() -> Self {
        Self::None
    }
}

pub struct AttackBoards {
    pub singular: u64,
    pub diagonal: u64,
    pub parallel: u64,
}
