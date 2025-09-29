use array_init::array_init;

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
