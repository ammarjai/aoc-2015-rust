#![warn(clippy::pedantic)]

use std::env;
use std::fs;

// reference https://www.guru99.com/print-all-possible-combinations.html
// highly modified
fn find_all_allowed_combinations(
    input: &[usize],
    output: &mut Vec<Vec<usize>>,
    start: usize,
    end: usize,
    index: usize,
    r: usize,
    filter_equal: usize,
) {
    if let Some(last) = output.last() {
        if index == r {
            let next = last[0..last.len() - 1].to_vec();
            if filter_equal > 0 && last.iter().sum::<usize>() != filter_equal {
                output.pop();
            }
            output.push(next);
            return;
        }
    } else {
        output.push(Vec::with_capacity(r));
    }

    let mut i = start;
    while i <= end && end - i + 1 > r - index {
        let v = output.last_mut().unwrap();
        v.push(input[i]);
        find_all_allowed_combinations(input, output, i + 1, end, index + 1, r, filter_equal);
        i += 1;
    }

    if let Some(last) = output.last_mut() {
        if last.is_empty() {
            output.pop();
        } else {
            last.pop();
        }
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let total = env::args()
        .nth(2)
        .expect("Missing total liters of eggnogs argument!");
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let containers: Vec<usize> = inputs.lines().map(|f| f.parse().unwrap()).collect();
    let mut output: Vec<Vec<usize>> = Vec::new();
    for r in 2..=containers.len() {
        output.push(Vec::with_capacity(r));
        find_all_allowed_combinations(
            &containers,
            &mut output,
            0,
            containers.len(),
            0,
            r,
            total.parse().unwrap(),
        );
    }
    // println!("Combinations: {output:#?}");
    println!("Total combinations: {}", output.len());

    let minimum_num_of_containers = output.iter().min_by_key(|&f| f.len()).unwrap();
    let minimum_containers = output
        .iter()
        .filter(|&f| f.len() == minimum_num_of_containers.len())
        .count();
    println!("Total combinations for minimum number of containers: {minimum_containers}");
}
