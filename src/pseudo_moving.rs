use std::mem::offset_of;

use crate::{
    board::SearchBoard,
    board_repr::BoardRepr,
    moving::{Move, MoveType},
    piece::{Piece, PieceType, Side},
    position::{Offset, Position},
    search_masks::{KING_MASKS, KNIGHT_MASKS, PAWN_TAKE_MASKS},
};

pub fn find_pawn(
    moves: &mut Vec<Move>,
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

pub fn find_knight(moves: &mut Vec<Move>, pos: Position, state: &SearchBoard) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let all_square_data = &state.state.board;
    moves.extend(
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
            }),
    );
}

pub fn find_king(moves: &mut Vec<Move>, pos: Position, state: &SearchBoard, attacked_squares: u64) {
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let castle_rights = state.state.side_castle_rights(side);
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

    if attacked_squares & pos.as_mask() == 0 {
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
}

fn find_rook(moves: &mut Vec<Move>, pos: Position, allies: u64, enemies: u64, state: &SearchBoard) {
    let all_pieces = allies | enemies;

    const DIRECTIONS: [Offset; 4] = [
        Offset::new(1, 0),
        Offset::new(0, 1),
        Offset::new(-1, 0),
        Offset::new(0, -1),
    ];

    for dir in DIRECTIONS {
        traverse_direction(moves, dir, pos, all_pieces, state, PieceType::Rook);
    }
}
fn find_bishop(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    enemies: u64,
    state: &SearchBoard,
) {
    let all_pieces = allies | enemies;

    const DIRECTIONS: [Offset; 4] = [
        Offset::new(1, 1),
        Offset::new(-1, 1),
        Offset::new(-1, -1),
        Offset::new(1, -1),
    ];

    for dir in DIRECTIONS {
        traverse_direction(moves, dir, pos, all_pieces, state, PieceType::Bishop);
    }
}

fn traverse_direction(
    moves: &mut Vec<Move>,
    dir: Offset,
    pos: Position,
    allies: u64,
    state: &SearchBoard,
    piece_type: PieceType,
) {
    for mul in 1..7 {
        let Some(multiplied_dir) = dir.mul(mul) else {
            return;
        };
        let Some(offset_pos) = pos.with_offset(multiplied_dir) else {
            return;
        };

        if allies & offset_pos.as_mask() != 0 {
            return;
        }
        moves.push(Move::new(
            pos,
            offset_pos,
            MoveType::Normal(piece_type),
            state.get_piece_at(offset_pos),
        ));
    }
}

fn find_queen(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    enemies: u64,
    state: &SearchBoard,
) {
    find_rook(moves, pos, allies, enemies, state);
    find_bishop(moves, pos, allies, enemies, state);
}

impl SearchBoard {
    fn find_all_pseudo(&self, attacked_squares: u64, side: Side) -> Vec<Move> {
        use PieceType::*;
        let mut moves = Vec::new();
        let allies = self.side_bitboards(side).combined();
        let enemies = self.side_bitboards(side.opposite()).combined();
        let all_square_data = &self.state.board;
        for i in (0..64).map(Position::from_index) {
            let Some(found_piece) = self.get_piece_at(i) else {
                continue;
            };
            match found_piece.filter_side(side).map(|i| i.piece_type) {
                Some(Pawn) => find_pawn(
                    &mut moves,
                    i,
                    side,
                    allies,
                    enemies,
                    all_square_data,
                    self.state.en_passant_square,
                ),
                Some(Rook) => find_rook(&mut moves, i, allies, enemies, self),
                Some(Knight) => find_knight(&mut moves, i, self),
                Some(Bishop) => find_bishop(&mut moves, i, allies, enemies, self),
                Some(Queen) => find_queen(&mut moves, i, allies, enemies, self),
                Some(King) => find_king(&mut moves, i, self, attacked_squares),
                None => continue,
            }
        }

        moves
    }
}
