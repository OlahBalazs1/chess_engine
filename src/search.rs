
    use crate::board::{Bitboards, BoardState};
    use std::iter::Filter;
    use std::ops::{Deref, Range};

    struct MoveIter<M: MoveNotation, List: Deref<Target = [M]> + Sized> {
        board: SearchBoardState,
        square: Range<usize>,

        moves: List,
        state: Range<usize>,
    }

    trait AsMoveIter<M>: Deref<Target = [M]> + Sized
    where
        M: MoveNotation,
    {
        fn as_move_iter<'list>(self, board: SearchBoardState) -> MoveIter<M, Self> {
            MoveIter {
                state: 0..self.len(),
                square: 0..64,
                moves: self,
                board,
            }
        }
    }

    impl<T, M> AsMoveIter<M> for T
    where
        M: MoveNotation,
        T: Deref<Target = [M]>,
    {
    }

    impl<M, List> Iterator for MoveIter<M, List>
    where
        M: MoveNotation,
        List: Deref<Target = [M]> + Sized,
    {
        type Item = M;
        fn next(&mut self) -> Option<Self::Item> {}
    }

    struct SearchBoardState {
        state: BoardState,
        // Attacked by black
        black_attacked: Bitboards,
        // Attacked by white
        white_attacked: Bitboards,

        // A summary of all attack bitboards that threaten the enemy's king
        check_path: Option<u64>,
    }
    use crate::{
        magic_bitboards::{BishopMove, MagicMover, RookMove, MAGIC_MOVER},
        moving::{MoveNotation, MoveType},
        piece::{PieceType, Side},
        position::{Offset, Position},
    };

    use super::QueenMoveIter;

    pub fn find_pawn<M, T>(
        side: Side,
        pos: Position,
        friendlies: u64,
        enemies: u64,
        must_block: u64,
    ) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        let mut moves: Vec<M> = Vec::with_capacity(4);
        let yo = match side {
            Side::White => 1,
            Side::Black => -1,
        };

        // Takes
        use PieceType::*;
        [Offset::new(-1, yo), Offset::new(1, yo)]
            .iter()
            .filter_map(|&off| pos.with_offset(off))
            .filter(|to| {
                enemies & to.as_mask() != 0
                    && friendlies & to.as_mask() == 0
                    && (must_block == 0 || must_block & to.as_mask() < must_block)
            })
            .for_each(|to| match (side, to.y()) {
                (Side::White, 7) | (Side::Black, 0) => {
                    for promote_to in [Rook, Knight, Bishop, Queen] {
                        moves.push(M::new(pos, to, MoveType::Promotion(promote_to)))
                    }
                }
                _ => moves.push(M::new(pos, to, MoveType::Normal(PieceType::Pawn))),
            });

        let forward = [Offset::new(0, yo), Offset::new(0, 2 * yo)];
        let valid_forward = match pos.y() {
            1 | 6 => &forward[..],
            _ => &forward[..1],
        }
        .iter()
        .filter_map(|&off| pos.with_offset(off));

        for to in valid_forward {
            if (friendlies & enemies) & to.as_mask() != 0 {
                break;
            }
            moves.push(M::new(pos, to, MoveType::Normal(PieceType::Pawn)))
        }

        moves.into()
    }

    pub fn find_knight<M, T>(pos: Position, friendlies: u64) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
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
        .filter(|p| friendlies & p.as_mask() != 0)
        .map(|i| M::new(pos, i, MoveType::Normal(PieceType::Knight)))
        .collect::<Vec<M>>()
        .into()
    }

    pub fn find_king<M, T>(
        pos: Position,
        friendlies: u64,
        attacked_squares: u64,
        castle_rights: (bool, bool),
    ) -> T
    where
        M: MoveNotation,
        T: From<Vec<M>>,
    {
        let mut moves: Vec<M> = Vec::with_capacity(10);
        let must_avoid = friendlies & attacked_squares;

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
            .filter(|i| must_avoid & i.as_mask() == 0)
            .map(|i| M::new(pos, i, MoveType::Normal(PieceType::King))),
        );
        let long = 0b11 << (1 + pos.y() * 8);
        let short = 0b11 << (5 + pos.y() * 8);

        if castle_rights.0 && long & must_avoid != 0 {
            moves.push(M::new(
                pos,
                pos.with_x(2),
                MoveType::Normal(PieceType::King),
            ));
        }

        if castle_rights.1 && short & must_avoid != 0 {
            moves.push(M::new(
                pos,
                pos.with_x(6),
                MoveType::Normal(PieceType::King),
            ));
        }

        moves.into()
    }

    pub fn find_rook(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
    ) -> impl Iterator<Item = RookMove> {
        find_rook_with_magic(pos, friendlies, all_pieces, &*MAGIC_MOVER)
    }

    pub fn find_rook_with_magic(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
        magic_mover: &MagicMover,
    ) -> impl Iterator<Item = RookMove> {
        magic_mover
            .get_rook(pos, all_pieces)
            .iter()
            .filter(move |i| friendlies & (1 << *i.to()) == 0)
    }

    pub fn find_bishop(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
    ) -> impl Iterator<Item = BishopMove> {
        find_bishop_with_magic(pos, friendlies, all_pieces, &*MAGIC_MOVER)
    }

    pub fn find_bishop_with_magic(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
        magic_mover: &MagicMover,
    ) -> <BishopMoveMove) -> bool> {
        magic_mover
            .get_bishop(pos, all_pieces)
            .iter()
            .filter(move |i| friendlies & (1 << *i.to()) == 0)

    }

    pub fn find_queen<M, T, R, B>(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
    ) -> QueenMoveIter<R, B>
    where
        M: MoveNotation,
        R: Iterator<Item = RookMove>,
        B: Iterator<Item = BishopMove>,
    {
        find_queen_with_magic(pos, friendlies, all_pieces, &*MAGIC_MOVER)
    }

    pub fn find_queen_with_magic<M, R, B>(
        pos: Position,
        friendlies: u64,
        all_pieces: u64,
        magic_mover: &MagicMover,
    ) -> QueenMoveIter<R, B>
    where
        M: MoveNotation,
        R: Iterator<Item = RookMove>,
        B: Iterator<Item = BishopMove>,
    {
        let bishop_moves = find_bishop_with_magic(pos, friendlies, all_pieces, magic_mover);
        let rook_moves = find_rook_with_magic(pos, friendlies, all_pieces, magic_mover);
        QueenMoveIter {
            rook: rook_moves,
            bishop: bishop_moves,
        }
    }
