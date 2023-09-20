#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let _part_2_enable = env::args().nth(2).or(None).is_some();
    let _input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");
}
