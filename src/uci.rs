use crate::{
    moving::{Move, MoveType},
    piece::PieceType,
};

impl Move {
    fn as_uci(&self) -> String {
        let mut output = String::new();

        if self.piece_type() != PieceType::Pawn {
            output.push(self.piece_type().as_char());
        }

        output.push_str(&format!("{}", self.from));
        output.push(if self.take.is_some() { 'x' } else { '-' });
        output.push_str(&format!("{}", self.to));

        if let Some(promoted_to) = self.promote_to() {
            output.push(promoted_to.as_char());
        }

        output
    }
}

fn uciok() {
    println!("uciok")
}
fn engine_introduction(name: &str, author: &str) {
    println!("id name {}", name);
    println!("id author {}", author);
    uciok();
}

fn send_best_move(mov: Move) {
    println!("bestmove {}", mov.as_uci())
}
