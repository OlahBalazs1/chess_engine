pub mod board;
pub mod board_repr;
#[allow(dead_code)]
pub mod engine;
pub mod hashers;
pub mod magic_bitboards;
pub mod moving;
#[allow(dead_code)]
pub mod perft;
pub mod perft_data;
pub mod piece;
pub mod position;
pub mod search;
pub mod search_data;
pub mod search_masks;
pub mod util;
pub mod zobrist;

#[cfg(feature = "ffi")]
pub mod ffi;

pub use crate::util::pseudo_moving;
use crate::{
    board::SearchBoard,
    engine::{evaluate::Outcome, play::Game},
    moving::{Move, MoveType},
    perft::test_unmake,
    position::Position,
};

fn main() {
    let mut game = Game::default();
    game.autoplay(6);
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::{evaluate::Outcome, play::Game},
        moving::{Move, MoveType},
        position::Position,
    };

    #[test]
    fn white_mate_in_one() {
        let mut game = Game::from_fen("8/8/8/3K4/7k/8/6R1/5R2 w - - 0 1");
        let moves: Vec<_> = game.find_best_moves(3).unwrap();
        let moves: Vec<_> = moves.into_iter().map(|e| e.0).collect();

        let stockfish_move = Move::new(
            Position::from_str("f1").unwrap(),
            Position::from_str("h1").unwrap(),
            MoveType::Normal(crate::piece::PieceType::Rook),
            None,
        );
        assert!(moves.contains(&stockfish_move));
        let outcome = game.make_best_move(3);
        assert_eq!(outcome, Outcome::WhiteWon);
    }

    #[test]
    fn black_mate_in_one() {
        let mut game = Game::from_fen("8/8/8/3k4/7K/8/6r1/5r2 b - - 0 1");
        let moves: Vec<_> = game.find_best_moves(3).unwrap();
        let moves: Vec<_> = moves.into_iter().map(|e| e.0).collect();

        let stockfish_move = Move::new(
            Position::from_str("f1").unwrap(),
            Position::from_str("h1").unwrap(),
            MoveType::Normal(crate::piece::PieceType::Rook),
            None,
        );
        assert!(moves.contains(&stockfish_move));
        let outcome = game.make_best_move(3);
        assert_eq!(outcome, Outcome::BlackWon);
    }

    #[test]
    fn white_block_mate_in_one() {
        let mut game = Game::from_fen("7k/8/6r1/K7/6rr/8/8/1R6 w - - 0 1");
        let moves: Vec<_> = game.find_best_moves(5).unwrap();
        let moves: Vec<_> = moves.into_iter().map(|e| e.0).collect();

        let stockfish_move = Move::new(
            Position::from_str("b1").unwrap(),
            Position::from_str("b5").unwrap(),
            MoveType::Normal(crate::piece::PieceType::Rook),
            None,
        );
        assert!(moves.contains(&stockfish_move));
    }

    #[test]
    fn black_block_mate_in_one() {
        let mut game = Game::from_fen("7K/8/6R1/k7/6RR/8/8/1r6 b - - 0 1");
        let moves: Vec<_> = game.find_best_moves(5).unwrap();
        println!("{:#?}", &moves[..5]);
        let moves: Vec<_> = moves.into_iter().map(|e| e.0).collect();

        let stockfish_move = Move::new(
            Position::from_str("b1").unwrap(),
            Position::from_str("b5").unwrap(),
            MoveType::Normal(crate::piece::PieceType::Rook),
            None,
        );
        assert!(moves.contains(&stockfish_move));
    }
}
