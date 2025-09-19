#![allow(dead_code)]
mod board;
mod board_repr;
mod engine;
mod hashers;
mod magic_bitboards;
mod moving;
mod perft;
mod perft_data;
mod piece;
mod position;
mod search;
mod search_data;
mod search_masks;
mod util;
mod zobrist;

use std::collections::HashMap;
use std::io::{Write, stdin, stdout};

use nohash_hasher::BuildNoHashHasher;

use crate::engine::play::{autoplay, autoplay_nega, play};
use crate::engine::{evaluate::evaluate, minimax};
use crate::moving::Move;
use crate::piece::Side;
use crate::position::Position;
use crate::search_masks::{BLACK_PAWN_TAKE_MASKS, WHITE_PAWN_TAKE_MASKS};
pub use crate::util::pseudo_moving;
use crate::{board::SearchBoard, perft::*};

use crate::magic_bitboards::{slide_blocker_possible_moves, test_rook_indices};

fn main() {
    let depth = 4;
    let board = SearchBoard::default();
    let player_side = Side::White;

    let minimax_moves = autoplay(depth, board);
    println!("Negamax");
    // let board = SearchBoard::default();
    // let negamax_moves = autoplay_nega(depth, board);
    // if minimax_moves == negamax_moves {
    //     println!("Success!")
    // }
}
