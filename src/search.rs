use crate::board_repr::BoardRepr;
use crate::piece::Piece;
use crate::search_data::{CheckPath, PinState};
use crate::search_masks::{KING_MASKS, KNIGHT_MASKS, choose_pawn_take_mask};
use std::u64;

use crate::{
    board::SearchBoard,
    magic_bitboards::{MAGIC_MOVER, MagicMover},
    moving::{Move, MoveType},
    piece::{PieceType, Side},
    position::Position,
};
use PieceType::*;
use arrayvec::ArrayVec;

pub fn find_pawn<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    gen_only_takes: bool,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_square_data = &state.state.board;
    let can_ep = pin_state.can_en_passant;

    let pin_state = pin_state.choose_relevant(pos);
    let check_path = match check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => *i,
        CheckPath::Multiple => return (),
    };

    if pin_state != 0 && check_path != 0 {
        return;
    }
    let must_block = pin_state | check_path;

    if must_block == 0 {
        find_pawn_unrestricted(
            moves,
            pos,
            side,
            allies,
            enemies,
            all_square_data,
            state.state.en_passant_square,
            can_ep,
            gen_only_takes,
        );
    } else {
        find_pawn_restricted(
            moves,
            pos,
            side,
            allies,
            enemies,
            all_square_data,
            must_block,
            state.state.en_passant_square,
            can_ep,
            gen_only_takes,
        );
    }
}
fn find_pawn_unrestricted<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    side: Side,
    allies: u64,
    enemies: u64,
    all_square_data: &BoardRepr,
    ep_square: Option<Position>,
    can_ep: bool,
    gen_only_takes: bool,
) {
    // takes
    // let yo = match side {
    //     Side::White => 1,
    //     Side::Black => -1,
    // };
    let data = &choose_pawn_take_mask(side)[*pos as usize];

    for i in data.positions.iter() {
        if enemies & i.as_mask() != 0 {
            gen_pawn_moves(moves, pos, *i, all_square_data.get(*i));
        } else if can_ep {
            // SAFETY: if ep_square is None, can_ep is false
            unsafe {
                if ep_square.unwrap_unchecked() == *i {
                    moves.push(Move::new(pos, *i, MoveType::EnPassant, None));
                }
            }
        }
    }
    if !gen_only_takes {
        let all_pieces = allies | enemies;
        match (*pos, side) {
            // white starting
            // SAFETY: add_y() and sub_y() only fail if the pawn steps off the board
            // which only happens if a pawn is in an illegal position
            (8..16, Side::White) => unsafe {
                let mut to = pos.add_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    gen_pawn_moves(moves, pos, to, None);
                } else {
                    return;
                }
                to = to.add_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            // black starting
            (48..56, Side::Black) if side == Side::Black => unsafe {
                let mut to = pos.sub_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    gen_pawn_moves(moves, pos, to, None);
                } else {
                    return;
                }
                to = to.sub_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            // a3 - g6
            (16..56, Side::White) => unsafe {
                let to = pos.add_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            // a3 - g6
            (8..48, Side::Black) => unsafe {
                let to = pos.sub_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            (i, _) => panic!(
                "Pawn in illegal position: {}: {}",
                Position::from_index(i),
                i
            ),
        }
    }
}

fn find_pawn_restricted<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    side: Side,
    allies: u64,
    enemies: u64,
    all_square_data: &BoardRepr,
    must_block: u64,
    ep_square: Option<Position>,
    can_ep: bool,
    gen_only_takes: bool,
) {
    let data = &choose_pawn_take_mask(side)[*pos as usize];

    for i in data.positions.iter().copied() {
        if must_block & i.as_mask() != 0 {
            if enemies & i.as_mask() != 0 {
                gen_pawn_moves(moves, pos, i, all_square_data.get(i));
            } else if can_ep && ep_square.is_some_and(|ep| ep == i) {
                moves.push(Move::new(pos, i, MoveType::EnPassant, None));
            }
        }
    }

    if !gen_only_takes {
        let all_pieces = allies | enemies;

        match (*pos, side) {
            // white starting
            // SAFETY: add_x() and sub_x() only fail if the pawn steps off the board
            // which only happens if a pawn is in an illegal position
            (8..16, Side::White) => unsafe {
                let mut to = pos.add_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    if must_block & to.as_mask() != 0 {
                        gen_pawn_moves(moves, pos, to, None);
                    }
                } else {
                    return;
                }
                to = to.add_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 && must_block & to.as_mask() != 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            // black starting
            (48..56, Side::Black) if side == Side::Black => unsafe {
                let mut to = pos.sub_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 {
                    if must_block & to.as_mask() != 0 {
                        gen_pawn_moves(moves, pos, to, None);
                    }
                } else {
                    return;
                }
                to = to.sub_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 && must_block & to.as_mask() != 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            // a3 - g6
            (16..56, Side::White) => unsafe {
                let to = pos.add_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 && must_block & to.as_mask() != 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            // a3 - g6
            (8..48, Side::Black) => unsafe {
                let to = pos.sub_y(1).unwrap_unchecked();
                if all_pieces & to.as_mask() == 0 && must_block & to.as_mask() != 0 {
                    gen_pawn_moves(moves, pos, to, None);
                }
            },
            (i, _) => panic!(
                "{:?} Pawn in illegal position: {}: {}",
                side,
                Position::from_index(i),
                i
            ),
        }
    }
}

fn gen_pawn_moves<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    from: Position,
    to: Position,
    take: Option<Piece>,
) {
    use PieceType::*;
    match to.y() {
        0 | 7 => {
            for i in [Queen, Rook, Knight, Bishop] {
                moves.push(Move::new(from, to, MoveType::Promotion(i), take));
            }
        }
        _ => moves.push(Move::new(from, to, MoveType::Normal(Pawn), take)),
    }
}

pub fn find_knight<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    gen_only_takes: bool,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_square_data = &state.state.board;

    if pin_state.choose_relevant(pos) != 0 {
        return;
    }

    match check_paths {
        CheckPath::None => moves.extend({
            KNIGHT_MASKS[*pos as usize]
                .positions
                .iter()
                .copied()
                .filter(|p| {
                    allies & p.as_mask() == 0 && (!gen_only_takes || enemies & p.as_mask() != 0)
                })
                .map(|i| {
                    Move::new(
                        pos,
                        i,
                        MoveType::Normal(PieceType::Knight),
                        all_square_data.get(i),
                    )
                })
        }),

        CheckPath::Blockable(must_block) => moves.extend({
            KNIGHT_MASKS[*pos as usize]
                .positions
                .iter()
                .copied()
                .filter(|p| {
                    allies & p.as_mask() == 0
                        && must_block & p.as_mask() != 0
                        && (!gen_only_takes || enemies & p.as_mask() != 0)
                })
                .map(|i| {
                    Move::new(
                        pos,
                        i,
                        MoveType::Normal(PieceType::Knight),
                        all_square_data.get(i),
                    )
                })
        }),

        CheckPath::Multiple => return,
    }
}

pub fn find_king<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    check_paths: &CheckPath,
    attacked_squares: u64,
    gen_only_takes: bool,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_square_data = &state.state.board;
    let must_avoid = allies | attacked_squares;

    moves.extend(
        KING_MASKS[*pos as usize]
            .positions
            .iter()
            .copied()
            .filter(|i| must_avoid & i.as_mask() == 0)
            .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::King),
                    all_square_data.get(i),
                )
            }),
    );

    // case
    if let CheckPath::None = check_paths
        && !gen_only_takes
    {
        let castle_rights = state.side_castle_rights(side);
        let short = 0x60 << (side.home_y() * 8);
        let long_occupy = 0xe << (side.home_y() * 8);
        let long_attack = 0xc << (side.home_y() * 8);

        if castle_rights.0
            && long_occupy & (allies | enemies) == 0
            && long_attack & attacked_squares == 0
        {
            moves.push(Move::new(
                pos,
                pos.with_x(2).unwrap(),
                MoveType::LongCastle,
                None,
            ));
        }

        if castle_rights.1 && short & (allies | enemies | attacked_squares) == 0 {
            moves.push(Move::new(
                pos,
                pos.with_x(6).unwrap(),
                MoveType::ShortCastle,
                None,
            ));
        }
    }
}

