#![warn(clippy::pedantic)]

use std::env;
use std::fs;

#[derive(PartialEq, Debug)]
enum Op {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct GridOp {
    operation: Op,
    start_coordinate: [usize; 2],
    end_coordinate: [usize; 2],
}

fn update_grid_op(grid_op: &mut GridOp, op: Op, commands: &[&str]) {
    grid_op.operation = op;
    let mut index = 1;
    if grid_op.operation != Op::Toggle {
        index = 2;
    }

    let coordinate1 = commands[index].split(',').collect::<Vec<&str>>();
    grid_op.start_coordinate = [
        coordinate1[0].parse().ok().unwrap(),
        coordinate1[1].parse().ok().unwrap(),
    ];

    let coordinate2 = commands[index + 2].split(',').collect::<Vec<&str>>();
    grid_op.end_coordinate = [
        coordinate2[0].parse().ok().unwrap(),
        coordinate2[1].parse().ok().unwrap(),
    ];
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut light_grid = vec![[0; 1000]; 1000].into_boxed_slice();
    for input in inputs.lines() {
        let commands = input.split(' ').collect::<Vec<&str>>();
        let mut grid_op = GridOp {
            operation: Op::On,
            start_coordinate: [0; 2],
            end_coordinate: [0; 2],
        };

        match *commands.first().unwrap() {
            "toggle" => update_grid_op(&mut grid_op, Op::Toggle, &commands),
            "turn" => match *commands.get(1).unwrap() {
                "on" => update_grid_op(&mut grid_op, Op::On, &commands),
                "off" => update_grid_op(&mut grid_op, Op::Off, &commands),
                _ => (),
            },
            _ => (),
        }

        for grid_row in &mut light_grid[grid_op.start_coordinate[1]..=grid_op.end_coordinate[1]] {
            for grid_value in &mut grid_row[grid_op.start_coordinate[0]..=grid_op.end_coordinate[0]]
            {
                match grid_op.operation {
                    Op::Toggle => {
                        if part_2_enable {
                            *grid_value += 2;
                        } else {
                            *grid_value ^= 0b0000_0001;
                        }
                    }
                    Op::On => {
                        if part_2_enable {
                            *grid_value += 1;
                        } else {
                            *grid_value = 1;
                        }
                    }
                    Op::Off => {
                        if part_2_enable {
                            if *grid_value != 0 {
                                *grid_value -= 1;
                            }
                        } else {
                            *grid_value = 0;
                        }
                    }
                }
            }
        }
    }

    let output: usize = if part_2_enable {
        light_grid
            .iter()
            .map(|&column| column.iter().sum::<usize>())
            .sum()
    } else {
        light_grid
            .iter()
            .map(|&column| column.iter().filter(|&v| *v == 1).count())
            .sum()
    };
    println!("{output:#?}");
}
