use std::sync::mpsc;
use std::sync::mpsc::{TryRecvError::Disconnected, TryRecvError::Empty};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::{TermRead, TermReadEventsAndRaw};
use termion::{AsyncReader, async_stdin};

pub struct KeyController {
    thread: thread::JoinHandle<()>,
    terminate_sender: mpsc::Sender<bool>,
}

impl KeyController {
    pub fn new(handler: fn(Key)) -> KeyController {
        let (terminate_sender, terminate_receiver) = mpsc::channel();
        KeyController {
            terminate_sender,
            thread: thread::spawn(move || {
                loop {
                    let stdin = async_stdin();
                    for c in stdin.keys() {
                        match c {
                            Ok(k) => match k {
                                Key::Left | Key::Right | Key::Down | Key::Up => handler(k),
                                Key::Esc | Key::Ctrl('c') | Key::Char('q') => {
                                    handler(Key::Esc);
                                    break;
                                }
                                _ => {}
                            },
                            Err(_) => continue,
                        }
                    }
                    //handler(Key::Right);
                    thread::sleep(Duration::from_millis(40));
                    match terminate_receiver.try_recv() {
                        Ok(_) | Err(Empty) => continue,
                        Err(Disconnected) => break,
                    }
                }
            }),
        }
    }

    pub fn stop(self) {
        drop(self.terminate_sender);
        self.thread.join().unwrap();
    }
}
