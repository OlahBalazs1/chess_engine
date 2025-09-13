use std::pin::Pin;

use crate::{
    board::{BoardState, SearchBoard},
    board_repr::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK},
    magic_bitboards::{MAGIC_MOVER, MagicData, MagicMover, print_bits},
    piece::Side,
    position::Position,
    search_masks::{KNIGHT_MASKS, choose_pawn_take_mask},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PinState {
    pub diagonal_1: u64,
    pub diagonal_2: u64,
    pub x_aligned: u64,
    pub y_aligned: u64,

    pub can_en_passant: bool,
}

impl PinState {
    pub fn choose_relevant(&self, pos: Position) -> u64 {
        let mask = pos.as_mask();
        if self.diagonal_1 & mask != 0 {
            return self.diagonal_1;
        } else if self.diagonal_2 & mask != 0 {
            return self.diagonal_2;
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
            diagonal_1: 0,
            diagonal_2: 0,
            x_aligned: 0,
            y_aligned: 0,
            can_en_passant: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    pub fn is_check(&self) -> bool {
        match self {
            CheckPath::None => false,
            _ => true,
        }
    }
    pub fn find(board: &BoardState, king_pos: Position, side: Side) -> Self {
        CheckPath::find_with(board, king_pos, side, &*MAGIC_MOVER)
    }

    fn find_with(
        board: &BoardState,
        king_pos: Position,
        side: Side,
        magic_mover: &MagicMover,
    ) -> Self {
        let mut path = CheckPath::None;
        let bitboards = board.side_bitboards(side);
        let allies = board.side_bitboards(side.opposite()).combined();
        let enemies = bitboards.combined();

        for i in KNIGHT_MASKS[*king_pos as usize].parts.iter().copied() {
            if bitboards.state[KNIGHT] & i != 0 {
                update_checkpath!(path, i)
            }
        }

        for i in choose_pawn_take_mask(side.opposite())[*king_pos as usize]
            .parts
            .iter()
            .copied()
        {
            if bitboards.state[PAWN] & i != 0 {
                update_checkpath!(path, i)
            }
        }
        let diagonal_attackers = bitboards.state[BISHOP] | bitboards.state[QUEEN];

        let diagonal_data = magic_mover.get_bishop(king_pos, allies | enemies | diagonal_attackers);
        let diagonals = diagonal_data
            .possible_takes()
            .filter(|i| diagonal_attackers & i.as_mask() != 0);
        for i in diagonals {
            update_checkpath!(path, {
                (magic_mover.get_bishop(i, king_pos.as_mask()).bitboard & diagonal_data.bitboard)
                    | i.as_mask()
            })
        }

        let parallel_attackers = bitboards.state[ROOK] | bitboards.state[QUEEN];
        let parallel_data = magic_mover.get_rook(king_pos, allies | enemies | parallel_attackers);
        let parallels = parallel_data
            .possible_takes()
            .filter(|i| parallel_attackers & i.as_mask() != 0);
        for i in parallels {
            update_checkpath!(path, {
                (magic_mover.get_rook(i, king_pos.as_mask()).bitboard & parallel_data.bitboard)
                    | i.as_mask()
            })
        }
        // match path {
        //     CheckPath::Blockable(i) => print_bits(i),
        //     _ => {}
        // }
        path
    }
}
impl PinState {
    pub fn find(state: &BoardState, king_pos: Position) -> Self {
        PinState::find_with(state, king_pos, &*MAGIC_MOVER)
    }

    pub fn combined(&self) -> u64 {
        self.diagonal_1 | self.diagonal_2 | self.x_aligned | self.y_aligned
    }
    fn find_with(state: &BoardState, king_pos: Position, magic_mover: &MagicMover) -> Self {
        let ally_bitboards = state.side_bitboards(state.side);
        let enemy_bitboards = state.side_bitboards(state.side.opposite());
        let king_mask = king_pos.as_mask();

        let friendlies = ally_bitboards.combined();
        let enemies = enemy_bitboards.combined();

        let first_pass = magic_mover
            .get_rook(king_pos, friendlies | enemies)
            .bitboard
            | magic_mover
                .get_bishop(king_pos, friendlies | enemies)
                .bitboard;

        let (diagonal_1, diagonal_2) = {
            let diagonal_attackers = enemy_bitboards.state[BISHOP] | enemy_bitboards.state[QUEEN];
            let targets = (friendlies & !(first_pass)) | enemy_bitboards.combined();
            let cast = magic_mover.get_bishop(king_pos, targets);
            let mut casts_1: u64 = 0;
            let mut casts_2: u64 = 0;

            for i in cast.possible_takes() {
                // if let Some(ep) = state.en_passant_square.filter(|_| can_en_passant) {
                //     if ep == i {
                //         let ep_cast = magic_mover.get_bishop(king_pos, targets & !(i.as_mask()));
                //         for i in ep_cast.possible_takes() {
                //             if diagonal_attackers & i.as_mask() != 0 {
                //                 if magic_mover.get_bishop(i, king_mask).bitboard
                //                     & ep_cast.bitboard
                //                     & ep.as_mask()
                //                     != 0
                //                 {
                //                     can_en_passant = false
                //                 }
                //             }
                //         }
                //     }
                // }
                if diagonal_attackers & i.as_mask() != 0 {
                    if (i.x() < king_pos.x() && i.y() < king_pos.y())
                        || (i.x() > king_pos.x() && i.y() > king_pos.y())
                    {
                        casts_1 |= (magic_mover.get_bishop(i, king_mask).bitboard & cast.bitboard)
                            | i.as_mask();
                    } else {
                        casts_2 |= (magic_mover.get_bishop(i, king_mask).bitboard & cast.bitboard)
                            | i.as_mask();
                    }
                }
            }
            // print_bits(casts_2);
            (cast.bitboard & casts_1, casts_2)
        };
        let (x_aligned, y_aligned) = {
            let parallel_attackers = enemy_bitboards.state[ROOK] | enemy_bitboards.state[QUEEN];
            let targets = friendlies & !(first_pass) | enemies;
            let cast = magic_mover.get_rook(king_pos, targets);
            let mut x_casts: u64 = 0;
            let mut y_casts: u64 = 0;

            for i in cast.possible_takes() {
                // if let Some(ep) = state.en_passant_square.filter(|_| can_en_passant) {
                //     if ep == i {
                //         let ep_cast = magic_mover.get_rook(king_pos, targets & !(i.as_mask()));
                //         for i in ep_cast.possible_takes() {
                //             if parallel_attackers & i.as_mask() != 0 {
                //                 if magic_mover.get_bishop(i, king_mask).bitboard
                //                     & ep_cast.bitboard
                //                     & ep.as_mask()
                //                     != 0
                //                 {
                //                     can_en_passant = false
                //                 }
                //             }
                //         }
                //     }
                // }
                if parallel_attackers & i.as_mask() != 0 {
                    if i.x() == king_pos.x() {
                        x_casts |= (magic_mover.get_rook(i, king_mask).bitboard & cast.bitboard)
                            | i.as_mask();
                    } else {
                        y_casts |= (magic_mover.get_rook(i, king_mask).bitboard & cast.bitboard)
                            | i.as_mask();
                    }
                }
            }
            (cast.bitboard & x_casts, cast.bitboard & y_casts)
        };

        PinState {
            diagonal_1,
            diagonal_2,
            x_aligned,
            y_aligned,
            can_en_passant: true,
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
