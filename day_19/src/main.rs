#![warn(clippy::pedantic)]

use std::env;
use std::fs;

fn mutate(conv_table: &[(&str, &str)], input: &str, output: &mut Vec<String>) {
    for (k, v) in conv_table {
        let len = k.len();
        if len > input.len() {
            continue;
        }

        for index in 0..=input.len() - len {
            if input[index..index + len] == **k {
                let mut new_str = input.to_string();
                new_str.replace_range(index..index + len, v);
                if !output.contains(&new_str) {
                    output.push(new_str);
                }
            }
        }
    }
}

fn decompose(conv_table: &[(&str, &str)], input: &str, output: &mut Vec<String>) {
    for (k, v) in conv_table {
        let len = v.len();
        if len > input.len() {
            continue;
        }

        for index in 0..=input.len() - len {
            if input[index..index + len] == **v {
                let mut new_str = input.to_string();
                new_str.replace_range(index..index + len, k);
                if new_str.contains('e') && new_str.len() > 1 {
                    continue;
                }

                if !output.contains(&new_str) {
                    output.push(new_str);
                }
            }
        }
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut conv_table: Vec<(&str, &str)> = Vec::new();
    let mut done_fill_conv_table = false;
    let mut target_molecule: String = String::new();
    for line in inputs.lines() {
        if done_fill_conv_table {
            target_molecule.push_str(line);
            break;
        }

        if line.is_empty() {
            done_fill_conv_table = true;
            continue;
        }

        let commands: Vec<&str> = line.split(' ').collect();
        let &lhs = commands.first().unwrap();
        let &rhs = commands.last().unwrap();
        conv_table.push((lhs, rhs));
    }

    let mut output: Vec<String> = Vec::new();
    if part_2_enable {
        let mut steps = 0usize;
        output.push(target_molecule);
        loop {
            // println!("Output at steps {steps}: {output:#?}");
            if output.contains(&String::from('e')) || output.is_empty() {
                break;
            }

            let shortest_string = output.iter().min_by_key(|&f| f.len()).unwrap().clone();
            output = output
                .iter()
                .filter(|f| f.len() == shortest_string.len())
                .take(100)
                .cloned()
                .collect();

            for _ in 0..output.len() {
                let input = output.remove(0);
                decompose(&conv_table, &input, &mut output);
            }

            steps += 1;
        }

        println!("Output: {output:#?}");
        println!("Total Steps: {steps}");
    } else {
        let input_molecule: String = target_molecule.clone();
        mutate(&conv_table, &input_molecule, &mut output);
        // println!("Output: {output:#?}");
        let total_mutations = output.len();
        println!("Total Mutations: {total_mutations}");
    }
}
