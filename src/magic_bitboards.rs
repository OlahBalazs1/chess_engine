#[path = "./utils.rs"]
mod utils;
use crate::utils::{Move, Position};

use std::iter::repeat_n;
struct MagicMover{
    rook_magics: [[SquareMagic; 8 ]; 8],
    bishop_magics: [[SquareMagic; 8 ]; 8],
}

struct SquareMagic{
    moves: Box<[Move]>,
    premask: u64,
    magic: u64,
    shift: u8
}

impl SquareMagic{
    fn find_rook_magic(pos: Position) -> Self{

    }

    fn rook_from_magic(pos: Position, magic: u64) -> Self {
        unimplemented!()
    }

    fn bishop_from_magic(pos: Position, magic: u64) -> Self {
        unimplemented!()
    }
}

fn rook_blocker_possible_moves(blocker_config: u64, start_pos: Position) -> Box<[Move]>{
    let mut moves = vec![];

    let mut left = true;
    let mut right = true;
    let mut down = true;
    let mut up = true;
    for i in 0..7{
        if left{
            if let Some(position) = start_pos.sub_x(i){
                left = blocker_config & (1 << position.index()) == 0;
                moves.push(Move::new(start_pos, position));
            }
            else {
                left = false
            }

        }
        if right{
            if let Some(position) = start_pos.add_x(i){
                right = blocker_config & (1 << position.index()) == 0;
                moves.push(Move::new(start_pos, position));
            }
            else {
                right = false
            }
        }
        if down{
            if let Some(position) = start_pos.sub_y(i){
                down = blocker_config & (1 << position.index()) == 0;
                moves.push(Move::new(start_pos, position));
            }
            else {
                down = false
            }
        }
        if up{
            if let Some(position) = start_pos.add_y(i){
                up = blocker_config & (1 << position.index()) == 0;
                moves.push(Move::new(start_pos, position));
            }
            else {
                up = false
            }
        }

    }
    moves.into_boxed_slice()
}

fn generate_blockers(indices: Box<[u8]>) -> Box<[u64]> {
    let mut blockers = vec![];

    for combination in 0..(1 << indices.len()){
        let bitboard = {
            let mut bitboard = 0u64;
            for (index, i ) in indices.iter().enumerate(){
                bitboard |= (combination & (1 << index) >> index) << i;
            } 
            bitboard
        };
        blockers.push(bitboard);
    }

    blockers.into_boxed_slice()
}

fn rook_indices(pos: Position) -> Box<[u8]> {
    let x = pos.x();
    let y = pos.y();
    let mut indices = Vec::with_capacity(14);
    for i in 1..7{
        if i != y{
            indices.push(Position::new(x, i).index());
        }
        if i != x{
            indices.push(Position::new(i, y).index());
        }
    }
    indices.into_boxed_slice()
}