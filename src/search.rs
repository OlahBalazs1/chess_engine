use rand::seq::IndexedRandom;

use crate::board::{Bitboards, BoardRepr, BoardState};
use crate::piece::Piece;
use crate::search_data::CheckPath;
use crate::search_masks::{SingularData, KING_MASKS, KNIGHT_MASKS, PAWN_TAKE_MASKS};
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

pub fn find_pawn(moves: &mut Vec<Move>, attack_bits: &mut u64, pos: Position, state: &SearchBoard) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let all_square_data = &state.state.board;
    let pin_state = state.pin_state.choose_relevant(pos);
    let check_path = match state.check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => i,
        CheckPath::Multiple => return (),
    };
    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };

    if must_block == 0 {
        find_pawn_unrestricted(
            moves,
            attack_bits,
            pos,
            side,
            allies,
            enemies,
            all_square_data,
            state.state.en_passant_square,
        );
    } else {
        find_pawn_restricted(
            moves,
            attack_bits,
            pos,
            side,
            allies,
            enemies,
            all_square_data,
            must_block,
            state.state.en_passant_square,
        );
    }
}
fn find_pawn_unrestricted(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    side: Side,
    allies: u64,
    enemies: u64,
    all_square_data: &BoardRepr,
    ep_square: Option<Position>,
) {
    // takes
    let yo = match side {
        Side::White => 1,
        Side::Black => -1,
    };
    if let Some(can_take) = pos.with_offset(Offset::new(0, yo)) {
        let data = &PAWN_TAKE_MASKS[*can_take as usize];
        *attack_bits |= data.sum;
        for i in data.positions.iter() {
            if enemies & i.as_mask() != 0 {
                gen_pawn_moves(moves, pos, *i, all_square_data.get(*i));
            } else if ep_square.is_some_and(|ep| ep == *i) {
                moves.push(Move::new(pos, *i, MoveType::EnPassant, None));
            }
        }
    }

    let mut moves_iter = [Offset::new(0, yo), Offset::new(0, yo * 2)]
        .into_iter()
        .filter_map(|i| pos.with_offset(i));

    let Some(next) = moves_iter.next() else {
        return;
    };
    if (allies | enemies) & next.as_mask() == 0 {
        gen_pawn_moves(moves, pos, next, None);
    } else {
        return;
    }

    if matches!(pos.y(), 1 | 6) {
        let Some(next) = moves_iter.next() else {
            return;
        };
        if (allies | enemies) & next.as_mask() == 0 {
            gen_pawn_moves(moves, pos, next, None);
        }
    }
}

fn find_pawn_restricted(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    side: Side,
    allies: u64,
    enemies: u64,
    all_square_data: &BoardRepr,
    must_block: u64,
    ep_square: Option<Position>,
) {
    // takes
    let yo = match side {
        Side::White => 1,
        Side::Black => -1,
    };

    if let Some(can_take) = pos.with_offset(Offset::new(0, yo)) {
        let data = &PAWN_TAKE_MASKS[*can_take as usize];
        *attack_bits |= data.sum;
        for i in data.positions.iter() {
            if must_block & i.as_mask() != 0 && enemies & i.as_mask() != 0 {
                gen_pawn_moves(moves, pos, *i, all_square_data.get(*i));
            } else if ep_square.is_some_and(|ep| ep == *i) {
                moves.push(Move::new(pos, *i, MoveType::EnPassant, None));
            }
        }
    }

    let mut moves_iter = [Offset::new(0, yo), Offset::new(0, yo * 2)]
        .into_iter()
        .filter_map(|i| pos.with_offset(i));

    let Some(next) = moves_iter.next() else {
        return;
    };
    if (allies | enemies) & next.as_mask() == 0 {
        if must_block & next.as_mask() != 0 {
            gen_pawn_moves(moves, pos, next, None);
        }
    } else {
        return;
    }

    if matches!(pos.y(), 1 | 6) {
        let Some(next) = moves_iter.next() else {
            return;
        };
        if (allies | enemies) & next.as_mask() == 0 {
            if must_block & next.as_mask() != 0 {
                gen_pawn_moves(moves, pos, next, None);
            }
        }
    }
}

