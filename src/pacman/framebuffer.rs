use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{TryRecvError::Disconnected, TryRecvError::Empty};
use std::thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub struct Line {
    number: u16, // 0 means invalid since numbering starts with 1
    payload: String,
}

impl Line {
    pub fn is_valid(&self) -> bool {
        if self.number == 0 {
            return false;
        }
        return true;
    }
}

pub struct Framebuffer {
    thread: thread::JoinHandle<()>,
    output_sender: mpsc::Sender<Line>,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        let (output_sender, output_receiver) = mpsc::channel();
        Framebuffer {
            output_sender,
            thread: thread::spawn(move || {
                let goto_start = cursor::Goto(1, 1);
                let mut output: Line = Line {
                    number: 0,
                    payload: String::from(""),
                };
                let mut stdout = std::io::stdout().lock().into_raw_mode().unwrap();
                write!(stdout, "{}{}", clear::All, goto_start).unwrap();
                loop {
                    match output.is_valid() {
                        false => {
                            write!(stdout, "{}", clear::CurrentLine).unwrap();
                        }
                        true => write!(
                            stdout,
                            "{}{}{}",
                            clear::CurrentLine,
                            goto_start,
                            output.payload
                        )
                        .unwrap(),
                    }
                    stdout.flush().unwrap();
                    thread::sleep(Duration::from_millis(40));
                    match output_receiver.try_recv() {
                        Ok(v) => output = v,
                        Err(Disconnected) => break,
                        Err(Empty) => continue,
                    }
                }
                write!(stdout, "{}{}", clear::All, goto_start).unwrap();
            }),
        }
    }

    pub fn update(&self, line: String) {
        self.output_sender
            .send(Line {
                number: 1,
                payload: line,
            })
            .unwrap();
    }

    pub fn clear(&self) {
        self.output_sender
            .send(Line {
                number: 0,
                payload: String::from(""),
            })
            .unwrap();
    }

    pub fn stop(self) {
        drop(self.output_sender);
        self.thread.join().unwrap();
    }
}
