use std::{
    alloc::System,
    cell::UnsafeCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    mem::MaybeUninit,
    ops::Deref,
    sync::{Arc, LazyLock, Mutex},
    time::SystemTime,
};

use crate::{
    board::SearchBoard,
    board_repr::KING,
    engine::perft,
    magic_bitboards::{MAGIC_MOVER, init_magic_mover, print_bits},
    moving::{Move, MoveType, Unmove},
    perft_data::PerftData,
    piece::PieceType,
    position::Position,
    search_data::{CheckPath, PinState},
    search_masks::init_masks,
};

use owo_colors::*;

use std::iter::zip;

pub const TARGETS: [u64; 7] = [
    20,
    400,
    8_902,
    197_281,
    4_865_609,
    119_060_324,
    3_195_901_860,
];

pub static mut COUNTER: u32 = 0;

fn increment_counter() {
    unsafe { COUNTER += 1 }
}

pub fn perft<const DEPTH: usize>() -> [PerftData; DEPTH] {
    let mut results = [PerftData::new(); DEPTH];
    let mut board = SearchBoard::default();

    // println!("{}", board.state);
    perft_search(&mut board, &mut results, DEPTH);

    return results;
}

use MoveType::*;
use PieceType::*;
use rayon::result;

fn perft_search<const N: usize>(
    board: &mut SearchBoard,
    results: &mut [PerftData; N],
    depth: usize,
) {
    if depth == 0 {
        return;
    }
    let (pin_state, check_path) = board.state.legal_data();
    if check_path.is_check() {
        results[N - depth].add_check();
    }
    let attacked_squares = board.state.get_attacked(board.side().opposite());
    let moves = board.find_all_moves(pin_state, check_path, attacked_squares);

    // if moves.len() == 0 {
    //     if check_path.is_check() {
    //         println!("Checkmate")
    //     } else {
    //         println!("Stalemate")
    //     }
    // }
    // if pin_state.combined() != 0 {
    //     print_bits(pin_state.combined());
    // }
    for mov in moves {
        // let board_clone = board.clone();
        // let check_before = check_path.is_check();
        // if check_before {
        //     println!("{}", board.state);
        // }
        results[N - depth].add_normal(mov);
        let unmove = Unmove::new(&mov, board);
        board.make(&mov);

        let attacked = board.state.get_attacked(board.side().opposite());
        let (pin_state, check) = board.state.legal_data();
        if check.is_check() {
            let moves = board.find_all_moves(pin_state, check, attacked);
            if moves.len() != 0 {
                results[N - depth].add_check();
            } else {
                results[N - depth].add_checkmate();
            }
        }

        // if let Some(taken) = mov.take {
        //     if mov.piece_type() == Pawn && mov.to().x() == mov.from().x() {
        //         panic!("Bad pawn take")
        //     }
        // }

        // match board.check_paths {
        //     CheckPath::Blockable(path) => {
        //         print_bits(path);
        //         increment_counter();
        //     }
        //     CheckPath::Multiple => panic!("Wrong checkpath"),
        //     CheckPath::None => {}
        // }
        perft_search(board, results, depth - 1);
        board.unmake(unmove);

        // if board_clone.state != board.state {
        //     println!("depth: {}", results.len() - depth + 1);
        //     println!("{:?}", board_clone.state);
        //     println!("{:?}", board_clone.state.board.get(Position::new(7, 5)));
        //     println!("{:?}", board.state.board.get(Position::new(7, 5)));
        //     println!("After: {}", mov);
        //     // println!("{:?}", board.state); println!("---");
        //     // println!("{:?}", board_clone.state);
        //     return;
        // }
    }
}

pub fn perft_copy<const DEPTH: usize>() -> [PerftData; DEPTH] {
    let mut results = [PerftData::new(); DEPTH];
    let board = SearchBoard::default();

    // println!("{}", board.state);
    perft_search_copy(board, &mut results, DEPTH);

    results
}

fn perft_search_copy<const N: usize>(
    board: SearchBoard,
    results: &mut [PerftData; N],
    depth: usize,
) {
    if depth == 0 {
        return;
    }
    let (pin, check) = board.state.legal_data();
    let attacked = board.state.get_attacked(board.side().opposite());

    let moves = board.find_all_moves(pin, check, 0);
    // results[N - depth] += moves.len() as u32;
    // let moves = board.find_all_moves(pin, check, attacked);
    for mov in moves {
        let mut board_clone = board.clone();
        board_clone.make(&mov);

        let king_pos = board_clone.state.side_king(board_clone.side().opposite());

        if board_clone.state.can_be_taken(king_pos, board_clone.side()) {
            // println!("before: {}", board.state);
            // println!("{}", board_clone.state);
            // print_bits(board_clone.state.get_attacked(board.side().opposite()));
            // print_bits(board_clone.state.get_attacked(board.side()));
            continue;
        }
        let attacked = board_clone
            .state
            .get_attacked(board_clone.side().opposite());
        let (pin_state, check) = board.state.legal_data();
        if check.is_check() {
            let moves = board.find_all_moves(pin_state, check, attacked);
            if moves.len() != 0 {
                results[N - depth].add_check();
            } else {
                results[N - depth].add_checkmate();
            }
        }

        // if board_clone.state.can_be_taken(
        //     board_clone.side_king(board_clone.side()),
        //     board_clone.side().opposite(),
        // ) {
        //     let mov_entry = move_counts.entry(mov).or_insert(0);
        //     *mov_entry += 1;
        // }

        // if mov.piece_type() == King {
        //     increment_counter();
        // }

        // if CheckPath::find(
        //     &board_clone.state,
        //     board_clone.state.find_king(board.side().opposite()),
        //     board_clone.side(),
        // )
        // .is_check()
        // {
        //     continue;
        // }
        // println!("{:?}", board_clone.state);
        // print_bits(board_clone.attacked);
        perft_search_copy(board_clone, results, depth - 1);
        // if board_clone != board_archive {
        //     println!("Mismatch: {:?}", mov);
        //     return;
        // }
    }
}

