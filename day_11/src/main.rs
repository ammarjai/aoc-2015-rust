#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn rule1(input: &str) -> bool {
    for (c1, c2, c3) in input[..input.len() - 2]
        .chars()
        .zip(input[1..].chars())
        .zip(input[2..].chars())
        .map(|((c1, c2), c3)| (c1 as u8, c2 as u8, c3 as u8))
    {
        if c2 - c1 == 1 && c3 - c2 == 1 {
            return true;
        }
    }
    false
}

fn rule2(input: &str) -> bool {
    !(input.contains('i') || input.contains('o') || input.contains('l'))
}

fn rule3(input: &str) -> bool {
    let mut count = 0;
    let mut chars: Vec<char> = Vec::new();
    for (c1, c2) in input[..input.len() - 1].chars().zip(input[1..].chars()) {
        if chars.contains(&c1) {
            continue;
        }
        if c1 == c2 {
            count += 1;
            chars.push(c2);
        }
    }
    if count >= 2 {
        return true;
    }
    false
}

fn increment(input: &str, index: Option<usize>) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    if let Some(i) = index {
        if chars[i] == 'z' {
            chars[i] = 'a';
            let input = String::from_iter(chars);
            increment(&input, Some(i - 1))
        } else {
            let new_char = (chars[i] as u8 + 1) as char;
            chars[i] = new_char;
            String::from_iter(chars)
        }
    } else {
        increment(input, Some(input.len() - 1))
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut next_pass = input.clone();
    for i in 1..3 {
        loop {
            next_pass = increment(&next_pass, None);
            if rule1(&next_pass) && rule2(&next_pass) && rule3(&next_pass) {
                break;
            }
        }
        println!("Output {i}: {next_pass}");
    }
}
