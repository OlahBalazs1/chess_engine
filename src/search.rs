use rand::seq::IndexedRandom;

use crate::board::{Bitboards, BoardRepr, BoardState};
use crate::search_data::CheckPath;
use std::ops::{Deref, Index, Range};
use std::sync::Arc;
use std::{iter, pin, u64};

use crate::{
    board::SearchBoard,
    magic_bitboards::{MagicMover, MAGIC_MOVER},
    moving::{Move, MoveType},
    piece::{PieceType, Side},
    position::{Offset, Position},
};
use PieceType::*;

pub fn find_pawn(
    moves: &mut Vec<Move>,
    side: Side,
    pos: Position,
    allies: u64,
    enemies: u64,
    must_block: u64,
    all_square_data: &BoardRepr,
) {
    let yo = match side {
        Side::White => 1,
        Side::Black => -1,
    };
    let take_side = side.opposite();

    // Takes
    use PieceType::*;
    [Offset::new(-1, yo), Offset::new(1, yo)]
        .iter()
        .filter_map(|&off| pos.with_offset(off))
        .filter(|to| {
            enemies & to.as_mask() != 0
                && allies & to.as_mask() == 0
                && (must_block == 0 || must_block & to.as_mask() < must_block)
        })
        .for_each(|to| match (side, to.y()) {
            (Side::White, 7) | (Side::Black, 0) => {
                for promote_to in [Rook, Knight, Bishop, Queen] {
                    moves.push(Move::new(
                        pos,
                        to,
                        MoveType::Promotion(promote_to),
                        all_square_data
                            .get(pos)
                            .and_then(|i| i.filter_side(side).map(|i| i.role())),
                    ))
                }
            }
            _ => moves.push(Move::new(
                pos,
                to,
                MoveType::Normal(PieceType::Pawn),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
            )),
        });

    let forward = [Offset::new(0, yo), Offset::new(0, 2 * yo)];
    let valid_forward = match pos.y() {
        1 | 6 => &forward[..],
        _ => &forward[..1],
    }
    .iter()
    .filter_map(|&off| pos.with_offset(off));

    for to in valid_forward {
        if (allies & enemies) & to.as_mask() != 0 {
            break;
        }
        match (side, to.y()) {
            (Side::White, 7) | (Side::Black, 0) => {
                for promote_to in [Rook, Knight, Bishop, Queen] {
                    moves.push(Move::new(pos, to, MoveType::Promotion(promote_to), None))
                }
            }
            _ => moves.push(Move::new(pos, to, MoveType::Normal(PieceType::Pawn), None)),
        }
    }
}

pub fn find_knight(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_square_data: &BoardRepr,
    side: Side,
) {
    moves.extend(
        [
            Offset::new(-2, -1),
            Offset::new(-2, 1),
            Offset::new(-1, 2),
            Offset::new(1, 2),
            Offset::new(2, 1),
            Offset::new(2, -1),
            Offset::new(1, -2),
            Offset::new(-1, -2),
        ]
        .iter()
        .filter_map(|&off| pos.with_offset(off))
        .filter(|p| allies & p.as_mask() == 0)
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::Knight),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
            )
        }),
    )
}

pub fn find_king(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    attacked_squares: u64,
    enemies: u64,
    castle_rights: (bool, bool),
    all_square_data: &BoardRepr,
    side: Side,
) {
    let must_avoid = allies | attacked_squares;

    // normal moving
    moves.extend(
        [
            Offset::new(0, 1),
            Offset::new(0, -1),
            Offset::new(1, 0),
            Offset::new(-1, 0),
            Offset::new(1, 1),
            Offset::new(-1, -1),
            Offset::new(-1, 1),
            Offset::new(1, -1),
        ]
        .iter()
        .filter_map(|i| pos.with_offset(*i))
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::King),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
            )
        })
        .filter(|i| must_avoid & (i.from().as_mask() | i.to().as_mask()) == 0),
    );
    let (short, long) = match pos.y() {
        0 => (0b11 << 5, 0b11 << 1),
        7 => (0b11 << (5 + 7 * 8), 0b11 << (1 + 7 * 8)),
        _ if castle_rights.0 || castle_rights.1 => unreachable!(),
        _ => (0, 0),
    };

    if castle_rights.0 && long & must_avoid == 0 && long & enemies == 0 {
        moves.push(Move::new(
            pos,
            pos.with_x(2).unwrap(),
            MoveType::Normal(PieceType::King),
            None,
        ));
    }

    if castle_rights.1 && short & must_avoid == 0 && long & enemies == 0 {
        moves.push(Move::new(
            pos,
            pos.with_x(6).unwrap(),
            MoveType::Normal(PieceType::King),
            None,
        ));
    }
}

pub fn find_rook(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    find_rook_with_magic(
        moves,
        pos,
        allies,
        all_pieces,
        &*MAGIC_MOVER,
        all_square_data,
        pin_state,
        check_path,
        side,
    )
}

