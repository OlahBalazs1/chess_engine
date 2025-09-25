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

use crate::board::SearchBoard;
use crate::engine::play::autoplay;
pub use crate::util::pseudo_moving;

fn main() {
    let depth = 5;
    let board = SearchBoard::default();
    // let player_side = Side::White;

    perft::test::<5>();
    // autoplay(depth, board);
    // if minimax_moves == negamax_moves {
    //     println!("Success!")
    // }
}
