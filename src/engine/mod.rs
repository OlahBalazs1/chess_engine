mod evaluate;
use self::evaluate::*;
use crate::{
    board::SearchBoard,
    moving::{Move, Unmove},
    piece::Side,
    search_data::CheckPath,
};

// it's actually negamax
pub fn minimax(mut board: SearchBoard, depth: i32) -> Move {
    if depth == 0 {
        panic!("Don't call minimax() with a depth of 0")
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);
    let mut evals = Vec::with_capacity(moves.len());
    for mov in moves.iter() {
        let unmake = Unmove::new(&mov, &board);
        board.make(&mov);
        evals.push(minimax_search(&mut board, depth - 1));
        board.unmake(unmake);
    }
    moves[evals
        .iter()
        .enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap()
        .0]
}

fn minimax_search(board: &mut SearchBoard, depth: i32) -> i64 {
    if depth == 0 {
        return evaluate(&board);
    }
    let (pin_state, check_paths) = board.legal_data();
    let moves = board.find_all_moves(pin_state, check_paths);
    let mut evals = Vec::with_capacity(moves.len());
    for mov in moves {
        let unmake = Unmove::new(&mov, &board);
        board.make(&mov);
        evals.push(minimax_search(board, depth - 1));
        board.unmake(unmake);
    }
    evals.iter().max().copied().unwrap_or(0)
}
