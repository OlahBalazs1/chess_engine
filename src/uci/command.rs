use crate::moving::Move;
use std::fmt::{Display, Write};

pub enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption {
        name: String,
        value: Option<String>,
    },
    UciNewGame,
    Position {
        position: UciPosition,
        moves: Vec<String>,
    },
    GoDepth(u32),
    Id {
        name: String,
        author: String,
    },
    BestMove(Move),
    UciOk,
    ReadyOk,
    Stop,
    Quit,
}

pub enum UciPosition {
    StartPos,
    Fen(String),
}

impl UciCommand {
    pub fn parse(command: &str) -> Option<Self> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts[0] {
            "uci" => Some(UciCommand::Uci),
            "uciok" => Some(UciCommand::UciOk),
            "readyok" => Some(UciCommand::ReadyOk),
            "stop" => Some(UciCommand::Stop),
            "quit" => Some(UciCommand::Quit),
            "isready" => Some(UciCommand::IsReady),
            "ucinewgame" => Some(UciCommand::UciNewGame),
            "debug" => Some(UciCommand::Debug(parts[1] == "on")),
            "setoption" => {
                if parts[1] != "name" {
                    return None;
                }
                let name = parts[2];
                if parts[3] != "value" {
                    return None;
                }
                let value = parts.get(4).map(|v| v.to_string());
                Some(UciCommand::SetOption {
                    name: name.to_string(),
                    value,
                })
            }
            "go" => {
                if parts[1] != "depth" {
                    return None;
                }
                let depth = parts[2].parse().ok()?;
                Some(UciCommand::GoDepth(depth))
            }
            "bestmove" => None,
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        let mut buf = String::new();
        match self {
            UciCommand::Uci => writeln!(buf, "uci").unwrap(),
            UciCommand::UciOk => writeln!(buf, "uciok").unwrap(),
            UciCommand::ReadyOk => writeln!(buf, "readyok").unwrap(),
            UciCommand::Stop => writeln!(buf, "stop").unwrap(),
            UciCommand::Quit => writeln!(buf, "quit").unwrap(),
            UciCommand::IsReady => writeln!(buf, "isready").unwrap(),
            UciCommand::UciNewGame => writeln!(buf, "ucinewgame").unwrap(),
            UciCommand::Debug(enabled) => {
                writeln!(buf, "debug {}", if *enabled { "on" } else { "off" }).unwrap()
            }
            UciCommand::SetOption { name, value } => {
                write!(buf, "setoption name {}", name,).unwrap();
                if let Some(value) = value {
                    write!(buf, "value {}", value).unwrap();
                }
                writeln!(buf).unwrap();
            }
            UciCommand::GoDepth(depth) => writeln!(buf, "go depth {}", depth).unwrap(),
            UciCommand::Id { name, author } => {
                writeln!(buf, "id name {}", name).unwrap();
                writeln!(buf, "id author {}", author).unwrap();
            }
            UciCommand::BestMove(mov) => {
                writeln!(buf, "bestmove {}", mov.into_long_algebraic()).unwrap()
            }
            UciCommand::Position { position, moves } => {
                writeln!(buf, "position {}", position).unwrap();
                if !moves.is_empty() {
                    writeln!(buf, "moves {}", moves.join(" ")).unwrap();
                }
            }
        }
        buf
    }
}

impl Display for UciPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UciPosition::StartPos => write!(f, "startpos")?,
            UciPosition::Fen(fen) => write!(f, "fen {}", fen)?,
        }
        Ok(())
    }
}
