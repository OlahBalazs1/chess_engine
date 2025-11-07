use std::{
    collections::HashMap,
    io::{Write, stdin, stdout},
};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    engine::{
        add_board_to_repetition,
        evaluate::{Outcome, outcome},
        is_draw_repetition,
        minimax::{minimax, minimax_single_threaded},
        transposition_table::TranspositionTable,
    },
    moving::Move,
    piece::Side,
    util::pgn,
};

pub fn play(depth: i32, mut board: SearchBoard, player_side: Side) {
    let mut repetitions = HashMap::with_hasher(BuildNoHashHasher::new());
    let mut transposition_table = TranspositionTable::default();
    let mut played_moves = Vec::new();

    loop {
        println!("{}", board.state);
        if board.side() == player_side.opposite() {
            let Some(chosen_move) =
                minimax(board.clone(), depth, &repetitions, &mut transposition_table)
                    .get(0)
                    .copied()
            else {
                break;
            };
            println!("{}", chosen_move);
            board.make(&chosen_move);
            played_moves.push(chosen_move);
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
                played_moves.push(player_move);
                break;
            }
        }
        let (pin_state, check_paths) = board.legal_data();
        let legal_moves = board.find_all_moves(pin_state, check_paths.clone());
        if outcome(
            &board,
            !legal_moves.is_empty(),
            check_paths.is_check(),
            &repetitions,
        )
        .is_game_over()
        {
            break;
        }
        add_board_to_repetition(&mut repetitions, &board);
    }
    println!("{}", pgn(&played_moves))
}

pub fn autoplay(depth: i32, mut board: SearchBoard) {
    let mut repetition = HashMap::with_hasher(BuildNoHashHasher::new());
    let mut transposition_table = TranspositionTable::default();
    let mut played_moves = Vec::new();
    loop {
        println!("{}", board.state);
        let Some(chosen_move) =
            minimax(board.clone(), depth, &repetition, &mut transposition_table)
                .get(0)
                .copied()
        else {
            break;
        };
        println!("{}", chosen_move);
        board.make(&chosen_move);
        played_moves.push(chosen_move);
        add_board_to_repetition(&mut repetition, &board);

        let (pin_state, check_path) = board.legal_data();
        let is_check = check_path.is_check();
        let moves = board.find_all_moves(pin_state, check_path);

        let outcome = outcome(&board, !moves.is_empty(), is_check, &repetition);
        if outcome.is_game_over() {
            println!("{:?}", outcome);
            break;
        }
    }
    println!("{}", board.state);
    println!("{}", pgn(&played_moves))
}
pub fn autoplay_single_threaded(depth: i32, mut board: SearchBoard) {
    let mut repetition = HashMap::with_hasher(BuildNoHashHasher::new());
    let mut transposition_table = TranspositionTable::default();
    loop {
        println!("{}", board.state);
        let Some(chosen_move) =
            minimax_single_threaded(board.clone(), depth, &repetition, &mut transposition_table)
                .get(0)
                .copied()
        else {
            break;
        };
        println!("{}", chosen_move);
        board.make(&chosen_move);
        add_board_to_repetition(&mut repetition, &board);

        let (pin_state, check_path) = board.legal_data();
        let is_check = check_path.is_check();
        let moves = board.find_all_moves(pin_state, check_path);

        let outcome = outcome(&board, !moves.is_empty(), is_check, &repetition);
        if outcome.is_game_over() {
            println!("{:?}", outcome);
            break;
        }
    }
    println!("{}", board.state);
}

pub struct Game {
    board: SearchBoard,
    repetitions: HashMap<u64, u8, BuildNoHashHasher<u64>>,
    last_move_outcome: Outcome,
}

impl Game {
    pub fn get_board(&self) -> &SearchBoard {
        &self.board
    }
    pub fn make_move(&mut self, mov: &Move) -> Option<Outcome> {
        if self.last_move_outcome.is_game_over() {
            return Some(self.last_move_outcome);
        }
        let (pin_state, check_paths) = self.board.legal_data();
        let legal_moves = self.board.find_all_moves(pin_state, check_paths);

        if legal_moves.contains(mov) {
            self.board.make(&mov);
            add_board_to_repetition(&mut self.repetitions, &self.board);

            let (pin_state, check_path) = self.board.legal_data();
            let is_check = check_path.is_check();
            let moves = self.board.find_all_moves(pin_state, check_path);

            let outcome = outcome(&self.board, !moves.is_empty(), is_check, &self.repetitions);
            self.last_move_outcome = outcome;
            Some(outcome)
        } else {
            None
        }
    }

    pub fn find_best_move(&self, depth: i32) -> Option<Move> {
        if self.last_move_outcome.is_game_over() {
            return None;
        }
        let mut transposition_table = TranspositionTable::default();
        minimax(
            self.board.clone(),
            depth,
            &self.repetitions,
            &mut transposition_table,
        )
        .get(0)
        .copied()
    }

    pub fn outcome(&self) -> Outcome {
        self.last_move_outcome
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: SearchBoard::default(),
            repetitions: HashMap::with_hasher(BuildNoHashHasher::new()),
            last_move_outcome: Outcome::Ongoing,
        }
    }
}
