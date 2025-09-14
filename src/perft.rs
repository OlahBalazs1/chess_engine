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
    board::{self, BoardState, SearchBoard},
    board_repr::KING,
    magic_bitboards::{MAGIC_MOVER, init_magic_mover, print_bits},
    moving::{Move, MoveType, Unmove},
    perft_data::PerftData,
    piece::PieceType,
    position::Position,
    search_data::{CheckPath, PinState},
    search_masks::init_masks,
};

use owo_colors::*;
use std::thread::scope;

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

pub fn perft<const DEPTH: usize>(mut board: SearchBoard) -> [PerftData; DEPTH] {
    let mut results = [PerftData::new(); DEPTH];

    // println!("{}", board.state);
    perft_search(&mut board, &mut results, DEPTH);

    return results;
}

use MoveType::*;
use PieceType::*;

fn perft_search<const N: usize>(
    board: &mut SearchBoard,
    results: &mut [PerftData; N],
    depth: usize,
) {
    if depth == 0 {
        return;
    }
    let (pin_state, check_path) = board.legal_data();
    let attacked = board.state.get_attacked(board.side().opposite());
    let moves = board.find_all_moves(pin_state, check_path, attacked);

    for mov in moves {
        results[N - depth].add_normal(mov);
        let unmove = Unmove::new(&mov, board);
        // let board_copy = board.clone();
        board.make(&mov);

        // let attacked = board.state.get_attacked(board.side().opposite());
        // let (pin_state, check) = board.state.legal_data();
        // if check.is_check() {
        //     let moves = board.find_all_moves(pin_state, check, attacked);
        //     if moves.len() != 0 {
        //         results[N - depth].add_check();
        //     } else {
        //         results[N - depth].add_checkmate();
        //     }
        // }

        perft_search(board, results, depth - 1);
        board.unmake(unmove);

        // if *board != board_copy {
        //     panic!(
        //         "unmake problem\nBefore: {:?}\nafter: {:?}\n{:?}\n{}",
        //         board_copy.state, board.state, mov, mov
        //     );
        // }
    }
}

pub fn perft_copy<const DEPTH: usize>(board: SearchBoard) -> [PerftData; DEPTH] {
    let mut results = [PerftData::new(); DEPTH];

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

    let moves = board.find_all_moves(pin, check, attacked);
    for mov in moves {
        let mut board_clone = board.clone();
        board_clone.make(&mov);

        // logging stuff
        // let attacked = board_clone
        //     .state
        //     .get_attacked(board_clone.side().opposite());
        // let (pin_state, check) = board.state.legal_data();
        // if check.is_check() {
        //     let moves = board.find_all_moves(pin_state, check, attacked);
        //     if moves.len() != 0 {
        //         results[N - depth].add_check();
        //     } else {
        //         results[N - depth].add_checkmate();
        //     }
        // }
        // logging end
        //

        perft_search_copy(board_clone, results, depth - 1);
        results[N - depth].add_normal(mov);
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
    let mut filtered_pseudo = Vec::with_capacity(moves.len());
    for mov in moves.iter().copied() {
        let mut board_copy = board.clone();
        // friendly fire
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
            // if is_reintroduced(mov.piece_type()) {
            //     panic!("Legal left king hanging: {:?}", mov.piece_type());
            // }
            continue;
        }
        // legal movegen debug stuff
        filtered_pseudo.push(mov);

        // logging stuff
        let other_king = board_copy.side_king(board_copy.side());
        // did move put enemy king into check
        if board_copy.is_attacked(other_king) {
            // are there any moves of the enemy?
            let moves = board_copy.find_all_pseudo(board_copy.side());

            let mut filtered_moves = Vec::with_capacity(moves.len());
            filter_moves_and(&board_copy, &moves, |i| filtered_moves.push(*i));

            if filtered_moves.len() == 0 {
                results[N - depth].add_checkmate();
            } else {
                results[N - depth].add_check();
            }
        }
        // logging stuff end

        results[N - depth].add_normal(mov);
        pseudo_perft_copy(board_copy, results, depth - 1);
    }
    let attacked = board.get_attacked_pseudo(board.side().opposite());
    let (pin_state, check_path) = board.state.legal_data();
    let mixed_moves = board.find_all_moves(pin_state, check_path, attacked);

    for i in filtered_pseudo.iter() {
        if !mixed_moves.contains(i) {
            panic!("Mixed doesn't contain: {:?}\n{}\n{:?}", i, i, *board);
        }
    }
    for i in mixed_moves {
        if !filtered_pseudo.contains(&i) {
            panic!("Pseudo doesn't contain: {:?}\n{}\n{:?}", i, i, *board);
        }
    }
}

pub fn test_custom<const N: usize>(board: SearchBoard, targets: Vec<u64>) {
    init_magic_mover();
    init_masks();
    let mut unmake_results = None;
    let mut copy_results = None;
    scope(|s| {
        s.spawn(|| {
            let start = SystemTime::now();
            // let unmake_results = [0; 8];
            unmake_results = Some(perft::<N>(board.clone()));
            println!("Unmake: {} ms", start.elapsed().unwrap().as_millis());
        });

        s.spawn(|| {
            let start = SystemTime::now();
            copy_results = Some(perft_copy::<N>(board.clone()));
            println!("copymake: {} ms", start.elapsed().unwrap().as_millis());
        });
    });
    let unmake_results = unmake_results.unwrap();
    let copy_results = copy_results.unwrap();

    for (i, (okay, (unmake, copy))) in zip(targets, zip(unmake_results, copy_results)).enumerate() {
        let error = (copy.nodes as i64) - (okay as i64);
        let copy_unmake_mismatch = (unmake.nodes as i64) - (copy.nodes as i64);

        let error_str = error.to_string();

        if copy_unmake_mismatch == 0 {
            println!(
                "{}. (okay: {}) {} {}",
                i + 1,
                okay,
                copy.nodes,
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
                unmake.nodes,
                copy.nodes,
                match &error_str as &str {
                    "0" => Style::new().green().style("0"),
                    e => Style::new().red().style(e),
                },
                copy_unmake_mismatch.red()
            )
        }
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
pub fn test<const N: usize>() {
    test_custom::<N>(SearchBoard::default(), TARGETS.to_vec());
}
pub fn pseudo_test<const N: usize>() {
    let board = SearchBoard::default();
    pseudo_test_custom::<N>(board, TARGETS.to_vec());
}

pub fn pseudo_test_custom<const N: usize>(board: SearchBoard, targets: Vec<u64>) {
    init_magic_mover();
    init_masks();
    let mut results = [PerftData::new(); N];
    let start = SystemTime::now();
    pseudo_perft_copy(board.clone(), &mut results, N);
    println!("{:?}", board.state.board.board);

    println!("pseudo: {} ms", start.elapsed().unwrap().as_millis());

    for (i, (okay, copy)) in zip(targets, results).enumerate() {
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

fn filter_moves_and<F: FnMut(&Move) -> ()>(board: &SearchBoard, moves: &[Move], mut f: F) {
    for mov in moves {
        let mut board_copy = board.clone();
        if board_copy
            .get_piece_at(mov.to())
            .is_some_and(|i| i.side() == board_copy.side())
        {
            continue;
        }
        board_copy.make(mov);

        if board_copy.is_attacked(board_copy.side_king(board_copy.side().opposite())) {
            continue;
        }

        f(mov)
    }
}