fn gen_pawn_moves(moves: &mut Vec<Move>, from: Position, to: Position, take: Option<Piece>) {
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

pub fn find_knight(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    state: &SearchBoard,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let all_square_data = &state.state.board;

    if state.pin_state.choose_relevant(pos) != 0 {
        return;
    }

    match state.check_paths {
        CheckPath::None => {
            *attack_bits |= KNIGHT_MASKS[*pos as usize].sum;
            moves.extend({
                KNIGHT_MASKS[*pos as usize]
                    .positions
                    .iter()
                    .copied()
                    .filter(|p| allies & p.as_mask() == 0)
                    .map(|i| {
                        Move::new(
                            pos,
                            i,
                            MoveType::Normal(PieceType::Knight),
                            all_square_data.get(i),
                        )
                    })
            })
        }

        CheckPath::Blockable(must_block) => {
            *attack_bits |= KNIGHT_MASKS[*pos as usize].sum;
            moves.extend({
                KNIGHT_MASKS[*pos as usize]
                    .positions
                    .iter()
                    .copied()
                    .filter(|p| allies & p.as_mask() == 0 && must_block & p.as_mask() != 0)
                    .map(|i| {
                        Move::new(
                            pos,
                            i,
                            MoveType::Normal(PieceType::Knight),
                            all_square_data.get(i),
                        )
                    })
            })
        }

        CheckPath::Multiple => return,
    }
}

pub fn find_king(moves: &mut Vec<Move>, attack_bits: &mut u64, pos: Position, state: &SearchBoard) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let attacked_squares = state.attacked;
    let enemies = state.side_bitboards(side.opposite()).combined();
    let castle_rights = state.state.side_castle_rights(side);
    let all_square_data = &state.state.board;
    let must_avoid = allies | attacked_squares;

    // normal moving
    *attack_bits |= KING_MASKS[*pos as usize].sum;
    moves.extend(
        KING_MASKS[*pos as usize]
            .positions
            .iter()
            .copied()
            .filter(|i| must_avoid & i.as_mask() == 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::King),
                    all_square_data.get(i),
                )
            }),
    );

    match state.check_paths {
        CheckPath::None => {
            let short = 0b110 << side.home_y();
            let long = 0x60 << side.home_y();

            if castle_rights.0 && long & must_avoid == 0 && long & enemies == 0 {
                moves.push(Move::new(
                    pos,
                    pos.with_x(2).unwrap(),
                    MoveType::LongCastle,
                    None,
                ));
            }

            if castle_rights.1 && short & must_avoid == 0 && long & enemies == 0 {
                moves.push(Move::new(
                    pos,
                    pos.with_x(6).unwrap(),
                    MoveType::ShortCastle,
                    None,
                ));
            }
        }
        _ => {}
    }
}

pub fn find_rook(moves: &mut Vec<Move>, attack_bits: &mut u64, pos: Position, state: &SearchBoard) {
    find_rook_with_magic(moves, attack_bits, pos, state, &*MAGIC_MOVER)
}

pub fn find_rook_with_magic(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    state: &SearchBoard,
    magic_mover: &MagicMover,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let all_pieces = state.side_bitboards(side.opposite()).combined() | allies;
    let all_square_data = &state.state.board;
    let pin_state = state.pin_state.choose_relevant(pos);
    let check_path = match state.check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => i,
        CheckPath::Multiple => return (),
    };
    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };
    let data = magic_mover.get_rook(pos, all_pieces);
    *attack_bits |= data.bitboard;

    if must_block == 0 {
        let normals = data
            .normal
            .iter()
            .copied()
            .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None));

        let takes = data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Rook),
                    all_square_data.get(i),
                )
            });

        moves.extend(normals.chain(takes));
    } else {
        let normals = data
            .normal
            .iter()
            .copied()
            .filter(|i| must_block & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None));

        let takes = data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Rook),
                    all_square_data.get(i),
                )
            });
        moves.extend(normals.chain(takes));
    }
}

