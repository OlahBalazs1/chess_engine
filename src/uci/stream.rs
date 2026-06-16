use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Write, stdin},
    sync::{Arc, Mutex},
    thread,
};

use crate::uci::command::UciCommand;

pub struct UciStream {
    inner: Arc<Mutex<VecDeque<UciCommand>>>,
    stdout: std::io::Stdout,
    stop_signal: Arc<Mutex<bool>>,
}

impl UciStream {
    pub fn new() -> Self {
        let stream = Self {
            inner: Arc::new(Mutex::new(VecDeque::new())),
            stdout: std::io::stdout(),
            stop_signal: Arc::new(Mutex::new(false)),
        };
        stream.start_listener();

        stream
    }

    pub fn send(&self, command: UciCommand) {
        let mut stdout = self.stdout.lock();
        writeln!(stdout, "{}", command.to_string()).ok();
    }

    pub fn try_read(&self) -> Option<UciCommand> {
        let mut queue = self.inner.lock().unwrap();
        queue.pop_front()
    }

    pub fn is_listening(&self) -> bool {
        !*self.stop_signal.lock().unwrap()
    }

    fn start_listener(&self) {
        let queue = Arc::clone(&self.inner);
        let stop_signal = Arc::clone(&self.stop_signal);
        thread::spawn(move || {
            let mut stdin = BufReader::new(stdin());
            while !*stop_signal.lock().unwrap() {
                let mut buf = String::new();
                stdin.read_line(&mut buf).ok();

                if let Some(command) = UciCommand::parse(&buf) {
                    queue.lock().unwrap().push_back(command);
                }
            }
        });
    }
}
