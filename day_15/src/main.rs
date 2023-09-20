#![warn(clippy::pedantic)]

use regex::{Captures, Regex};
use std::env;
use std::fs;

fn generate_all_allowed_permutations(
    n: usize,
    total: usize,
    count: i32,
    output: &mut Vec<Vec<i32>>,
) {
    if let Some(last) = output.last_mut() {
        if last.len() < n - 2 {
            last.push(count);
        } else {
            let next = last[0..n - 2].to_vec();
            last.push(count);
            last.push(i32::try_from(total).unwrap() - last.iter().sum::<i32>());
            if *last.last().unwrap() < 0 {
                output.pop();
            }
            output.push(next);
            return;
        }
    } else {
        output.push(Vec::new());
    }

    for i in 0..=total {
        generate_all_allowed_permutations(n, total, i32::try_from(i).unwrap(), output);
    }

    if let Some(last) = output.last_mut() {
        if last.is_empty() {
            output.pop();
        } else {
            last.pop();
        }
    }
}

fn score(ingredients: &[Ingredient], amounts: &Vec<i32>, calories: i32) -> i32 {
    let total_calories: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.calories * a)
        .sum();
    if calories > 0 && total_calories != calories {
        return 0;
    }

    let mut total_capacity: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.capacity * a)
        .sum();
    if total_capacity < 0 {
        total_capacity = 0;
    }

    let mut total_durability: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.durability * a)
        .sum();
    if total_durability < 0 {
        total_durability = 0;
    }

    let mut total_flavor: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.flavor * a)
        .sum();
    if total_flavor < 0 {
        total_flavor = 0;
    }

    let mut total_texture: i32 = ingredients
        .iter()
        .zip(amounts)
        .map(|(i, a)| i.texture * a)
        .sum();
    if total_texture < 0 {
        total_texture = 0;
    }

    total_capacity * total_durability * total_flavor * total_texture
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn from_captures(caps: &Captures) -> Self {
        Ingredient {
            capacity: caps.name("capacity").unwrap().as_str().parse().unwrap(),
            durability: caps.name("durability").unwrap().as_str().parse().unwrap(),
            flavor: caps.name("flavor").unwrap().as_str().parse().unwrap(),
            texture: caps.name("texture").unwrap().as_str().parse().unwrap(),
            calories: caps.name("calories").unwrap().as_str().parse().unwrap(),
        }
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let total_teaspoons = env::args()
        .nth(2)
        .expect("Missing total teaspoons!")
        .parse()
        .unwrap();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let re = Regex::new(
        r"(?:\w+): capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>-?\d+)",
    ).unwrap();
    let mut ingredients: Vec<Ingredient> = Vec::new();
    for input in inputs.lines() {
        let caps = re.captures(input).unwrap();
        ingredients.push(Ingredient::from_captures(&caps));
    }

    let mut amounts = Vec::new();
    generate_all_allowed_permutations(ingredients.len(), total_teaspoons, 0, &mut amounts);

    let max_score = amounts
        .iter()
        .map(|amt| score(&ingredients, amt, 0))
        .max()
        .unwrap();

    println!("max score: {max_score}");

    let max_score = amounts
        .iter()
        .map(|amt| score(&ingredients, amt, 500))
        .max()
        .unwrap();

    println!("max score with calories 500: {max_score}");
}
