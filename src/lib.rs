mod board;
mod board_repr;
#[allow(dead_code)]
mod engine;
mod hashers;
mod magic_bitboards;
mod moving;
#[allow(dead_code)]
mod perft;
mod perft_data;
mod piece;
mod position;
mod search;
mod search_data;
mod search_masks;
mod uci;
mod util;
mod zobrist;

#[cfg(feature = "ffi")]
mod ffi;

use crate::board::SearchBoard;
use crate::engine::play::autoplay;
use crate::perft::KIWIPETE_TARGETS;
use crate::piece::Side;
pub use crate::util::pseudo_moving;

fn main() {
    // let board =
    //     SearchBoard::from_fen("r1b1k1nr/pppp1ppp/n7/4p3/5P2/2NB1N2/PPPB1PPP/R2QR1K1 b kq - 0 1");
    // let (pin_state, check_path) = board.legal_data();
    // println!(
    //     "black: {}\nWhite: {}",
    //     board.side_king(Side::Black),
    //     board.side_king(Side::White)
    // );
    // println!("{:?}\n{:?}", pin_state, check_path);
    // let depth = 5;
    // let board = SearchBoard::default();
    // // let player_side = Side::White;
    //
    //
    //

    perft::test_unmake::<6>();
    // autoplay(depth, board);
    // if minimax_moves == negamax_moves {
    //     println!("Success!")
    // }
}
