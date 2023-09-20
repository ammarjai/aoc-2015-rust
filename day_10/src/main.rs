#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn look_and_say(input: &str) -> String {
    let mut output = String::new();
    let mut current_char: Option<char> = None;
    let mut count: usize = 0;
    for c in input.chars() {
        if current_char.is_none() {
            current_char = Some(c);
            count = 1;
            continue;
        }
        let c2 = current_char.unwrap();

        if c == c2 {
            count += 1;
            continue;
        }

        output.push_str(&format!("{count}{c2}"));
        current_char = Some(c);
        count = 1;
    }
    output.push_str(&format!("{}{}", count, current_char.unwrap()));
    output
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut output = input;
    let mut max: usize = 40;
    if part_2_enable {
        max = 50;
    }

    for _ in 0..max {
        output = look_and_say(&output);
    }
    println!("Output Length: {}", output.len());
}
