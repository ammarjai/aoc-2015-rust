#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut total_square_feet_of_wrapping_paper = 0;
    let mut total_feet_of_ribbon = 0;
    for input in inputs.lines() {
        let dimensions: Vec<&str> = input.split('x').collect();
        let [length, width, height] = <[&str; 3]>::try_from(dimensions).ok().unwrap();
        let length: i32 = length.parse().unwrap();
        let width: i32 = width.parse().unwrap();
        let height: i32 = height.parse().unwrap();

        let area1 = length * width;
        let area2 = width * height;
        let area3 = height * length;
        let min_area = **[&area1, &area2, &area3].iter().min().ok_or(0).unwrap();
        let square_feet_of_wrapping_paper = (2 * area1) + (2 * area2) + (2 * area3) + min_area;
        total_square_feet_of_wrapping_paper += square_feet_of_wrapping_paper;

        let volume = length * width * height;
        let mut area_sorted = [length, width, height];
        area_sorted.sort_unstable();
        let feet_of_ribbon = volume + (area_sorted[0] * 2) + (area_sorted[1] * 2);
        total_feet_of_ribbon += feet_of_ribbon;
    }

    println!(
        "{: >40} => {}",
        "total square feet of wrapping paper", total_square_feet_of_wrapping_paper
    );
    println!(
        "{: >40} => {}",
        "total feet of ribbon", total_feet_of_ribbon
    );
}
