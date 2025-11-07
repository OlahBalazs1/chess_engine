use crate::moving::Move;

// stuff that might help me out later in development
pub mod pseudo_moving;

pub static ROW_BITBOARDS: [u64; 8] = [
    0xFF,
    0xFF << 8,
    0xFF << 16,
    0xFF << 24,
    0xFF << 32,
    0xFF << 40,
    0xFF << 48,
    0xFF << 56,
];

pub fn pgn(game: &[Move]) -> String {
    let mut acc = "".to_string();
    for (nth, moves) in game.chunks(2).enumerate() {
        acc.push_str(&format!("{}. {} ", nth + 1, moves[0].into_algebraic()));
        if let Some(blacks_move) = moves.get(1) {
            acc.push_str(&blacks_move.into_algebraic());
        }
        acc.push_str(" ");
    }
    acc
}
