pub const ROOK_DIRECTIONS: [Offset; 4] = [
    Offset::new(1, 0),
    Offset::new(0, 1),
    Offset::new(-1, 0),
    Offset::new(0, -1),
];
const BISHOP_DIRECTIONS: [Offset; 4] = [
    Offset::new(1, 1),
    Offset::new(-1, 1),
    Offset::new(-1, -1),
    Offset::new(1, -1),
];
const QUEEN_DIRECTIONS: [Offset; 8] = [
    Offset::new(1, 0),
    Offset::new(0, 1),
    Offset::new(-1, 0),
    Offset::new(0, -1),
    Offset::new(1, 1),
    Offset::new(-1, 1),
    Offset::new(-1, -1),
    Offset::new(1, -1),
];
const KING_DIRECTIONS: [Offset; 8] = [
    Offset::new(1, 0),
    Offset::new(0, 1),
    Offset::new(-1, 0),
    Offset::new(0, -1),
    Offset::new(1, 1),
    Offset::new(-1, 1),
    Offset::new(-1, -1),
    Offset::new(1, -1),
];
const WHITE_PAWN_DIRECTIONS: [Offset; 2] = [Offset::new(-1, 1), Offset::new(1, 1)];
const BLACK_PAWN_DIRECTIONS: [Offset; 2] = [Offset::new(-1, -1), Offset::new(1, -1)];
const KNIGHT_DIRECTIONS: [Offset; 8] = [
    Offset::new(1, -2),
    Offset::new(1, 2),
    Offset::new(-1, -2),
    Offset::new(-1, 2),
    Offset::new(2, -1),
    Offset::new(2, 1),
    Offset::new(-2, -1),
    Offset::new(-2, 1),
];

use crate::{
    board::SearchBoard,
    board_repr::BoardRepr,
    moving::{Move, MoveType},
    piece::{Piece, PieceType, Side},
    position::{Offset, Position},
};

#[allow(unused_imports)]
use crate::search::{
    find_bishop as legal_bishop, find_king as legal_king, find_knight as legal_knight,
    find_pawn as legal_pawn, find_queen as legal_queen, find_rook as legal_rook,
};

