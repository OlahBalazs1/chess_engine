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
        add_board_to_repetition,
        evaluate::{Outcome, outcome},
        is_draw_repetition,
        searcher::SearchContext,
        transposition_table::TranspositionTable,
    },
    moving::Move,
    piece::Side,
    util::pgn,
};

pub struct Game {
    ctx: SearchContext,
    last_move_outcome: Outcome,
}

impl Game {
    pub fn get_board(&self) -> &SearchBoard {
        &self.ctx.board
    }
    pub fn make_move(&mut self, mov: &Move) -> Option<Outcome> {
        if self.last_move_outcome.is_game_over() {
            return Some(self.last_move_outcome);
        }
        let (pin_state, check_paths) = self.ctx.board.legal_data();
        let legal_moves = self.ctx.board.find_all_moves(pin_state, check_paths);

        if legal_moves.contains(mov) {
            self.ctx.board.make(&mov);
            add_board_to_repetition(&mut self.ctx.repetitions, &self.ctx.board);

            let (pin_state, check_path) = self.ctx.board.legal_data();
            let is_check = check_path.is_check();
            let moves = self.ctx.board.find_all_moves(pin_state, check_path);

            let outcome = outcome(
                &self.ctx.board,
                !moves.is_empty(),
                is_check,
                &self.ctx.repetitions,
            );
            self.last_move_outcome = outcome;
            Some(outcome)
        } else {
            None
        }
    }

    pub fn find_best_move(&mut self, depth: i32) -> Option<Move> {
        if self.last_move_outcome.is_game_over() {
            return None;
        }
        self.ctx.find_move(depth, depth)
    }

    pub fn autoplay(&mut self, depth: i32) {
        while !self.last_move_outcome.is_game_over() {
            print_board(&self.ctx.board.board);
            let start = SystemTime::now();
            let mov = self.find_best_move(depth).unwrap();
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
    }

    pub fn outcome(&self) -> Outcome {
        self.last_move_outcome
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            ctx: SearchContext::new(SearchBoard::default()),
            last_move_outcome: Outcome::Ongoing,
        }
    }
}
