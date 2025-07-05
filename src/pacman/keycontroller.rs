use std::sync::mpsc::{self, Receiver};
use std::thread;
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct KeyController {
    thread: thread::JoinHandle<()>,
}

impl KeyController {
    pub fn new(handler: fn(char)) -> KeyController {
        KeyController {
            thread: thread::spawn(move || {
                let stdin = async_stdin();
                for c in stdin.keys() {
                    match c {
                        Ok(k) => match k {
                            Key::Left => println!("<"),
                            Key::Right => handler('>'),
                            Key::Down => handler('<'),
                            Key::Up => handler('>'),
                            Key::Char('q') => {
                                handler('0');
                                break;
                            }
                            Key::Ctrl('c') => {
                                handler('0');
                                break;
                            }
                            _ => {}
                        },
                        Err(_) => continue,
                    }
                }
            }),
        }
    }
}