pub fn find_rook<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    gen_only_takes: bool,
) {
    find_rook_with_magic(
        moves,
        pos,
        state,
        pin_state,
        check_paths,
        &*MAGIC_MOVER,
        gen_only_takes,
    )
}

pub fn find_rook_with_magic<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    magic_mover: &MagicMover,
    gen_only_takes: bool,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_pieces = state.side_bitboards(side.opposite()).combined() | allies;
    let all_square_data = &state.state.board;
    let pin_state = pin_state.choose_relevant(pos);
    let check_path = match check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => *i,
        CheckPath::Multiple => return (),
    };
    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };
    let data = magic_mover.get_rook(pos, all_pieces);

    if must_block == 0 {
        if !gen_only_takes {
            moves.extend(
                data.normal
                    .iter()
                    .copied()
                    .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None)),
            );
        }

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0)
                .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
                .map(|i| {
                    Move::new(
                        pos,
                        i,
                        MoveType::Normal(PieceType::Rook),
                        all_square_data.get(i),
                    )
                }),
        );
    } else {
        if !gen_only_takes {
            moves.extend(
                data.normal
                    .iter()
                    .copied()
                    .filter(|i| must_block & i.as_mask() != 0)
                    .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None)),
            );
        }

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
                .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
                .map(|i| {
                    Move::new(
                        pos,
                        i,
                        MoveType::Normal(PieceType::Rook),
                        all_square_data.get(i),
                    )
                }),
        );
    }
}

