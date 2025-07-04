use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Framebuffer {
    terminate: mpsc::Sender<bool>,
    thread: thread::JoinHandle<()>,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        let (tx, rx) = mpsc::channel();
        let thread = thread::spawn(move || {
            let mut counter: u32 = 0;
            loop {
                println!("hi number {} from the spawned thread!", counter);
                counter += 1;
                thread::sleep(Duration::from_millis(100));
                match rx.try_recv() {
                    Ok(_) => break,
                    Err(mpsc::TryRecvError::Disconnected) => break,
                    Err(mpsc::TryRecvError::Empty) => continue,
                }
            }
        });
        Framebuffer {
            terminate: tx,
            thread: thread,
        }
    }

    pub fn terminate_wait(self) {
        self.terminate.send(true).unwrap();
        self.thread.join().unwrap();
    }
}