pub fn find_rook_with_magic(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    if pin_state == 0 {
        let data = magic_mover.get_rook(pos, all_pieces);
        let normals = data
            .normal
            .iter()
            .copied()
            .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None));

        let takes = data
            .takes
            .iter()
            .copied()
            .filter(|i| allies & i.as_mask() == 0)
            .map(|i| {
                Move::new_with_checkpath_parallel(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Rook),
                    all_square_data
                        .get(pos)
                        .and_then(|i| i.filter_side(side).map(|i| i.role())),
                    check_path,
                )
            });

        moves.extend(normals.chain(takes));
    } else {
        let data = magic_mover.get_rook(pos, all_pieces);
        let normals = data
            .normal
            .iter()
            .copied()
            .filter(|i| pin_state & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None));

        let takes = data
            .takes
            .iter()
            .copied()
            .filter(|i| allies & i.as_mask() == 0 && pin_state & i.as_mask() != 0)
            .map(|i| {
                Move::new_with_checkpath_parallel(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Rook),
                    all_square_data
                        .get(pos)
                        .and_then(|i| i.filter_side(side).map(|i| i.role())),
                    check_path,
                )
            });
        moves.extend(normals.chain(takes));
    }
}

pub fn find_bishop(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    find_bishop_with_magic(
        moves,
        pos,
        allies,
        all_pieces,
        &*MAGIC_MOVER,
        all_square_data,
        pin_state,
        check_path,
        side,
    )
}

pub fn find_bishop_with_magic(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    if pin_state == 0 {
        let data = magic_mover.get_bishop(pos, all_pieces);
        let normals = data
            .normal
            .iter()
            .copied()
            .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None));

        let takes = data
            .takes
            .iter()
            .copied()
            .filter(|i| allies & i.as_mask() == 0)
            .map(|i| {
                Move::new_with_checkpath_diagonal(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Rook),
                    all_square_data
                        .get(pos)
                        .and_then(|i| i.filter_side(side).map(|i| i.role())),
                    check_path,
                )
            });

        moves.extend(normals.chain(takes));
    } else {
        let data = magic_mover.get_bishop(pos, all_pieces);
        let normals = data
            .normal
            .iter()
            .copied()
            .filter(|i| pin_state & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None));

        let takes = data
            .takes
            .iter()
            .copied()
            .filter(|i| allies & i.as_mask() == 0 && pin_state & i.as_mask() != 0)
            .map(|i| {
                Move::new_with_checkpath_diagonal(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Rook),
                    all_square_data
                        .get(pos)
                        .and_then(|i| i.filter_side(side).map(|i| i.role())),
                    check_path,
                )
            });
        moves.extend(normals.chain(takes));
    }
}

pub fn find_queen(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    find_queen_with_magic(
        moves,
        pos,
        allies,
        all_pieces,
        &*MAGIC_MOVER,
        all_square_data,
        pin_state,
        check_path,
        side,
    )
}

pub fn find_queen_with_magic(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    if pin_state == 0 {
        queen_unpinned(
            moves,
            pos,
            allies,
            all_pieces,
            magic_mover,
            all_square_data,
            check_path,
            side,
        );
    } else {
        queen_pinned(
            moves,
            pos,
            allies,
            all_pieces,
            magic_mover,
            all_square_data,
            pin_state,
            check_path,
            side,
        );
    }
}

fn queen_unpinned(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    check_path: &mut CheckPath,
    side: Side,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    let normals = rook_data
        .normal
        .iter()
        .copied()
        .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None));
    let takes = rook_data
        .takes
        .iter()
        .copied()
        .filter(|i| allies & i.as_mask() == 0)
        .map(|i| {
            Move::new_with_checkpath_parallel(
                pos,
                i,
                MoveType::Normal(PieceType::Queen),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
                check_path,
            )
        });
    moves.extend(normals.chain(takes));

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);
    let normals = bishop_data
        .normal
        .iter()
        .copied()
        .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None));

    let takes = bishop_data
        .takes
        .iter()
        .copied()
        .filter(|i| allies & i.as_mask() == 0)
        .map(|i| {
            Move::new_with_checkpath_diagonal(
                pos,
                i,
                MoveType::Normal(PieceType::Queen),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
                check_path,
            )
        });
    moves.extend(normals.chain(takes));
}

fn queen_pinned(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    pin_state: u64,
    check_path: &mut CheckPath,
    side: Side,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    let normals = rook_data
        .normal
        .iter()
        .copied()
        .filter(|i| pin_state & i.as_mask() != 0)
        .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None));
    let takes = rook_data
        .takes
        .iter()
        .copied()
        .filter(|i| allies & i.as_mask() == 0 && pin_state & i.as_mask() != 0)
        .map(|i| {
            Move::new_with_checkpath_parallel(
                pos,
                i,
                MoveType::Normal(PieceType::Queen),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
                check_path,
            )
        });

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);
    let normals = normals.chain(
        bishop_data
            .normal
            .iter()
            .copied()
            .filter(|i| pin_state & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );

    let takes = takes.chain(
        bishop_data
            .takes
            .iter()
            .copied()
            .filter(|i| allies & i.as_mask() == 0 && pin_state & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );
    moves.extend(normals.chain(takes));
}
