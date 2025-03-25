struct ZobristHasher {
    piece_boards: [[u64; 64]; 12],
    black: u64,
    hash: Option<u64>,
}

impl ZobristHasher {
    pub fn init() -> Self {
        let piece_boards = [[0u64; 64]; 12];
        for piece in 0..12 {
            for index in 0..64 {}
        }
    }
}

struct BoardState {}
