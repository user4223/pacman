use rand::Rng;
use std::io;

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

const TOP: char = '\u{2500}';

fn main() {
    println!("Input: {TOP} {} {}", generate_number(), read_line());
}
