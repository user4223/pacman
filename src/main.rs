use rand::Rng;
use std::sync::{Arc, atomic::AtomicI64, atomic::Ordering};
use std::{thread, time::Duration};
use termion::event::Key;

#[path = "pacman/framebuffer.rs"]
mod framebuffer;
#[path = "pacman/keycontroller.rs"]
mod keycontroller;

fn generate_number() -> i32 {
    rand::rng().random_range(1..=100)
}

const HORIZONTAL: char = '\u{2501}';
const VERTICAL: char = '\u{2503}';

struct Character {
    pool: std::vec::Vec<char>,
    index: Arc<AtomicI64>,
}

impl Character {
    pub fn new() -> Character {
        Character {
            pool: std::vec![HORIZONTAL, VERTICAL],
            index: Arc::new(AtomicI64::new(0)),
        }
    }

    pub fn increment(&mut self, _: Key) {
        if self.index.fetch_add(1, Ordering::Relaxed) >= 2 {
            self.index.store(0, Ordering::Relaxed);
        };
    }
}

fn main() {
    let character = Character::new();
    let key_controller = keycontroller::KeyController::new(|_key| println!("{}", 23));
    let frame_buffer = framebuffer::Framebuffer::new();

    let line_length: i32 = 100;
    let stroke_length: i32 = 5;
    let mut offset: i32 = 0;
    let mut increment: i32 = 1;
    for i in 0..1000 {
        frame_buffer.update(format!(
            "{}{}{}",
            String::from(" ").repeat(offset as usize),
            String::from(HORIZONTAL).repeat(stroke_length as usize),
            String::from(" ").repeat((line_length - offset - stroke_length) as usize)
        ));
        thread::sleep(Duration::from_millis(25));
        offset += increment;
        if offset + stroke_length >= line_length {
            increment = -1;
        }
        if offset <= 0 {
            increment = 1;
        }
    }

    frame_buffer.stop();
    key_controller.stop();
}
