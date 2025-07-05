use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{TryRecvError::Disconnected, TryRecvError::Empty};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;

pub struct Framebuffer {
    terminate: mpsc::Sender<bool>,
    thread: thread::JoinHandle<()>,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        let (tx, rx) = mpsc::channel();
        Framebuffer {
            terminate: tx,
            thread: thread::spawn(move || {
                let mut counter = 0;
                let mut stdout = std::io::stdout().into_raw_mode().unwrap();
                write!(stdout, "{}", termion::clear::All).unwrap();
                loop {
                    write!(
                        stdout,
                        "{}{}{} hello",
                        termion::clear::CurrentLine,
                        termion::cursor::Goto(1, 1),
                        counter
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    counter += 1;

                    thread::sleep(Duration::from_millis(100));
                    match rx.try_recv() {
                        Ok(_) => break,
                        Err(Disconnected) => break,
                        Err(Empty) => continue,
                    }
                }
            }),
        }
    }

    pub fn terminate_wait(self) {
        self.terminate.send(true).unwrap();
        self.thread.join().unwrap();
    }
}
