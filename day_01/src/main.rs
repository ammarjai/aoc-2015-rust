#![warn(clippy::pedantic)]

use std::fs;

fn main() {
    let input_file_path = std::env::args()
        .nth(1)
        .expect("No file path argument provided!");
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let up_count: Vec<&str> = input.matches('(').collect();
    let down_count: Vec<&str> = input.matches(')').collect();
    println!(
        "{: >40} => {}",
        "Final floor",
        i32::try_from(up_count.len()).unwrap() - i32::try_from(down_count.len()).unwrap(),
    );

    let mut count = 0;
    for (index, char) in input.chars().enumerate() {
        count = match char {
            '(' => count + 1,
            ')' => count - 1,
            _ => count,
        };

        if count == -1 {
            println!(
                "{: >40} => {}",
                "Position when first reach -1 floor",
                index + 1
            );
            break;
        }
    }
}
