use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{TryRecvError::Disconnected, TryRecvError::Empty};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub struct Framebuffer {
    thread: thread::JoinHandle<()>,
    output_sender: mpsc::Sender<String>,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        let (output_sender, output_receiver) = mpsc::channel();
        Framebuffer {
            output_sender,
            thread: thread::spawn(move || {
                let goto_start = cursor::Goto(1, 1);
                let mut output: Option<String> = None;
                let mut stdout = std::io::stdout().into_raw_mode().unwrap();
                write!(stdout, "{}{}", clear::All, goto_start).unwrap();
                loop {
                    match output.clone() {
                        None => {
                            write!(stdout, "{}", clear::CurrentLine).unwrap();
                        }
                        Some(v) => {
                            write!(stdout, "{}{}{}", clear::CurrentLine, goto_start, v).unwrap()
                        }
                    }
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(40));
                    match output_receiver.try_recv() {
                        Ok(v) => {
                            output = match v.is_empty() {
                                true => None,
                                false => Some(v),
                            }
                        }
                        Err(Disconnected) => break,
                        Err(Empty) => continue,
                    }
                }
                write!(stdout, "{}{}", clear::All, goto_start).unwrap();
            }),
        }
    }

    pub fn update(&self, line: String) {
        self.output_sender.send(line).unwrap();
    }

    pub fn clear(&self) {
        self.output_sender.send(String::from("")).unwrap();
    }

    pub fn stop(self) {
        drop(self.output_sender);
        self.thread.join().unwrap();
    }
}
