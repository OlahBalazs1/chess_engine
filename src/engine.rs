use crate::{
    board::SearchBoard,
    magic_bitboards::print_bits,
    moving::{Move, MoveType},
    piece::PieceType,
    position::Position,
};

static mut LAST_MOVE: Option<Move> = None;

fn set_last_move(mov: Move) {
    unsafe { LAST_MOVE = Some(mov) }
}
fn get_last_move() -> Move {
    unsafe { LAST_MOVE.unwrap() }
}

pub fn perft<const DEPTH: usize>() -> [u32; DEPTH] {
    let mut results = [0; DEPTH];
    let mut board = SearchBoard::default();

    // println!("{}", board.state);
    perft_search(&mut board, &mut results, DEPTH);

    return results;
}

fn perft_search<const N: usize>(board: &mut SearchBoard, results: &mut [u32; N], depth: usize) {
    if depth == 0 {
        return;
    }
    let (moves, attacked_squares) = board.find_all_moves();
    results[depth - 1] += moves.len() as u32;
    for (index, mov) in moves.iter().enumerate() {
        // let last_arch;
        // unsafe {
        //     last_arch = LAST_MOVE;
        // }
        // let board_clone = board.clone();
        // if mov.take.is_some_and(|i| i.side == board.side()) {
        //     panic!("Friendly fire")
        // }
        // set_last_move(*mov);
        let unmove = board.make(&mov, attacked_squares);
        perft_search(board, results, depth - 1);
        board.unmake(unmove);

        // if board_clone.state != board.state {
        //     println!("depth: {}", results.len() - depth + 1);
        //     println!("{:?}", board_clone.state);
        //     println!("{:?}", board_clone.state.board.get(Position::new(7, 5)));
        //     println!("{:?}", board.state.board.get(Position::new(7, 5)));
        //     if let Some(last_arch) = last_arch {
        //         println!("Mismatch: {}", last_arch)
        //     }
        //     println!("After: {}", mov);
        //     // println!("{:?}", board.state); println!("---");
        //     // println!("{:?}", board_clone.state);
        //     return;
        // }
    }
}

pub fn perft_copy<const DEPTH: usize>() -> [u32; DEPTH] {
    let mut results = [0; DEPTH];
    let board = SearchBoard::default();

    // println!("{}", board.state);
    perft_search_copy(board, &mut results, DEPTH);

    return results;
}

fn perft_search_copy<const N: usize>(board: SearchBoard, results: &mut [u32; N], depth: usize) {
    if depth == 0 {
        return;
    }
    let (moves, attacked_squares) = board.find_all_moves();
    results[depth - 1] += moves.len() as u32;
    for mov in moves {
        let mut board_clone = board.clone();
        let _ = board_clone.make(&mov, attacked_squares);
        // println!("{:?}", board_clone.state);
        // print_bits(board_clone.attacked);
        perft_search_copy(board_clone.clone(), results, depth - 1);
        // if board_clone != board_archive {
        //     println!("Mismatch: {:?}", mov);
        //     return;
        // }
    }
}
