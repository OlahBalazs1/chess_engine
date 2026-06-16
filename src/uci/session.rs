use crate::{engine::Bot, uci::stream::UciStream};

pub struct UciSession {
    stream: UciStream,
    game: Option<Bot>,

    uci_received: bool,
}

impl UciSession {
    pub fn new(stream: UciStream) -> Self {
        Self {
            stream,
            game: None,
            uci_received: false,
        }
    }
}