pub fn find_bishop(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    state: &SearchBoard,
) {
    find_bishop_with_magic(moves, attack_bits, pos, state, &*MAGIC_MOVER)
}

pub fn find_bishop_with_magic(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    state: &SearchBoard,
    magic_mover: &MagicMover,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let all_pieces = state.side_bitboards(side.opposite()).combined() | allies;
    let all_square_data = &state.state.board;
    let pin_state = state.pin_state.choose_relevant(pos);
    let check_path = match state.check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => i,
        CheckPath::Multiple => return (),
    };

    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };
    let data = magic_mover.get_bishop(pos, all_pieces);
    *attack_bits |= data.bitboard;

    if must_block == 0 {
        let normals = data
            .normal
            .iter()
            .copied()
            .map(|i| Move::new(pos, i, MoveType::Normal(Bishop), None));

        let takes = data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Bishop),
                    all_square_data.get(i),
                )
            });

        moves.extend(normals.chain(takes));
        // moves.extend(normals);
    } else {
        let normals = data
            .normal
            .iter()
            .copied()
            .filter(|i| must_block & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(Bishop), None));

        let takes = data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Bishop),
                    all_square_data.get(i),
                )
            });
        moves.extend(normals.chain(takes));
    }
}

pub fn find_queen(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    state: &SearchBoard,
) {
    find_queen_with_magic(moves, attack_bits, pos, state, &*MAGIC_MOVER)
}

pub fn find_queen_with_magic(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    state: &SearchBoard,
    magic_mover: &MagicMover,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let all_pieces = state.side_bitboards(side.opposite()).combined() | allies;
    let all_square_data = &state.state.board;
    let pin_state = state.pin_state.choose_relevant(pos);
    let check_path = match state.check_paths {
        CheckPath::None => 0,
        CheckPath::Blockable(i) => i,
        CheckPath::Multiple => return (),
    };
    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };
    if must_block == 0 {
        queen_unrestricted(
            moves,
            attack_bits,
            pos,
            allies,
            all_pieces,
            magic_mover,
            all_square_data,
            side,
        );
    } else {
        queen_restricted(
            moves,
            attack_bits,
            pos,
            allies,
            all_pieces,
            magic_mover,
            all_square_data,
            must_block,
            side,
        );
    }
}

fn queen_unrestricted(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    side: Side,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    *attack_bits |= rook_data.bitboard;
    let normals = rook_data
        .normal
        .iter()
        .copied()
        .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None));
    let takes = rook_data
        .possible_takes()
        .filter(|i| allies & i.as_mask() == 0)
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::Queen),
                all_square_data.get(i),
            )
        });
    moves.extend(normals.chain(takes));

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);
    *attack_bits |= bishop_data.bitboard;
    let normals = bishop_data
        .normal
        .iter()
        .copied()
        .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None));

    let takes = bishop_data
        .possible_takes()
        .filter(|i| allies & i.as_mask() == 0)
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::Queen),
                all_square_data.get(i),
            )
        });
    moves.extend(normals.chain(takes));
}

fn queen_restricted(
    moves: &mut Vec<Move>,
    attack_bits: &mut u64,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    must_block: u64,
    side: Side,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    *attack_bits |= rook_data.bitboard;
    let normals = rook_data
        .normal
        .iter()
        .copied()
        .filter(|i| must_block & i.as_mask() != 0)
        .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None));
    let takes = rook_data
        .possible_takes()
        .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::Queen),
                all_square_data.get(i),
            )
        });

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);
    *attack_bits |= bishop_data.bitboard;

    let normals = normals.chain(
        bishop_data
            .normal
            .iter()
            .copied()
            .filter(|i| must_block & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );

    let takes = takes.chain(
        bishop_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::Queen),
                    all_square_data.get(i),
                )
            }),
    );
    moves.extend(normals.chain(takes));
}
