#![warn(clippy::pedantic)]

use queues::IsQueue;
use queues::Queue;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(PartialEq, Eq)]
enum Op {
    Assign,
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}

fn extract_op_and_output<'a>(commands: &'a Vec<&str>) -> Result<(Op, String), &'a str> {
    match commands.len() {
        3 => Ok((Op::Assign, String::from(commands[2]))),
        4 => Ok((Op::Not, String::from(commands[3]))),
        5 => {
            let output = String::from(commands[4]);
            match *commands.get(1).unwrap() {
                "AND" => Ok((Op::And, output)),
                "OR" => Ok((Op::Or, output)),
                "LSHIFT" => Ok((Op::Lshift, output)),
                "RSHIFT" => Ok((Op::Rshift, output)),
                _ => Err("Unsupported operation!"),
            }
        }
        _ => Err("Unsupported operation!"),
    }
}

fn to_defer(operation: &Op, commands: &[&str], outputs: &HashMap<String, u16>) -> bool {
    // println!("{:#?}", commands);
    match operation {
        Op::Assign => {
            let left = outputs.get(commands[0]);
            if commands[0].parse::<u16>().is_err() && left.is_none() {
                return true;
            }
        }
        Op::Not => {
            if outputs.get(commands[1]).is_none() {
                return true;
            }
        }
        Op::And | Op::Or => {
            let left = outputs.get(commands[0]);
            let right = outputs.get(commands[2]);
            if (commands[0].parse::<u16>().is_ok() && right.is_none())
                || (commands[2].parse::<u16>().is_ok() && left.is_none())
                || (commands[0].parse::<u16>().is_err()
                    && commands[2].parse::<u16>().is_err()
                    && (left.is_none() || right.is_none()))
            {
                return true;
            }
        }
        Op::Lshift | Op::Rshift => {
            if outputs.get(commands[0]).is_none() {
                return true;
            }
        }
    };

    false
}

fn get_gate_output(operation: &Op, commands: &[&str], outputs: &HashMap<String, u16>) -> u16 {
    match operation {
        Op::Assign => {
            if commands[0].parse::<u16>().is_ok() {
                commands[0].parse::<u16>().unwrap()
            } else {
                *outputs.get(commands[0]).unwrap()
            }
        }
        Op::Not => !*outputs.get(commands[1]).unwrap(),
        Op::And => {
            if commands[0].parse::<u16>().is_ok() {
                let left = commands[0].parse::<u16>().unwrap();
                left & outputs.get(commands[2]).unwrap()
            } else if commands[2].parse::<u16>().is_ok() {
                let right = commands[2].parse::<u16>().unwrap();
                outputs.get(commands[0]).unwrap() & right
            } else {
                outputs.get(commands[0]).unwrap() & outputs.get(commands[2]).unwrap()
            }
        }
        Op::Or => {
            if commands[0].parse::<u16>().is_ok() {
                let left = commands[0].parse::<u16>().unwrap();
                left | outputs.get(commands[2]).unwrap()
            } else if commands[2].parse::<u16>().is_ok() {
                let right = commands[2].parse::<u16>().unwrap();
                outputs.get(commands[0]).unwrap() | right
            } else {
                outputs.get(commands[0]).unwrap() | outputs.get(commands[2]).unwrap()
            }
        }
        Op::Lshift => {
            let right = commands[2].parse::<u16>().unwrap();
            outputs.get(commands[0]).unwrap() << right
        }
        Op::Rshift => {
            let right = commands[2].parse::<u16>().unwrap();
            outputs.get(commands[0]).unwrap() >> right
        }
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut outputs: HashMap<String, u16> = HashMap::new();
    let mut defer: Queue<Vec<&str>> = Queue::new();
    for input in inputs.lines() {
        let commands = input.split(' ').collect::<Vec<&str>>();
        let (operation, output) = extract_op_and_output(&commands).unwrap();

        let skip = to_defer(&operation, &commands, &outputs);
        if skip {
            defer.add(commands).unwrap();
            continue;
        }

        let gate_output = get_gate_output(&operation, &commands, &outputs);
        outputs.insert(output, gate_output);
    }

    while let Ok(commands) = defer.remove() {
        let (operation, output) = extract_op_and_output(&commands).unwrap();

        let skip = to_defer(&operation, &commands, &outputs);
        if skip {
            defer.add(commands).unwrap();
            continue;
        }

        let gate_output = get_gate_output(&operation, &commands, &outputs);
        outputs.insert(output, gate_output);
    }

    let a_val = *outputs.get("a").unwrap();
    outputs.clear();

    for input in inputs.lines() {
        let commands = input.split(' ').collect::<Vec<&str>>();
        let (operation, output) = extract_op_and_output(&commands).unwrap();

        if output == "b" {
            outputs.insert(output, a_val);
            continue;
        }

        let skip = to_defer(&operation, &commands, &outputs);
        if skip {
            defer.add(commands).unwrap();
            continue;
        }

        let gate_output = get_gate_output(&operation, &commands, &outputs);
        outputs.insert(output, gate_output);
    }

    while let Ok(commands) = defer.remove() {
        let (operation, output) = extract_op_and_output(&commands).unwrap();

        if output == "b" {
            outputs.insert(output, a_val);
            continue;
        }

        let skip = to_defer(&operation, &commands, &outputs);
        if skip {
            defer.add(commands).unwrap();
            continue;
        }

        let gate_output = get_gate_output(&operation, &commands, &outputs);
        outputs.insert(output, gate_output);
    }

    // println!("{:#?}", outputs);
    println!("{: >40} => {}", "Output for a (part 1)", a_val);
    println!(
        "{: >40} => {}",
        "Output for a (part 2)",
        outputs.get("a").unwrap()
    );
}
