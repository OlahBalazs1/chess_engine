use crate::{board::SearchBoard, magic_bitboards::print_bits};

pub fn perft<const DEPTH: usize>() -> [u32; DEPTH] {
    let mut results = [0; DEPTH];
    let mut board = SearchBoard::default();

    // println!("{}", board.state);
    perft_search(board.clone(), &mut results, DEPTH);

    return results;
}

fn perft_search<const N: usize>(mut board: SearchBoard, results: &mut [u32; N], depth: usize) {
    if depth == 0 {
        return;
    }
    let (moves, attacked_squares) = board.find_all_moves();
    results[depth - 1] += moves.len() as u32;
    for mov in moves {
        let unmove = board.make(&mov, attacked_squares);
        println!("{:?}", board.state);
        // print_bits(board.attacked);
        perft_search(board.clone(), results, depth - 1);
        // board.unmake(unmove);
    }
}
