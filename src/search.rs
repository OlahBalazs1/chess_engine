use rand::seq::IndexedRandom;

use crate::board::{Bitboards, BoardRepr, BoardState};
use std::iter;
use std::ops::{Deref, Index, Range};
use std::sync::Arc;

use crate::{
    board::SearchBoard,
    magic_bitboards::{MagicMover, MAGIC_MOVER},
    moving::{Move, MoveType},
    piece::{PieceType, Side},
    position::{Offset, Position},
};

pub struct MovesIter<'a> {
    source: std::vec::IntoIter<Move>,
    board: &'a SearchBoard,
    on_square: Range<u8>,
}
impl<'a> MovesIter<'a> {
    pub fn init(board: &'a SearchBoard) -> Self {
        let mut on_square = 0u8..64;
        let mut source = None;
        while let Some(next_square) = on_square.next() {
            if let Some(next_source) = board
                .find_moves_at(Position::from_index(next_square), board.side())
                .map(|i: Vec<_>| i.into_iter())
            {
                source = Some(next_source);
                break;
            }
        }

        Self {
            source: source.expect("Board should have pieces of both side on it"),
            on_square,
            board,
        }
    }
}
impl Iterator for MovesIter<'_> {
    type Item = Move;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.source.next() {
            return Some(next);
        }

        while let Some(next_square) = self.on_square.next() {
            let next_square = Position::from_index(next_square);
            if let Some(next_source) = self
                .board
                .find_moves_at::<Vec<_>>(next_square, self.board.side())
            {
                self.source = next_source.into_iter();
                if let Some(next_move) = self.source.next() {
                    return Some(next_move);
                }
            }
        }
        None
    }
}

pub fn find_pawn<T>(
    side: Side,
    pos: Position,
    allies: u64,
    enemies: u64,
    must_block: u64,
    all_square_data: &BoardRepr,
) -> T
where
    T: From<Vec<Move>>,
{
    let mut moves: Vec<_> = Vec::with_capacity(4);
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
                            .and_then(|i| i.filter_side(take_side).map(|i| i.role())),
                    ))
                }
            }
            _ => moves.push(Move::new(
                pos,
                to,
                MoveType::Normal(PieceType::Pawn),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(take_side).map(|i| i.role())),
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

    moves.into()
}

pub fn find_knight<T>(pos: Position, allies: u64, all_square_data: &BoardRepr, side: Side) -> T
where
    T: From<Vec<Move>>,
{
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
    })
    .collect::<Vec<_>>()
    .into()
}

pub fn find_king<T>(
    pos: Position,
    allies: u64,
    attacked_squares: u64,
    enemies: u64,
    castle_rights: (bool, bool),
    all_square_data: &BoardRepr,
    side: Side,
) -> T
where
    T: From<Vec<Move>>,
{
    let mut moves: Vec<_> = Vec::with_capacity(8);
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

    moves.into()
}

pub fn find_rook(
    pos: Position,
    allies: u64,
    all_pieces: u64,
    all_square_data: &BoardRepr,
    side: Side,
) -> Vec<Move> {
    find_rook_with_magic(
        pos,
        allies,
        all_pieces,
        &*MAGIC_MOVER,
        all_square_data,
        side,
    )
}

pub fn find_rook_with_magic<T>(
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    side: Side,
) -> T
where
    T: From<Vec<Move>>,
{
    magic_mover
        .get_rook(pos, all_pieces)
        .iter()
        .cloned()
        .filter(|i| allies & i.as_mask() == 0)
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::Rook),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
            )
        })
        .collect::<Vec<_>>()
        .into()
}

pub fn find_bishop<T>(
    pos: Position,
    allies: u64,
    all_pieces: u64,
    all_square_data: &BoardRepr,
    side: Side,
) -> T
where
    T: From<Vec<Move>>,
{
    find_bishop_with_magic(
        pos,
        allies,
        all_pieces,
        &*MAGIC_MOVER,
        all_square_data,
        side,
    )
}

pub fn find_bishop_with_magic<T>(
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    side: Side,
) -> T
where
    T: From<Vec<Move>>,
{
    magic_mover
        .get_bishop(pos, all_pieces)
        .iter()
        .cloned()
        .filter(|i| allies & i.as_mask() == 0)
        .map(|i| {
            Move::new(
                pos,
                i,
                MoveType::Normal(PieceType::Bishop),
                all_square_data
                    .get(pos)
                    .and_then(|i| i.filter_side(side).map(|i| i.role())),
            )
        })
        .collect::<Vec<_>>()
        .into()
}

pub fn find_queen<T>(
    pos: Position,
    allies: u64,
    all_pieces: u64,
    all_square_data: &BoardRepr,
    side: Side,
) -> T
where
    T: From<Vec<Move>>,
{
    find_queen_with_magic(
        pos,
        allies,
        all_pieces,
        &*MAGIC_MOVER,
        all_square_data,
        side,
    )
}

pub fn find_queen_with_magic<T>(
    pos: Position,
    allies: u64,
    all_pieces: u64,
    magic_mover: &MagicMover,
    all_square_data: &BoardRepr,
    side: Side,
) -> T
where
    T: From<Vec<Move>>,
{
    let mut queen_moves: Vec<Move> = vec![];
    let bishop_moves = find_bishop_with_magic::<Vec<Move>>(
        pos,
        allies,
        all_pieces,
        magic_mover,
        all_square_data,
        side,
    )
    .into_iter()
    .map(|mut i| {
        i.move_type = MoveType::Normal(PieceType::Queen);
        i
    });
    let rook_moves = find_rook_with_magic::<Vec<Move>>(
        pos,
        allies,
        all_pieces,
        magic_mover,
        all_square_data,
        side,
    )
    .into_iter()
    .map(|mut i| {
        i.move_type = MoveType::Normal(PieceType::Queen);
        i
    });
    for (bishop, rook) in iter::zip(bishop_moves, rook_moves) {
        queen_moves.push(bishop);
        queen_moves.push(rook);
    }

    queen_moves.into()
}
