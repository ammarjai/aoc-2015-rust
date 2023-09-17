use std::collections::HashSet;
use std::env;
use std::fs;

const BAD_PATTERN: [&str; 4] = ["ab", "cd", "pq", "xy"];
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_nice(input: &str) -> bool {
    if BAD_PATTERN.iter().any(|&s| input.contains(s)) {
        return false;
    }

    let mut vowel_count = 0;
    let mut same_char = false;
    let mut prev_char: Option<char> = None;
    for c in input.chars() {
        if VOWEL.iter().any(|&s| c == s) {
            vowel_count += 1;
        }

        match prev_char {
            Some(a) if a == c => {
                same_char = true;
            }
            _ => (),
        }
        prev_char = Some(c);

        if vowel_count >= 3 && same_char {
            return true;
        }
    }

    false
}

fn is_nice_v2(input: &str) -> bool {
    let mut pair_repeat = false;
    let mut one_repeat = false;

    let mut pair_sets = HashSet::new();
    let mut ignore_once = false;
    let mut ignore_counter = 0;
    let mut prev_char_2: Option<char> = None;
    let mut prev_char: Option<char> = None;
    for c in input.chars() {
        if ignore_once && ignore_counter == 0 {
            ignore_counter += 1;
        } else {
            ignore_once = false;
            ignore_counter = 0;
        }

        match prev_char_2 {
            Some(a) if a == c => {
                one_repeat = true;
            }
            _ => (),
        }

        if let Some(a) = prev_char {
            let pair = format!("{}{}", a, c);
            let insert = pair_sets.insert(pair.clone());
            if insert && input.matches(&pair).collect::<Vec<&str>>().len() > 1 {
                pair_repeat = true;
            }

            prev_char_2 = Some(a);
        }
        prev_char = Some(c);
    }

    pair_repeat && one_repeat
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut nice_strings_counter = 0;
    for input in inputs.lines() {
        if part_2_enable && is_nice_v2(input) || !part_2_enable && is_nice(input) {
            nice_strings_counter += 1;
        }
    }

    println!(
        "{: >40} => {}",
        "Nice strings counter", nice_strings_counter
    );
}
