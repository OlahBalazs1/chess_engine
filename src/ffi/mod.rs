use std::{
    ffi::{CStr, c_char},
    ptr::null_mut,
    slice,
};

use crate::{
    board::SearchBoard,
    engine::{evaluate::Outcome, play::Game},
    ffi::struct_reprs::{FFIMove, SimplePieceRepr},
    position::Position,
};

mod struct_reprs;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_free(board: *mut SearchBoard) {
    unsafe {
        std::mem::drop(Box::from_raw(board));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sb_new_default() -> *mut SearchBoard {
    Box::into_raw(Box::new(SearchBoard::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn sb_new_from_fen(fen: *const c_char) -> *mut SearchBoard {
    unsafe {
        Box::into_raw(Box::new(SearchBoard::from_fen(
            CStr::from_ptr(fen).to_str().unwrap(),
        )))
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sb_piecewise_board(
    board: Option<&SearchBoard>,
    arr: *mut SimplePieceRepr,
) {
    unsafe {
        let arr = slice::from_raw_parts_mut(arr, 64);
        for (piece, entry) in std::iter::zip(
            board.expect("Board should not be null").board.board.iter(),
            arr.iter_mut(),
        ) {
            *entry = SimplePieceRepr::from(*piece);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sb_find_all_moves(
    board: Option<&SearchBoard>,
    moves: *mut FFIMove,
    moves_len: usize,
) -> i32 {
    let mut written = 0;
    let board = board.expect("Board should not be null");
    let (pin_state, check_paths) = board.legal_data();
    let found_moves = board.find_all_moves(pin_state, check_paths);
    unsafe {
        let moves = slice::from_raw_parts_mut(moves, moves_len);

        for (mov, entry) in std::iter::zip(found_moves.iter(), moves.iter_mut()) {
            written += 1;
            *entry = From::from(*mov);
        }
    }

    written
}

pub extern "C" fn sb_en_passant_square(
    board: Option<&SearchBoard>,
    out: Option<&mut Position>,
) -> bool {
    let board = board.expect("Board should not be null");
    let out = out.expect("Out should not be null");

    let Some(ep) = board.en_passant_square else {
        return false;
    };
    *out = ep;
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn gm_free(val: *mut Game) {
    unsafe {
        std::mem::drop(Box::from_raw(val));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gm_new() -> *mut Game {
    Box::into_raw(Box::new(Game::default()))
}

// -1 => error
// 0 => white won
// 1 => black won
// 2 => stalemate
#[unsafe(no_mangle)]
pub extern "C" fn gm_play_move(game: Option<&mut Game>, mov: Option<&FFIMove>) -> i32 {
    let game = game.expect("Game should not be null");
    let mov = mov.expect("Move should not be null");
    let mov = From::from(*mov);

    let outcome = game.make_move(&mov);
    outcome.map(|i| i as i32).unwrap_or(-1)
}

#[unsafe(no_mangle)]
pub extern "C" fn gm_searchboard(game: Option<&Game>) -> *mut SearchBoard {
    let Some(game) = game else {
        return null_mut();
    };

    let board = game.get_board().clone();
    Box::into_raw(Box::new(board))
}

#[unsafe(no_mangle)]
pub extern "C" fn gm_best_move(game: Option<&Game>, depth: i32) -> FFIMove {
    let game = game.expect("Game should not be null");

    let best_move = game.find_best_move(depth).map(|i| From::from(i));

    best_move.expect("Don't call Game::best_move() if the outcome of the game is not Ongoing")
}

#[unsafe(no_mangle)]
pub extern "C" fn gm_outcome(game: Option<&Game>) -> Outcome {
    let game = game.expect("Game should not be null");
    game.outcome()
}
