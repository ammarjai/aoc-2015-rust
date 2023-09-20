#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut house_visits = HashMap::new();
    house_visits.insert("0,0".to_string(), 1);

    let mut first_pos_x = 0;
    let mut first_pos_y = 0;
    let mut second_pos_x = 0;
    let mut second_pos_y = 0;

    for (index, char) in input.chars().enumerate() {
        match char {
            '^' => {
                if part_2_enable && (index % 2) == 0 {
                    second_pos_y += 1;
                } else {
                    first_pos_y += 1;
                }
            }
            '>' => {
                if part_2_enable && (index % 2) == 0 {
                    second_pos_x += 1;
                } else {
                    first_pos_x += 1;
                }
            }
            'v' => {
                if part_2_enable && (index % 2) == 0 {
                    second_pos_y -= 1;
                } else {
                    first_pos_y -= 1;
                }
            }
            '<' => {
                if part_2_enable && (index % 2) == 0 {
                    second_pos_x -= 1;
                } else {
                    first_pos_x -= 1;
                }
            }
            _ => (),
        }

        if part_2_enable && (index % 2) == 0 {
            let key = format!("{second_pos_x},{second_pos_y}");
            match house_visits.get(&key) {
                Some(visits) => {
                    house_visits.insert(key, *visits + 1);
                }
                None => {
                    house_visits.insert(key, 1);
                }
            }
            continue;
        }

        let key = format!("{first_pos_x},{first_pos_y}");
        match house_visits.get(&key) {
            Some(visits) => {
                house_visits.insert(key, *visits + 1);
            }
            None => {
                house_visits.insert(key, 1);
            }
        }
    }

    let mut at_least_one_visit = 0;
    for value in house_visits.values() {
        if *value >= 1 {
            at_least_one_visit += 1;
        }
    }

    println!("{: >40} => {}", "At least 1 present", at_least_one_visit);
}
