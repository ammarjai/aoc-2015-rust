#![warn(clippy::pedantic)]

use std::env;

fn gifts(multi: usize, min_presents: usize, limit: usize) -> usize {
    let max_elves = min_presents / multi;
    let mut house_number = max_elves;
    let mut houses = vec![0; max_elves];
    for i in 1..max_elves {
        for (count, j) in (i..max_elves).step_by(i).enumerate() {
            if limit > 0 && count == limit {
                break;
            }

            houses[j] += i * multi;
            if houses[j] >= min_presents && j < house_number {
                house_number = j;
            }
        }
    }
    house_number
}

fn main() {
    let min_presents = env::args()
        .nth(1)
        .expect("Missing total presents argument!")
        .parse()
        .unwrap();

    println!("Lowest house number: {}", gifts(10, min_presents, 0)); // 776_160
    println!("Lowest house number: {}", gifts(11, min_presents, 50)); // 786_240
}
