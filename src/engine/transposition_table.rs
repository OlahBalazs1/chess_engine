use std::collections::{HashMap, hash_map::Entry};

use nohash_hasher::BuildNoHashHasher;

use crate::engine::ZobristHash;

// pub type TranspositionTable = HashMap<ZobristHash, TTableEntry, BuildNoHashHasher<ZobristHash>>;

#[derive(Clone, Copy, Debug)]
pub struct TTableEntry {
    pub depth: i32,
    pub score: i64,
    pub node_type: NodeType,
}
#[derive(Clone, Copy, Debug)]
pub enum NodeType {
    PV,
    UpperBound,
    LowerBound,
}

pub struct TranspositionTable {
    table: HashMap<ZobristHash, TTableEntry, BuildNoHashHasher<ZobristHash>>,
}
impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::with_hasher(BuildNoHashHasher::default()),
        }
    }

    pub fn insert(&mut self, zobrist: ZobristHash, score: i64, depth: i32, node_type: NodeType) {
        self.table.insert(
            zobrist,
            TTableEntry {
                score,
                depth,
                node_type,
            },
        );
    }

    pub fn get(&self, zobrist: ZobristHash, depth: i32, alpha: i64, beta: i64) -> Option<i64> {
        let entry = self.table.get(&zobrist)?;
        if entry.depth >= depth {
            match entry {
                &TTableEntry {
                    depth: _,
                    score,
                    node_type: NodeType::PV,
                } => Some(score),
                &TTableEntry {
                    depth: _,
                    score,
                    node_type: NodeType::LowerBound,
                } if score >= beta => Some(score),
                &TTableEntry {
                    depth: _,
                    score,
                    node_type: NodeType::UpperBound,
                } if score <= alpha => Some(score),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }
}
