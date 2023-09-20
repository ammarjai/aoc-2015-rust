#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn update_light_state(light_grid: &mut [Vec<u8>], sticky_corner: bool) {
    let clone = light_grid.to_vec();
    for i in 0..light_grid.len() {
        for j in 0..light_grid[i].len() {
            if sticky_corner
                && ((i == j && (i == 0 || i == light_grid.len() - 1))
                    || (i == 0 && j == light_grid[i].len() - 1)
                    || (j == 0 && i == light_grid.len() - 1))
            {
                continue;
            }

            let a: usize = (i > 0).into();
            let b: usize = (j > 0).into();
            let on_sum = &clone
                .iter()
                .skip(i - a)
                .take(2 + a)
                .map(|f| f.iter().skip(j - b).take(2 + b).sum::<u8>())
                .sum::<u8>();

            match clone[i].get(j).unwrap() {
                1 => {
                    let sum = on_sum - 1;
                    if !(2..=3).contains(&sum) {
                        light_grid[i][j] = 0;
                    }
                }
                0 if *on_sum == 3 => {
                    light_grid[i][j] = 1;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let total_steps = env::args().nth(2).expect("Missing total steps argument!");
    let part_2_enable = env::args().nth(3).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut light_grid: Vec<Vec<u8>> = Vec::new();
    for line in inputs.lines() {
        light_grid.push(line.chars().map(|f| (f == '#').into()).collect());
    }

    for _ in 0..total_steps.parse().unwrap() {
        update_light_state(&mut light_grid, part_2_enable);
    }

    let total_lights_on = light_grid
        .iter()
        .map(|f| bytecount::count(f, 1u8))
        .sum::<usize>();
    println!("Total lights turned on: {total_lights_on}");
}
