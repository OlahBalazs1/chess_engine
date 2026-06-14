use std::{
    alloc::System,
    cmp,
    collections::HashMap,
    io::{Write, stdin, stdout},
    time::{Duration, SystemTime},
};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    board_repr::print_board,
    engine::{
        RepetitionHashmap, add_board_to_repetition,
        evaluate::{Outcome, outcome},
        is_draw_repetition,
        searcher::SearchContext,
        transposition_table::TranspositionTable,
        who2move,
    },
    moving::Move,
    piece::Side,
    util::pgn,
};

pub struct Game {
    board: SearchBoard,
    repetitions: RepetitionHashmap,
    last_move_outcome: Outcome,
}

impl Game {
    pub fn from_fen(fen: &str) -> Self {
        let board = SearchBoard::from_fen(fen);
        let repetitions = HashMap::with_hasher(BuildNoHashHasher::new());

        let (pin_state, check_paths) = board.legal_data();
        let is_check = check_paths.is_check();
        let moves = board.find_all_moves(pin_state, check_paths);

        let outcome = outcome(&board, !moves.is_empty(), is_check, &repetitions);
        Self {
            board,
            repetitions,
            last_move_outcome: outcome,
        }
    }
    pub fn get_board(&self) -> &SearchBoard {
        &self.board
    }
    pub fn make_best_move(&mut self, depth: i32) -> Outcome {
        if self.last_move_outcome.is_game_over() {
            return self.last_move_outcome;
        }
        let Some((mov, _)) = self.find_best_move(depth) else {
            let (_, check_paths) = self.board.legal_data();
            self.last_move_outcome = outcome(
                &self.board,
                false,
                check_paths.is_check(),
                &self.repetitions,
            );
            return self.last_move_outcome;
        };
        self.make_move(&mov);

        return self.last_move_outcome;
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

    pub fn find_best_move(&mut self, depth: i32) -> Option<(Move, i64)> {
        self.find_best_moves(depth)?.get(0).map(|e| *e)
    }

    pub fn find_best_moves(&mut self, depth: i32) -> Option<Vec<(Move, i64)>> {
        if self.last_move_outcome.is_game_over() {
            return None;
        }

        let (pin_state, check_paths) = self.board.legal_data();
        let moves = self.board.find_all_moves(pin_state, check_paths);

        let mut evals = moves
            .into_iter()
            .map(|e| {
                let mut ctx = SearchContext::new(self.board.clone(), self.repetitions.clone(), e);
                ctx.evaluate(depth, depth)
            })
            .collect::<Vec<_>>();
        evals.sort_by_key(|(_, eval)| -eval);

        Some(evals)
    }

    pub fn autoplay(&mut self, depth: i32) -> Outcome {
        while !self.last_move_outcome.is_game_over() {
            print_board(&self.board.board);
            let start = SystemTime::now();
            let (mov, _) = self.find_best_move(depth).unwrap();
            let move_duration = start.elapsed().unwrap();
            self.make_move(&mov);
            println!(
                "made move {} in {} milliseconds",
                mov,
                move_duration.as_millis()
            );

            std::thread::sleep(
                Duration::from_secs(1)
                    .checked_sub(move_duration)
                    .unwrap_or(Duration::from_secs(0)),
            );
        }

        return self.last_move_outcome;
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
