#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    for i in 1..std::u64::MAX {
        let mut digest_input = input.clone();
        digest_input.push_str(&i.to_string());
        let digest = md5::compute(&digest_input);
        let digest_string = format!("{digest:x}");

        if (part_2_enable && !digest_string.starts_with("000000"))
            || !digest_string.starts_with("00000")
        {
            continue;
        }

        println!("{: >40} => {}", "Secret Key", digest_input);
        println!("{: >40} => {}", "MD5 Digest", digest_string);
        break;
    }
}
