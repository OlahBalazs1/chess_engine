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
    engine::play::{autoplay, autoplay_single_threaded},
};

fn main() {
    autoplay_single_threaded(5, SearchBoard::default());
}
