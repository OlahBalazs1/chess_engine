use std::pin::Pin;

use crate::{
    board::{SearchBoard, BISHOP, QUEEN, ROOK},
    magic_bitboards::{MagicData, MAGIC_MOVER},
    position::Position,
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

impl CheckPath {
    pub fn set_singular(&mut self, attacker_pos: Position) {
        match self {
            Self::None => *self = Self::Blockable(attacker_pos.as_mask()),
            Self::Blockable(_) => *self = Self::Multiple,
            Self::Multiple => {}
        }
    }

    pub fn set_diagonal(&mut self, king_pos: Position, attacker_pos: Position) {
        match self {
            Self::None => {
                *self = Self::Blockable(
                    MAGIC_MOVER
                        .get_bishop(king_pos, attacker_pos.as_mask())
                        .bitboard
                        & MAGIC_MOVER
                            .get_bishop(attacker_pos, king_pos.as_mask())
                            .bitboard
                        | attacker_pos.as_mask(),
                )
            }
            Self::Blockable(_) => *self = Self::Multiple,
            Self::Multiple => {}
        }
    }
    pub fn set_parallel(&mut self, king_pos: Position, attacker_pos: Position) {
        match self {
            Self::None => {
                *self = Self::Blockable(
                    MAGIC_MOVER
                        .get_rook(king_pos, attacker_pos.as_mask())
                        .bitboard
                        & MAGIC_MOVER
                            .get_rook(attacker_pos, king_pos.as_mask())
                            .bitboard
                        | attacker_pos.as_mask(),
                )
            }
            Self::Blockable(_) => *self = Self::Multiple,
            Self::Multiple => {}
        }
    }
}

impl Default for CheckPath {
    fn default() -> Self {
        Self::None
    }
}

pub fn set_pin(state: &mut SearchBoard) {
    let ally_bitboards = state.state.side_bitboard(state.state.side);
    let enemy_bitboards = state.state.side_bitboard(state.state.side.opposite());
    let king_pos: Position = state.find_king();
    let king_mask = king_pos.as_mask();

    let friendlies = ally_bitboards.combined();
    let diagonal_attackers = enemy_bitboards.state[BISHOP] | enemy_bitboards.state[QUEEN];
    let parallel_attackers = enemy_bitboards.state[ROOK] | enemy_bitboards.state[QUEEN];

    let surrounding_friendlies = MAGIC_MOVER.get_rook(king_pos, friendlies).bitboard
        | MAGIC_MOVER.get_bishop(king_pos, friendlies).bitboard;

    let diagonal = {
        let cast = MAGIC_MOVER.get_bishop(
            king_pos,
            friendlies & !(surrounding_friendlies) | diagonal_attackers,
        );
        let mut all_casts: u64 = 0;

        for i in cast.takes {
            if friendlies & i.as_mask() == 0 {
                all_casts |= MAGIC_MOVER.get_bishop(i, king_mask).bitboard | i.as_mask();
            }
        }
        cast.bitboard & all_casts
    };
    let (x_aligned, y_aligned) = {
        let cast = MAGIC_MOVER.get_rook(
            king_pos,
            friendlies & !(surrounding_friendlies) | diagonal_attackers,
        );
        let mut all_casts: u64 = 0;
        let mut x_casts: u64 = 0;
        let mut y_casts: u64 = 0;

        for i in cast.takes {
            if friendlies & i.as_mask() == 0 {
                if i.x() == king_pos.x() {
                    x_casts |= MAGIC_MOVER.get_rook(i, king_mask).bitboard | i.as_mask();
                } else {
                    y_casts |= MAGIC_MOVER.get_rook(i, king_mask).bitboard | i.as_mask();
                }
            }
        }
        (cast.bitboard & x_casts, cast.bitboard & y_casts)
    };

    state.pin_state = PinState {
        diagonal,
        x_aligned,
        y_aligned,
    }
}

pub struct AttackBoards {
    pub singular: u64,
    pub diagonal: u64,
    pub parallel: u64,
}
