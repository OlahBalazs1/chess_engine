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
    let (pin_state, check_paths) = ctx.board.legal_data();
    let moves = ctx.board.find_all_moves(pin_state, check_paths);

    let evals: Vec<i64> = moves
        .par_iter()
        .copied()
        .map(|mov| {
            let ctx = &mut ctx.clone();
            let unmake = Unmove::new(mov, &ctx.board);
            ctx.board.make(&mov);
            let eval = -negamax_eval(ctx, depth, i64::MIN + 1, i64::MAX - 1);
            ctx.board.unmake(unmake);
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

pub fn negamax_eval(ctx: &mut SearchContext, depth: i32, mut alpha: i64, beta: i64) -> i64 {
    if depth == 0 {
        return quiesce(ctx, -beta, -alpha, 0);
        // return evaluate(&ctx.board);
    }
    let (pin_state, check_paths) = ctx.board.legal_data();
    let in_check = check_paths.is_check();
    let mut moves = ctx.board.find_all_moves(pin_state, check_paths);
    moves.sort_by_cached_key(|mov| -rate_move(mov, ctx.board.side()));
    let mut eval = i64::MIN + 1;

    if moves.is_empty() {
        if in_check {
            return i64::MIN + 1;
        } else {
            return 0;
        }
    }

    if ctx.board.halfmove_clock >= 50 || moves.is_empty() {
        return 0;
    }
    for mov in moves {
        let unmake = Unmove::new(mov, &ctx.board);
        ctx.board.make(&mov);
        let repetition = ctx.repetitions.entry(ctx.board.zobrist).or_insert(0);
        *repetition += 1;

        if *repetition == 2 {
            *repetition -= 1;
            ctx.board.unmake(unmake);
            return 0;
        }
        let score = -negamax_eval(ctx, depth - 1, -beta, -alpha);
        eval = cmp::max(score, eval);
        alpha = cmp::max(alpha, eval);

        // rebind it because of the borrow checker
        let repetition = ctx.repetitions.entry(ctx.board.zobrist).or_insert(0);
        *repetition -= 1;
        ctx.board.unmake(unmake);

        if eval >= beta {
            return eval;
        }
    }
    eval
}

fn quiesce(ctx: &mut SearchContext, mut alpha: i64, beta: i64, descended: i32) -> i64 {
    if descended == ctx.quiescence_depth_limit {
        return evaluate(ctx);
    }

    let mut eval = i64::MIN + 1;
    let (pin_state, check_paths) = ctx.board.legal_data();
    let in_check = check_paths.is_check();
    let mut moves = ctx.board.find_all_moves(pin_state, check_paths);
    moves.sort_by_cached_key(|mov| -rate_move(mov, ctx.board.side()));

    if moves.is_empty() {
        if in_check {
            return i64::MIN + 1;
        } else {
            return 0;
        }
    }

    for mov in moves {
        if mov.take.is_none() {
            continue;
        }
        let unmake = Unmove::new(mov, &ctx.board);
        ctx.board.make(&mov);
        let repetition = ctx.repetitions.entry(ctx.board.zobrist).or_insert(0);
        *repetition += 1;

        if *repetition == 2 {
            *repetition -= 1;
            ctx.board.unmake(unmake);
            return 0;
        }
        let score = quiesce(ctx, -beta, -alpha, descended + 1);
        eval = cmp::max(score, eval);
        alpha = cmp::max(alpha, eval);

        // rebind it because of the borrow checker
        let repetition = ctx.repetitions.entry(ctx.board.zobrist).or_insert(0);
        *repetition -= 1;
        ctx.board.unmake(unmake);

        if eval >= beta {
            return eval;
        }
    }

    eval
}

// fn evaluate(board: &SearchBoard) -> i64 {
//     const CENTER_MASK: u64 = 0x1818000000;
//     let mut eval = 0;
//     // eval += (board.side_bitboards(Side::White).combined() & CENTER_MASK).count_ones() as i64;
//     // eval -= (board.side_bitboards(Side::Black).combined() & CENTER_MASK).count_ones() as i64;
//     eval += eval_material(board, Side::White) * 2;
//     eval -= eval_material(board, Side::Black) * 2;

//     eval * who2move(board.side())
// }

// fn eval_material(board: &SearchBoard, side: Side) -> i64 {
//     let mut eval = board.side_bitboards(side)[PAWN].count_ones() as i64;
//     eval += board.side_bitboards(side)[KNIGHT].count_ones() as i64 * 3;
//     eval += board.side_bitboards(side)[BISHOP].count_ones() as i64 * 3;
//     eval += board.side_bitboards(side)[ROOK].count_ones() as i64 * 5;
//     eval += board.side_bitboards(side)[QUEEN].count_ones() as i64 * 9;

//     eval
// }
