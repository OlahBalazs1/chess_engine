use std::collections::HashMap;

use nohash_hasher::BuildNoHashHasher;

use crate::engine::ZobristHash;

pub type TranspositionTable = HashMap<ZobristHash, TTableEntry, BuildNoHashHasher<ZobristHash>>;

#[derive(Clone)]
pub struct TTableEntry {
    pub depth: i32,
    pub score: i64,
    pub node_type: NodeType,
}
#[derive(Clone)]
pub enum NodeType {
    PV,
    All,
    Cut,
}
