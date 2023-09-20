#![warn(clippy::pedantic)]

use json::JsonValue;
use std::env;
use std::fs;

fn has_red(o: &JsonValue) -> bool {
    o.entries()
        .any(|(_, v)| v.is_string() && v.as_str().unwrap().eq_ignore_ascii_case("red"))
}

fn count_object(o: &JsonValue, ignore_red: bool) -> i32 {
    match o {
        JsonValue::Object(_) => {
            if ignore_red && has_red(o) {
                return 0;
            }

            o.entries()
                .map(|(_, v)| count_object(v, ignore_red))
                .sum::<i32>()
        }
        JsonValue::Array(_) => o
            .members()
            .map(|v| count_object(v, ignore_red))
            .sum::<i32>(),
        JsonValue::Number(_) => o.as_i32().unwrap(),
        _ => 0,
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let _part_2_enable = env::args().nth(2).or(None).is_some();
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let parsed = json::parse(&input).unwrap();
    println!("sum: {}", count_object(&parsed, false));
    println!("sum w/o red: {}", count_object(&parsed, true));
}
