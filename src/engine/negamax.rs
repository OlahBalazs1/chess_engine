use rayon::prelude::*;
use std::{cmp, panic};

use crate::{
    board::SearchBoard,
    board_repr::{BISHOP, KNIGHT, PAWN, QUEEN, ROOK},
    engine::{
        MinimaxResult,
        evaluate::{evaluate, rate_move},
        searcher::SearchContext,
        who2move,
    },
    moving::Unmove,
    piece::{
        Piece,
        PieceType::{self, Queen},
        Side,
    },
};

pub fn negamax(ctx: &mut SearchContext, depth: i32) -> MinimaxResult {
    let (pin_state, check_paths) = ctx.board().legal_data();
    let moves = ctx.board().find_all_moves(pin_state, check_paths);

    let evals: Vec<i64> = moves
        .par_iter()
        .copied()
        .map(|mov| {
            let ctx = &mut ctx.clone();
            let eval = -ctx.evaluate(depth, depth).1;
            eval
        })
        .collect();

    let Some(max_eval) = evals.iter().max().copied() else {
        return MinimaxResult { best_moves: vec![] };
    };

    println!("max eval: {}", max_eval);

    MinimaxResult {
        best_moves: moves
            .into_iter()
            .enumerate()
            .filter(|(index, _)| evals[*index] == max_eval)
            .map(|(_, e)| e)
            .collect(),
    }
}