pub fn find_pawn(
    moves: &mut Vec<Move>,
    pos: Position,
    side: Side,
    _allies: u64,
    _enemies: u64,
    all_square_data: &BoardRepr,
    ep_square: Option<Position>,
) {
    // takes
    let yo = match side {
        Side::White => 1,
        Side::Black => -1,
    };
    // let take_positions = &PAWN_TAKE_MASKS[*offset_for_take as usize].positions;

    for take_pos in choose_pawn_direction(side)
        .iter()
        .copied()
        .filter_map(|i| pos.with_offset(i))
    {
        if all_square_data.get(take_pos).is_some() {
            gen_pawn_moves(moves, pos, take_pos, all_square_data.get(take_pos));
        } else if ep_square.is_some_and(|ep| ep == take_pos) {
            moves.push(Move::new(pos, take_pos, MoveType::EnPassant, None));
        }
    }

    if let Some(to) = pos.with_offset(Offset::new(0, yo))
        && all_square_data.get(to).is_none()
    {
        gen_pawn_moves(moves, pos, to, None);
    } else {
        return;
    };

    if matches!(pos.y(), 1 | 6)
        && let Some(to) = pos.with_offset(Offset::new(0, yo * 2))
        && all_square_data.get(to).is_none()
    {
        gen_pawn_moves(moves, pos, to, None);
    } else {
        return;
    };
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
    let all_square_data = &state.state.board;

    moves.extend(
        KNIGHT_DIRECTIONS
            .iter()
            .copied()
            .filter_map(|i| pos.with_offset(i))
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

pub fn find_king(moves: &mut Vec<Move>, pos: Position, state: &SearchBoard) {
    let all_square_data = &state.state.board;

    moves.extend(
        KING_DIRECTIONS
            .iter()
            .copied()
            .filter_map(|i| pos.with_offset(i))
            .map(|i| {
                Move::new(
                    pos,
                    i,
                    MoveType::Normal(PieceType::King),
                    all_square_data.get(i),
                )
            }),
    );
    let side = state.side();
    let castle_rights = state.side_castle_rights(side);
    let all_pieces =
        state.side_bitboards(Side::White).combined() | state.side_bitboards(Side::Black).combined();

    let is_attacked = |pos| state.is_attacked_by(pos, side.opposite());

    if !state.is_attacked(pos) {
        // long
        if castle_rights.0
            && !is_attacked(pos.with_x(2).unwrap())
            && !is_attacked(pos.with_x(3).unwrap())
            && all_pieces
                & (pos.with_x(1).unwrap().as_mask()
                    | pos.with_x(2).unwrap().as_mask()
                    | pos.with_x(3).unwrap().as_mask())
                == 0
        {
            moves.push(Move::new(
                pos,
                pos.with_x(2).unwrap(),
                MoveType::LongCastle,
                None,
            ));
        }

        // short
        if castle_rights.1
            && !is_attacked(pos.with_x(5).unwrap())
            && !is_attacked(pos.with_x(6).unwrap())
            && all_pieces & (pos.with_x(5).unwrap().as_mask() | pos.with_x(6).unwrap().as_mask())
                == 0
        {
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
    for dir in ROOK_DIRECTIONS {
        traverse_direction(moves, dir, pos, allies, enemies, state, PieceType::Rook);
    }
}
fn find_bishop(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    enemies: u64,
    state: &SearchBoard,
) {
    for dir in BISHOP_DIRECTIONS {
        traverse_direction(moves, dir, pos, allies, enemies, state, PieceType::Bishop);
    }
}

fn traverse_direction(
    moves: &mut Vec<Move>,
    dir: Offset,
    pos: Position,
    _allies: u64,
    _enemies: u64,
    state: &SearchBoard,
    piece_type: PieceType,
) {
    for mul in 1..8 {
        let Some(multiplied_dir) = dir.mul(mul) else {
            return;
        };
        let Some(offset_pos) = pos.with_offset(multiplied_dir) else {
            return;
        };

        let taken = state.get_piece_at(offset_pos);
        moves.push(Move::new(
            pos,
            offset_pos,
            MoveType::Normal(piece_type),
            taken,
        ));

        if taken.is_some() {
            return;
        }
    }
}

fn find_queen(
    moves: &mut Vec<Move>,
    pos: Position,
    allies: u64,
    enemies: u64,
    state: &SearchBoard,
) {
    for dir in QUEEN_DIRECTIONS {
        traverse_direction(moves, dir, pos, allies, enemies, state, PieceType::Queen);
    }
}

impl SearchBoard {
    pub fn find_all_pseudo(&self, side: Side) -> Vec<Move> {
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
                // Some(Rook) => find_rook(&mut moves, i, self, &pin_state, &check_paths),
                Some(Knight) => find_knight(&mut moves, i, self),
                Some(Bishop) => find_bishop(&mut moves, i, allies, enemies, self),
                Some(Queen) => find_queen(&mut moves, i, allies, enemies, self),
                Some(King) => find_king(&mut moves, i, self),
                None => continue,
            }
        }

        moves
    }

    pub fn get_attacked_pseudo(&self, attacker: Side) -> u64 {
        let mut accumulator = 0;
        let all_pieces = (self.side_bitboards(Side::White).combined()
            | self.side_bitboards(Side::Black).combined())
            & !self.side_king(attacker.opposite()).as_mask();
        for i in (0..64).map(|i| Position::from_index(i)) {
            let Some(found_piece) = self.get_piece_at(i) else {
                continue;
            };
            if let None = found_piece.filter_side(attacker) {
                continue;
            };
            let role = found_piece.role();

            accumulator |= match role {
                PieceType::King => KING_DIRECTIONS
                    .iter()
                    .copied()
                    .filter_map(|off| i.with_offset(off))
                    .fold(0, |acc, i| acc | i.as_mask()),
                PieceType::Pawn => choose_pawn_direction(attacker)
                    .iter()
                    .copied()
                    .filter_map(|off| i.with_offset(off))
                    .fold(0, |acc, i| acc | i.as_mask()),
                PieceType::Knight => KNIGHT_DIRECTIONS
                    .iter()
                    .copied()
                    .filter_map(|off| i.with_offset(off))
                    .fold(0, |acc, i| acc | i.as_mask()),
                PieceType::Queen => simple_traverse(i, all_pieces, QUEEN_DIRECTIONS),
                PieceType::Rook => simple_traverse(i, all_pieces, ROOK_DIRECTIONS),
                PieceType::Bishop => simple_traverse(i, all_pieces, BISHOP_DIRECTIONS),
            }
        }

        accumulator
    }
    pub fn is_attacked_by(&self, pos: Position, enemy: Side) -> bool {
        self.get_attacked_pseudo(enemy) & pos.as_mask() != 0
    }
    pub fn is_attacked(&self, pos: Position) -> bool {
        match self.get_piece_at(pos) {
            Some(piece) => self.is_attacked_by(pos, piece.side().opposite()),
            None => self.is_attacked_by(pos, Side::White) || self.is_attacked_by(pos, Side::Black),
        }
    }
}

fn choose_pawn_direction(side: Side) -> [Offset; 2] {
    match side {
        Side::White => WHITE_PAWN_DIRECTIONS,
        Side::Black => BLACK_PAWN_DIRECTIONS,
    }
}

fn simple_traverse<const N: usize>(pos: Position, all_pieces: u64, dirs: [Offset; N]) -> u64 {
    let mut acc = 0;
    for i in dirs {
        for mult in 1..8 {
            let Some(multiplied) = i.mul(mult) else {
                break;
            };
            let Some(off_pos) = pos.with_offset(multiplied) else {
                break;
            };
            acc |= off_pos.as_mask();
            if all_pieces & off_pos.as_mask() != 0 {
                break;
            }
        }
    }
    acc
}
