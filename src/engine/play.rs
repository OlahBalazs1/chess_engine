use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::{self, SearchBoard},
    engine::{add_board_to_repetition, is_draw_repetition, minimax, negamax},
    moving::Move,
    piece::Side,
};

pub fn play(depth: i32, mut board: SearchBoard, player_side: Side) {
    let repetition = HashMap::with_hasher(BuildNoHashHasher::new());

    loop {
        println!("{}", board.state);
        if board.side() == player_side.opposite() {
            let chosen_move = minimax(board.clone(), depth, &repetition, board.side());
            println!("{}", chosen_move);
            board.make(&chosen_move);
            continue;
        }
        if is_draw_repetition(&board, &repetition) {
            break;
        }

        let (pin_state, check_paths) = board.legal_data();
        let legal_moves = board.find_all_moves(pin_state, check_paths);
        loop {
            print!("Make your move: ");
            let _ = stdout().flush();
            let mut player_move = String::new();
            let _ = stdin().read_line(&mut player_move);
            let Some(player_move) = Move::from_string(&board, player_move.trim_end()) else {
                continue;
            };
            if legal_moves.contains(&player_move) {
                board.make(&player_move);
                break;
            }
        }
        if is_draw_repetition(&board, &repetition) {
            break;
        }
    }
}

pub fn autoplay(depth: i32, mut board: SearchBoard) -> Vec<Move> {
    let mut repetition = HashMap::with_hasher(BuildNoHashHasher::new());
    let mut played_moves = Vec::new();
    loop {
        println!("{}", board.state);
        let chosen_move = minimax(board.clone(), depth, &repetition, board.side());
        println!("{}", chosen_move);
        board.make(&chosen_move);
        played_moves.push(chosen_move);
        add_board_to_repetition(&mut repetition, &board);
        if is_draw_repetition(&board, &repetition) {
            break;
        }
    }
    played_moves
}
pub fn autoplay_nega(depth: i32, mut board: SearchBoard) -> Vec<Move> {
    let mut repetition = HashMap::with_hasher(BuildNoHashHasher::new());
    let mut played_moves = Vec::new();
    loop {
        println!("{}", board.state);
        let chosen_move = negamax(board.clone(), depth, &repetition);
        println!("{}", chosen_move);
        board.make(&chosen_move);
        played_moves.push(chosen_move);
        add_board_to_repetition(&mut repetition, &board);
        if is_draw_repetition(&board, &repetition) {
            break;
        }
    }
    played_moves
}
