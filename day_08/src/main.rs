#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn part1(inputs: &str) {
    let mut cumulative_literals_length = 0usize;
    let mut cumulative_characters_length = 0usize;
    for input in inputs.lines() {
        cumulative_literals_length += input.len();

        let value = input[1..input.len() - 1].to_string();
        let mut iter = value.chars();

        let mut parse_string = String::new();
        while let Some(c) = iter.next() {
            if c == '\\' {
                let next = iter.next().unwrap();
                if next == 'x' {
                    let c1 = iter.next().unwrap();
                    let c2 = iter.next().unwrap();
                    let co = u8::from_str_radix(&format!("{c1}{c2}"), 16).unwrap();
                    if co > 32 && co < 127 {
                        parse_string.push(co as char);
                        continue;
                    }
                }

                parse_string.push(next);
                continue;
            }
            parse_string.push(c);
        }

        cumulative_characters_length += parse_string.len();
    }
    println!(
        "{: >40} => {}",
        "Cumulative literals length", cumulative_literals_length
    );

    println!(
        "{: >40} => {}",
        "Cumulative characters length", cumulative_characters_length
    );

    println!(
        "{: >40} => {}",
        "Answer",
        cumulative_literals_length - cumulative_characters_length
    );
}

fn part2(inputs: &str) {
    let mut cumulative_literals_length = 0usize;
    let mut cumulative_new_literals_length = 0usize;
    for input in inputs.lines() {
        cumulative_literals_length += input.len();

        let value = input[0..input.len()].to_string();

        let mut parse_string = String::from('"');
        for c in value.chars() {
            match c {
                '\\' | '"' => parse_string.push('\\'),
                _ => (),
            }
            parse_string.push(c);
        }

        parse_string.push('"');
        cumulative_new_literals_length += parse_string.len();
    }
    println!(
        "{: >40} => {}",
        "Cumulative new literals length", cumulative_new_literals_length
    );

    println!(
        "{: >40} => {}",
        "Cumulative literals length", cumulative_literals_length
    );

    println!(
        "{: >40} => {}",
        "Answer",
        cumulative_new_literals_length - cumulative_literals_length
    );
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    if part_2_enable {
        part2(&inputs);
    } else {
        part1(&inputs);
    }
}
