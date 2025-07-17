use owo_colors::colors::White;

use crate::{
    board::SearchBoard,
    board_repr::{BoardRepr, KING, KNIGHT, PAWN},
    moving::{Move, MoveType},
    piece::{self, Piece, PieceType, Side},
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
    let Some(offset_for_take) = pos.with_offset(Offset::new(0, yo)) else {
        return;
    };
    let take_positions = [Offset::new(-1, yo), Offset::new(1, yo)]
        .into_iter()
        .filter_map(|i| pos.with_offset(i));
    // let take_positions = &PAWN_TAKE_MASKS[*offset_for_take as usize].positions;

    for take_pos in take_positions {
        if all_square_data
            .get(take_pos)
            .is_some_and(|target| target.side() != side)
        {
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
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let all_square_data = &state.state.board;

    moves.extend(
        KNIGHT_MASKS[*pos as usize]
            .positions
            .iter()
            .copied()
            .filter(|p| all_square_data.get(*p).is_none_or(|i| i.side() != side))
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
    let side = state.side();
    let allies = state.side_bitboards(side).combined();
    let enemies = state.side_bitboards(side.opposite()).combined();
    let castle_rights = state.state.side_castle_rights(side);
    let all_square_data = &state.state.board;

    moves.extend(
        KING_MASKS[*pos as usize]
            .positions
            .iter()
            .filter(|i| all_square_data.get(**i).is_none_or(|i| i.side() != side))
            .map(|i| {
                Move::new(
                    pos,
                    *i,
                    MoveType::Normal(PieceType::King),
                    all_square_data.get(*i),
                )
            }),
    );

    // let is_attacked = |pos| state.is_attacked(pos, side.opposite());

    // if !state.is_attacked(pos, side.opposite()) {
    //     let short = 0b110 << side.home_y();
    //     let long = 0x60 << side.home_y();
    //
    //     if castle_rights.0
    //         && is_attacked(pos.with_x(3).unwrap())
    //         && is_attacked(pos.with_x(2).unwrap())
    //         && (allies | enemies)
    //             & (pos.with_x(3).unwrap().as_mask() | pos.with_x(2).unwrap().as_mask())
    //             == 0
    //     {
    //         moves.push(Move::new(
    //             pos,
    //             pos.with_x(2).unwrap(),
    //             MoveType::LongCastle,
    //             None,
    //         ));
    //     }
    //
    //     if castle_rights.1
    //         && is_attacked(pos.with_x(5).unwrap())
    //         && is_attacked(pos.with_x(6).unwrap())
    //         && (allies | enemies)
    //             & (pos.with_x(5).unwrap().as_mask() | pos.with_x(6).unwrap().as_mask())
    //             == 0
    //     {
    //         moves.push(Move::new(
    //             pos,
    //             pos.with_x(6).unwrap(),
    //             MoveType::ShortCastle,
    //             None,
    //         ));
    //     }
    // }
}

fn find_rook(moves: &mut Vec<Move>, pos: Position, allies: u64, enemies: u64, state: &SearchBoard) {
    const DIRECTIONS: [Offset; 4] = [
        Offset::new(1, 0),
        Offset::new(0, 1),
        Offset::new(-1, 0),
        Offset::new(0, -1),
    ];

    for dir in DIRECTIONS {
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
    const DIRECTIONS: [Offset; 4] = [
        Offset::new(1, 1),
        Offset::new(-1, 1),
        Offset::new(-1, -1),
        Offset::new(1, -1),
    ];

    for dir in DIRECTIONS {
        traverse_direction(moves, dir, pos, allies, enemies, state, PieceType::Bishop);
    }
}

fn traverse_direction(
    moves: &mut Vec<Move>,
    dir: Offset,
    pos: Position,
    allies: u64,
    enemies: u64,
    state: &SearchBoard,
    piece_type: PieceType,
) {
    let side = state.get_piece_at(pos).unwrap().side();
    for mul in 1..7 {
        let Some(multiplied_dir) = dir.mul(mul) else {
            return;
        };
        let Some(offset_pos) = pos.with_offset(multiplied_dir) else {
            return;
        };

        match state.get_piece_at(offset_pos) {
            Some(i) if i.side() != side => {
                moves.push(Move::new(
                    pos,
                    offset_pos,
                    MoveType::Normal(piece_type),
                    state.get_piece_at(offset_pos),
                ));
                return;
            }
            None => moves.push(Move::new(
                pos,
                offset_pos,
                MoveType::Normal(piece_type),
                state.get_piece_at(offset_pos),
            )),
            _ => return,
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
    const DIRECTIONS: [Offset; 8] = [
        Offset::new(1, 0),
        Offset::new(0, 1),
        Offset::new(-1, 0),
        Offset::new(0, -1),
        Offset::new(1, 1),
        Offset::new(-1, 1),
        Offset::new(-1, -1),
        Offset::new(1, -1),
    ];
    for dir in DIRECTIONS {
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
        let mut attacked = 0;
        for i in (0..64).map(Position::from_index) {
            if self.is_attacked(i, attacker) {
                attacked |= i.as_mask()
            }
        }
        attacked
    }
    pub fn is_attacked(&self, pos: Position, enemy: Side) -> bool {
        use PieceType::*;
        let ally_data = self.side_bitboards(enemy.opposite());
        let enemy_data = self.side_bitboards(enemy);
        let can_king = KING_MASKS[*pos as usize].sum & enemy_data[KING] != 0;
        if can_king {
            return true;
        }

        let yo = match enemy {
            Side::White => -1,
            Side::Black => 1,
        };
        if let Some(pos) = pos.with_offset(Offset::new(0, yo))
            && enemy_data[PAWN] & PAWN_TAKE_MASKS[*pos as usize].sum != 0
        {
            return true;
        }

        if KNIGHT_MASKS[*pos as usize].sum & enemy_data[KNIGHT] != 0 {
            return true;
        }

        let mut moves = Vec::new();
        find_rook(
            &mut moves,
            pos,
            ally_data.combined(),
            enemy_data.combined(),
            self,
        );
        for i in moves.drain(..) {
            if let Some(taken) = i.take
                && let Some(filtered) = taken.filter_side(enemy)
                && matches!(filtered.piece_type, Rook | Queen)
            {
                return true;
            }
        }
        let mut moves = Vec::new();
        find_bishop(
            &mut moves,
            pos,
            ally_data.combined(),
            enemy_data.combined(),
            self,
        );
        for i in moves.drain(..) {
            if let Some(taken) = i.take
                && let Some(filtered) = taken.filter_side(enemy)
                && matches!(filtered.piece_type, Bishop | Queen)
            {
                return true;
            }
        }

        false
    }
}