pub fn find_bishop<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,

    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    gen_only_takes: bool,
) {
    find_bishop_with_magic(
        moves,
        pos,
        state,
        pin_state,
        check_paths,
        &*MAGIC_MOVER,
        gen_only_takes,
    )
}

pub fn find_bishop_with_magic<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    magic_mover: &MagicMover,
    gen_only_takes: bool,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_pieces = state.side_bitboards(side.opposite()).combined() | allies;
    let all_square_data = &state.state.board;
    let pin_state = pin_state.choose_relevant(pos);
    let check_path = match check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => *i,
        CheckPath::Multiple => return (),
    };

    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };
    let data = magic_mover.get_bishop(pos, all_pieces);

    if must_block == 0 {
        if !gen_only_takes {
            moves.extend(
                data.normal
                    .iter()
                    .copied()
                    .map(|i| Move::new(pos, i, MoveType::Normal(Bishop), None)),
            );
        }

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0)
                .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
                .map(|i| {
                    Move::new(
                        pos,
                        i,
                        MoveType::Normal(PieceType::Bishop),
                        all_square_data.get(i),
                    )
                }),
        );

        // moves.extend(normals);
    } else {
        if !gen_only_takes {
            moves.extend(
                data.normal
                    .iter()
                    .copied()
                    .filter(|i| must_block & i.as_mask() != 0)
                    .map(|i| Move::new(pos, i, MoveType::Normal(Bishop), None)),
            );
        }

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
                .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
                .map(|i| {
                    Move::new(
                        pos,
                        i,
                        MoveType::Normal(PieceType::Bishop),
                        all_square_data.get(i),
                    )
                }),
        );
    }
}

pub fn find_queen<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    gen_only_takes: bool,
) {
    find_queen_with_magic(
        moves,
        pos,
        state,
        pin_state,
        check_paths,
        &*MAGIC_MOVER,
        gen_only_takes,
    )
}

pub fn find_queen_with_magic<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    magic_mover: &MagicMover,
    gen_only_takes: bool,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_pieces = state.side_bitboards(side.opposite()).combined() | allies;
    let all_square_data = &state.state.board;
    let parsed_pin = pin_state.choose_relevant(pos);
    let check_path = match check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => *i,
        CheckPath::Multiple => return (),
    };
    let must_block = match (parsed_pin, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => parsed_pin,
        _ => return,
    };
    if must_block == 0 {
        queen_unrestricted(
            moves,
            pos,
            allies,
            enemies,
            all_pieces,
            magic_mover,
            all_square_data,
            gen_only_takes,
        );
    } else {
        queen_restricted(
            moves,
            pos,
            allies,
            enemies,
            all_pieces,
            magic_mover,
            all_square_data,
            must_block,
            gen_only_takes,
        );
    }
}

fn queen_unrestricted<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    allies: u64,
    enemies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    gen_only_takes: bool,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    if !gen_only_takes {
        moves.extend(
            rook_data
                .normal
                .iter()
                .copied()
                .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
        );
    }
    moves.extend(
        rook_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0)
            .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Queen),
                    all_square_data.get(i),
                )
            }),
    );

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);
    if !gen_only_takes {
        moves.extend(
            bishop_data
                .normal
                .iter()
                .copied()
                .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
        );
    }

    moves.extend(
        bishop_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0)
            .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Queen),
                    all_square_data.get(i),
                )
            }),
    );
}

fn queen_restricted<const CAP: usize>(
    moves: &mut ArrayVec<Move, CAP>,
    pos: Position,
    allies: u64,
    enemies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    must_block: u64,
    gen_only_takes: bool,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    if !gen_only_takes {
        moves.extend(
            rook_data
                .normal
                .iter()
                .copied()
                .filter(|i| must_block & i.as_mask() != 0)
                .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
        );
    }
    moves.extend(
        rook_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
            .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Queen),
                    all_square_data.get(i),
                )
            }),
    );

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);

    if !gen_only_takes {
        moves.extend(
            bishop_data
                .normal
                .iter()
                .copied()
                .filter(|i| must_block & i.as_mask() != 0)
                .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
        );
    }

    moves.extend(
        bishop_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
            .filter(|i| !gen_only_takes || enemies & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Queen),
                    all_square_data.get(i),
                )
            }),
    );
}
