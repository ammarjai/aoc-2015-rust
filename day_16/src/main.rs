#![warn(clippy::pedantic)]

use regex::{Captures, Regex};
use std::env;
use std::fs;

#[derive(Debug)]
struct Sue {
    _number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
    scores: usize,
}

impl Sue {
    fn from_captures_incomplete(cap: &Captures) -> Self {
        Sue {
            _number: cap.name("number").unwrap().as_str().parse().unwrap(),
            children: Sue::extract_capture_group(cap, "children"),
            cats: Sue::extract_capture_group(cap, "cats"),
            samoyeds: Sue::extract_capture_group(cap, "samoyeds"),
            pomeranians: Sue::extract_capture_group(cap, "pomeranians"),
            akitas: Sue::extract_capture_group(cap, "akitas"),
            vizslas: Sue::extract_capture_group(cap, "vizslas"),
            goldfish: Sue::extract_capture_group(cap, "goldfish"),
            trees: Sue::extract_capture_group(cap, "trees"),
            cars: Sue::extract_capture_group(cap, "cars"),
            perfumes: Sue::extract_capture_group(cap, "perfumes"),
            scores: 0,
        }
    }

    fn extract_capture_group(cap: &Captures, name: &str) -> Option<usize> {
        for i in 1..4 {
            if cap.name(&format!("cat{i}")).unwrap().as_str() == name {
                return cap.name(&format!("val{i}")).unwrap().as_str().parse().ok();
            }
        }
        None
    }

    fn compare_part1(&self, target: &Sue) -> usize {
        let mut score = 0;
        if self.children == target.children {
            score += 1;
        }
        if self.cats == target.cats {
            score += 1;
        }
        if self.samoyeds == target.samoyeds {
            score += 1;
        }
        if self.pomeranians == target.pomeranians {
            score += 1;
        }
        if self.akitas == target.akitas {
            score += 1;
        }
        if self.vizslas == target.vizslas {
            score += 1;
        }
        if self.goldfish == target.goldfish {
            score += 1;
        }
        if self.trees == target.trees {
            score += 1;
        }
        if self.cars == target.cars {
            score += 1;
        }
        if self.perfumes == target.perfumes {
            score += 1;
        }

        score
    }

    fn compare_part2(&self, target: &Sue) -> usize {
        let mut score = 0;
        if self.children == target.children {
            score += 1;
        }
        if self.samoyeds == target.samoyeds {
            score += 1;
        }
        if self.akitas == target.akitas {
            score += 1;
        }
        if self.vizslas == target.vizslas {
            score += 1;
        }
        if self.cars == target.cars {
            score += 1;
        }
        if self.perfumes == target.perfumes {
            score += 1;
        }
        if target.cats.is_some() && self.cats < target.cats {
            score += 1;
        }
        if target.trees.is_some() && self.trees < target.trees {
            score += 1;
        }
        if target.pomeranians.is_some() && self.pomeranians > target.pomeranians {
            score += 1;
        }
        if target.goldfish.is_some() && self.goldfish > target.goldfish {
            score += 1;
        }

        score
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let target_sue = Sue {
        _number: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
        scores: 0,
    };

    let re = Regex::new(r"(?:\w+) (?<number>\d+): (?<cat1>\w+): (?<val1>\d+), (?<cat2>\w+): (?<val2>\d+), (?<cat3>\w+): (?<val3>\d+)").unwrap();
    let mut sues: Vec<Sue> = Vec::new();
    for line in inputs.lines() {
        let cap = re.captures(line).unwrap();
        sues.push(Sue::from_captures_incomplete(&cap));
    }

    if part_2_enable {
        for sue in &mut sues {
            sue.scores = target_sue.compare_part2(sue);
        }
    } else {
        for sue in &mut sues {
            sue.scores = target_sue.compare_part1(sue);
        }
    }

    let actual_sue = sues.iter().max_by_key(|f| f.scores).unwrap();
    println!("Real Aunt Sue: {actual_sue:#?}");
}