fn pseudo_perft_copy<const N: usize>(
    board: SearchBoard,
    results: &mut [PerftData; N],
    depth: usize,
) {
    if depth == 0 {
        return;
    }
    let moves = board.find_all_pseudo(board.side());
    for mov in moves {
        let mut board_copy = board.clone();
        if board_copy.get_piece_at(mov.to()).is_some()
            && mov.take.is_none()
            && board_copy.is_attacked(mov.to())
        {
            panic!("is attacked is bad");
        }
        if let Some(taken_piece) = board_copy.get_piece_at(mov.to())
            && taken_piece.side() == board_copy.side()
        {
            continue;
        }
        board_copy.make(&mov);

        // is king in check after its side's move
        // if yes -> move was illegal
        let king = board_copy.side_king(board_copy.side().opposite());

        if board_copy.is_attacked(king) {
            continue;
        }

        let other_king = board_copy.side_king(board_copy.side());
        // did move put enemy king into check
        if board_copy.is_attacked(other_king) {
            // are there any moves of the enemy?
            let moves = board_copy.find_all_pseudo(board_copy.side());

            let filtered_moves = moves
                .iter()
                .filter(|mov| {
                    let mut board_copy = board_copy.clone();
                    board_copy.make(&mov);
                    let king = board_copy.side_king(board_copy.side().opposite());
                    !board_copy.is_attacked_by(king, board_copy.side())
                })
                .collect::<Vec<_>>();
            if filtered_moves.len() == 0 {
                results[N - depth].add_checkmate();
            } else {
                results[N - depth].add_check();
            }
        }

        results[N - depth].add_normal(mov);
        pseudo_perft_copy(board_copy, results, depth - 1);
    }
}

pub fn test<const N: usize>() {
    init_magic_mover();
    init_masks();
    let start = SystemTime::now();
    // let unmake_results = [0; 8];
    let unmake_results = perft::<N>();
    println!("Unmake: {} ms", start.elapsed().unwrap().as_millis());

    let start = SystemTime::now();
    let copy_results = perft_copy::<N>();
    println!("copymake: {} ms", start.elapsed().unwrap().as_millis());

    for (i, (okay, (unmake, copy))) in zip(
        TARGETS,
        zip(unmake_results, copy_results).map(|i| (i.0.nodes, i.1.nodes)),
    )
    .enumerate()
    {
        let error = (copy as i64) - (okay as i64);
        let copy_unmake_mismatch = (unmake as i64) - (copy as i64);

        let error_str = error.to_string();

        if copy_unmake_mismatch == 0 {
            println!(
                "{}. (okay: {}) {} {}",
                i + 1,
                okay,
                copy,
                match &error_str as &str {
                    "0" => Style::new().green().style("0"),
                    e => Style::new().red().style(e),
                },
            )
        } else {
            println!(
                "{}. (okay: {}) un: {} cpy: {} {} {}",
                i + 1,
                okay,
                unmake,
                copy,
                match &error_str as &str {
                    "0" => Style::new().green().style("0"),
                    e => Style::new().red().style(e),
                },
                copy_unmake_mismatch.red()
            )
        }
    }
}

pub fn pseudo_test<const N: usize>() {
    init_magic_mover();
    init_masks();
    let board = SearchBoard::default();
    let mut results = [PerftData::new(); N];
    let start = SystemTime::now();
    pseudo_perft_copy(board.clone(), &mut results, N);
    println!("{:?}", board.state.board.board);

    println!("pseudo: {} ms", start.elapsed().unwrap().as_millis());

    for (i, (okay, copy)) in zip(TARGETS, results).enumerate() {
        let error = (copy.nodes as i64) - (okay as i64);

        let error_str = error.to_string();

        println!(
            "{}. (okay: {}) {} {}",
            i + 1,
            okay,
            copy.nodes,
            match &error_str as &str {
                "0" => Style::new().green().style("0"),
                e => Style::new().red().style(e),
            },
        );
        println!(
            "cap: {}\tep: {}\tcastle: {}\tpromo: {}\tcheck: {}\tmate: {}",
            copy.captures,
            copy.en_passant,
            copy.castles,
            copy.promotions,
            copy.checks,
            copy.checkmates
        );
    }
}
