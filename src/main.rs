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
use crate::perft::KIWIPETE_TARGETS;
pub use crate::util::pseudo_moving;

fn main() {
    let depth = 5;
    let board = SearchBoard::default();
    // let player_side = Side::White;

    let board = SearchBoard::from_fen(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    );
    perft::test_unmake_custom::<6>(board, KIWIPETE_TARGETS.to_vec());
    // autoplay(depth, board);
    // if minimax_moves == negamax_moves {
    //     println!("Success!")
    // }
}
