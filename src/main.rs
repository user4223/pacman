use rand::Rng;
use std::{array::from_mut, collections::btree_map::Range, io, thread, time::Duration};

#[path = "pacman/framebuffer.rs"]
mod framebuffer;
#[path = "pacman/keycontroller.rs"]
mod keycontroller;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn generate_number() -> i32 {
    rand::rng().random_range(1..=100)
}

const HORIZONTAL: char = '\u{2501}';
const VERTICAL: char = '\u{2503}';

fn draw_horizontal_line(length: u32) {
    for _i in (0..length).rev() {
        print!("{HORIZONTAL}");
    }
}

fn main() {
    draw_horizontal_line(100);

    let keycontroller = keycontroller::KeyController::new(|key| println!("{key}"));
    let framebuffer = framebuffer::Framebuffer::new();
    for i in 0..10 {
        framebuffer.update(format!("{} hello", i));
        thread::sleep(Duration::from_millis(600));
        framebuffer.clear();
        thread::sleep(Duration::from_millis(200));
    }
    framebuffer.stop();
}
