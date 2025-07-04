use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct Framebuffer {
    terminate_sender: mpsc::Sender<bool>,
    handle: thread::JoinHandle<()>,
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        let (terminate_sender, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut counter: u32 = 0;
            loop {
                println!("hi number {} from the spawned thread!", counter);
                counter += 1;
                thread::sleep(Duration::from_millis(100));
                match rx.try_recv() {
                    Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        continue;
                    }
                }
            }
        });
        Framebuffer {
            terminate_sender,
            handle,
        }
    }

    pub fn terminate_wait(self) {
        self.terminate_sender.send(true).unwrap();
        self.handle.join().unwrap();
    }
}
