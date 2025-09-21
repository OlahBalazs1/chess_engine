use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    engine::minimax::minimax,
    engine::{add_board_to_repetition, evaluate::outcome, is_draw_repetition},
    moving::Move,
    piece::Side,
};

pub fn play(depth: i32, mut board: SearchBoard, player_side: Side) {
    let mut repetitions = HashMap::with_hasher(BuildNoHashHasher::new());

    loop {
        println!("{}", board.state);
        if board.side() == player_side.opposite() {
            let Some(chosen_move) = minimax(board.clone(), depth, &repetitions).get(0).copied()
            else {
                break;
            };
            println!("{}", chosen_move);
            board.make(&chosen_move);
            continue;
        }
        if is_draw_repetition(&board, &repetitions) {
            break;
        }
        add_board_to_repetition(&mut repetitions, &board);

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
        let (pin_state, check_paths) = board.legal_data();
        let legal_moves = board.find_all_moves(pin_state, check_paths.clone());
        if outcome(&board, &legal_moves, check_paths.is_check(), &repetitions).is_game_over() {
            break;
        }
        add_board_to_repetition(&mut repetitions, &board);
    }
}

pub fn autoplay(depth: i32, mut board: SearchBoard) {
    let mut repetition = HashMap::with_hasher(BuildNoHashHasher::new());
    loop {
        println!("{}", board.state);
        let Some(chosen_move) = minimax(board.clone(), depth, &repetition).get(0).copied() else {
            break;
        };
        println!("{}", chosen_move);
        board.make(&chosen_move);
        add_board_to_repetition(&mut repetition, &board);

        let (pin_state, check_path) = board.legal_data();
        let is_check = check_path.is_check();
        let moves = board.find_all_moves(pin_state, check_path);

        let outcome = outcome(&board, &moves, is_check, &repetition);
        if outcome.is_game_over() {
            println!("{:?}", outcome);
            break;
        }
    }
    println!("{}", board.state);
}
