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
    position::{Offset, Position},
};
use PieceType::*;

pub fn find_pawn(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
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
    let must_block = match (pin_state, check_path) {
        (0, 0) => 0,
        (0, 1..) => check_path,
        (1.., 0) => pin_state,
        _ => return,
    };

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
        );
    }
}
fn find_pawn_unrestricted(
    moves: &mut Vec<Move>,
    pos: Position,
    side: Side,
    allies: u64,
    enemies: u64,
    all_square_data: &BoardRepr,
    ep_square: Option<Position>,
    can_ep: bool,
) {
    // takes
    let yo = match side {
        Side::White => 1,
        Side::Black => -1,
    };
    let data = &choose_pawn_take_mask(side)[*pos as usize];

    for i in data.positions.iter() {
        if enemies & i.as_mask() != 0 {
            gen_pawn_moves(moves, pos, *i, all_square_data.get(*i));
        } else if ep_square.is_some_and(|ep| ep == *i) && can_ep {
            moves.push(Move::new(pos, *i, MoveType::EnPassant, None));
        }
    }
    let all_pieces = allies | enemies;
    if let Some(to) = pos.with_offset(Offset::new(0, yo)) {
        if all_pieces & to.as_mask() == 0 {
            gen_pawn_moves(moves, pos, to, None);
        } else {
            return;
        }
    } else {
        return;
    }

    if matches!(pos.y(), 1 | 6)
        && let Some(to) = pos.with_offset(Offset::new(0, yo * 2))
        && all_pieces & to.as_mask() == 0
    {
        gen_pawn_moves(moves, pos, to, None);
    }
}

fn find_pawn_restricted(
    moves: &mut Vec<Move>,
    pos: Position,
    side: Side,
    allies: u64,
    enemies: u64,
    all_square_data: &BoardRepr,
    must_block: u64,
    ep_square: Option<Position>,
    can_ep: bool,
) {
    // takes
    let yo = match side {
        Side::White => 1,
        Side::Black => -1,
    };

    let data = &choose_pawn_take_mask(side)[*pos as usize];

    for i in data.positions.iter().copied() {
        if must_block & i.as_mask() != 0 {
            if enemies & i.as_mask() != 0 {
                gen_pawn_moves(moves, pos, i, all_square_data.get(i));
            } else if ep_square.is_some_and(|ep| ep == i) && can_ep {
                moves.push(Move::new(pos, i, MoveType::EnPassant, None));
            }
        }
    }

    let all_pieces = allies | enemies;

    if let Some(to) = pos.with_offset(Offset::new(0, yo))
        && all_pieces & to.as_mask() == 0
    {
        if all_pieces & to.as_mask() == 0 {
            if must_block & to.as_mask() != 0 {
                gen_pawn_moves(moves, pos, to, None);
            }
        } else {
            return;
        }
    }
    if matches!(pos.y(), 1 | 6)
        && let Some(to) = pos.with_offset(Offset::new(0, yo * 2))
        && all_pieces & to.as_mask() == 0
        && must_block & to.as_mask() != 0
    {
        if must_block & to.as_mask() != 0 {
            gen_pawn_moves(moves, pos, to, None);
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
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
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
                .filter(|p| allies & p.as_mask() == 0)
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
                .filter(|p| allies & p.as_mask() == 0 && must_block & p.as_mask() != 0)
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

pub fn find_king(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    check_paths: &CheckPath,
    attacked_squares: u64,
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
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::King),
                    all_square_data.get(i),
                )
            }),
    );

    if let CheckPath::None = check_paths {
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

pub fn find_rook(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
) {
    find_rook_with_magic(moves, pos, state, pin_state, check_paths, &*MAGIC_MOVER)
}

pub fn find_rook_with_magic(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    magic_mover: &MagicMover,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
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
        moves.extend(
            data.normal
                .iter()
                .copied()
                .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None)),
        );

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0)
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
        moves.extend(
            data.normal
                .iter()
                .copied()
                .filter(|i| must_block & i.as_mask() != 0)
                .map(|i| Move::new(pos, i, MoveType::Normal(Rook), None)),
        );

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
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

pub fn find_bishop(
    moves: &mut Vec<Move>,

    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
) {
    find_bishop_with_magic(moves, pos, state, pin_state, check_paths, &*MAGIC_MOVER)
}

pub fn find_bishop_with_magic(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    magic_mover: &MagicMover,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
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
        moves.extend(
            data.normal
                .iter()
                .copied()
                .map(|i| Move::new(pos, i, MoveType::Normal(Bishop), None)),
        );

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0)
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
        moves.extend(
            data.normal
                .iter()
                .copied()
                .filter(|i| must_block & i.as_mask() != 0)
                .map(|i| Move::new(pos, i, MoveType::Normal(Bishop), None)),
        );

        moves.extend(
            data.possible_takes()
                .filter(|i| allies & i.as_mask() == 0 && must_block & i.as_mask() != 0)
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

pub fn find_queen(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
) {
    find_queen_with_magic(moves, pos, state, pin_state, check_paths, &*MAGIC_MOVER)
}

pub fn find_queen_with_magic(
    moves: &mut Vec<Move>,
    pos: Position,
    state: &SearchBoard,
    pin_state: &PinState,
    check_paths: &CheckPath,
    magic_mover: &MagicMover,
) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
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
        queen_unrestricted(moves, pos, allies, all_pieces, magic_mover, all_square_data);
    } else {
        queen_restricted(
            moves,
            pos,
            allies,
            all_pieces,
            magic_mover,
            all_square_data,
            must_block,
        );
    }
}

fn queen_unrestricted(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    moves.extend(
        rook_data
            .normal
            .iter()
            .copied()
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );
    moves.extend(
        rook_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0)
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
    moves.extend(
        bishop_data
            .normal
            .iter()
            .copied()
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );

    moves.extend(
        bishop_data
            .possible_takes()
            .filter(|i| allies & i.as_mask() == 0)
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

fn queen_restricted(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    must_block: u64,
) {
    let rook_data = magic_mover.get_rook(pos, all_pieces);
    moves.extend(
        rook_data
            .normal
            .iter()
            .copied()
            .filter(|i| must_block & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );
    moves.extend(
        rook_data
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

    let bishop_data = magic_mover.get_bishop(pos, all_pieces);

    moves.extend(
        bishop_data
            .normal
            .iter()
            .copied()
            .filter(|i| must_block & i.as_mask() != 0)
            .map(|i| Move::new(pos, i, MoveType::Normal(PieceType::Queen), None)),
    );

    moves.extend(
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
}
