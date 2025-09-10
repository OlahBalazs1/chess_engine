use crate::moving::{Move, MoveType};

#[derive(Clone, Copy)]
pub struct PerftData {
    pub nodes: u32,
    pub captures: u32,
    pub en_passant: u32,
    pub castles: u32,
    pub promotions: u32,
    pub checks: u32,
    pub checkmates: u32,
}

impl PerftData {
    pub fn new() -> Self {
        PerftData {
            nodes: 0,
            captures: 0,
            en_passant: 0,
            castles: 0,
            promotions: 0,
            checks: 0,
            checkmates: 0,
        }
    }

    pub fn add_normal(&mut self, mov: Move) {
        self.nodes += 1;
        if mov.take.is_some() {
            self.captures += 1;
        }
        match mov.move_type {
            MoveType::EnPassant => {
                self.en_passant += 1;
                self.captures += 1;
            }
            MoveType::LongCastle | MoveType::ShortCastle => self.castles += 1,
            MoveType::Promotion(_) => self.promotions += 1,
            _ => {}
        }
    }

    pub fn add_check(&mut self) {
        self.checks += 1
    }
    pub fn add_checkmate(&mut self) {
        self.checks += 1;
        self.checkmates += 1
    }
}
