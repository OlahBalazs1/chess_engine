use rayon::prelude::*;
use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use nohash_hasher::BuildNoHashHasher;

use crate::{
    board::SearchBoard,
    board_repr::print_board,
    engine::{
        RepetitionHashmap, add_board_to_repetition,
        evaluate::{Outcome, evaluate, outcome},
        searcher::SearchContext,
    },
    moving::Move,
};

pub struct Bot {
    board: SearchBoard,
    repetitions: RepetitionHashmap,
    last_move_outcome: Outcome,
}

impl Bot {
    pub fn from_fen(fen: &str) -> Self {
        let board = SearchBoard::from_fen(fen);
        let repetitions = HashMap::with_hasher(BuildNoHashHasher::new());

        let (pin_state, check_paths) = board.legal_data();
        let is_check = check_paths.is_check();
        let moves = board.find_all_moves(pin_state, check_paths, false);

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
    pub fn static_evaluate(&self) -> i64 {
        return evaluate(&self.board, &self.repetitions, 0);
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
        let legal_moves = self.board.find_all_moves(pin_state, check_paths, false);

        if legal_moves.contains(mov) {
            self.board.make(&mov);
            add_board_to_repetition(&mut self.repetitions, &self.board);

            let (pin_state, check_path) = self.board.legal_data();
            let is_check = check_path.is_check();
            let moves = self.board.find_all_moves(pin_state, check_path, false);

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
        let moves = self.board.find_all_moves(pin_state, check_paths, false);

        let mut evals = moves
            .into_par_iter()
            .map(|e| {
                let mut ctx = SearchContext::new(self.board.clone(), self.repetitions.clone(), *e);
                ctx.evaluate(depth, depth)
            })
            .collect::<Vec<_>>();
        evals.sort_by_key(|(_, eval)| -eval);
        for (mov, eval) in &evals {
            println!("{eval}: {mov}");
        }
        let best_eval = evals.get(0)?.1;
        evals.retain(|e| best_eval == e.1);

        Some(evals)
    }

    pub fn autoplay(&mut self, depth: i32) -> Outcome {
        while !self.last_move_outcome.is_game_over() {
            print_board(&self.board.board);
            let start = SystemTime::now();
            let (mov, rating) = self.find_best_move(depth).unwrap();
            let move_duration = start.elapsed().unwrap();
            self.make_move(&mov);
            println!(
                "made move {mov} in {} milliseconds (rating: {rating})",
                move_duration.as_millis()
            );

            std::thread::sleep(
                Duration::from_secs(1)
                    .checked_sub(move_duration)
                    .unwrap_or(Duration::from_secs(0)),
            );
        }

        print_board(&self.board.board);

        return self.last_move_outcome;
    }

    pub fn outcome(&self) -> Outcome {
        self.last_move_outcome
    }
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            board: SearchBoard::default(),
            repetitions: HashMap::with_hasher(BuildNoHashHasher::new()),
            last_move_outcome: Outcome::Ongoing,
        }
    }
}
