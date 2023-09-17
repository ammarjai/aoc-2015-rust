use std::collections::HashMap;
use std::env;
use std::fs;

// Reference: https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn permute_heap(k: usize, data: &mut Vec<String>, output: &mut Vec<Vec<String>>) {
    if k == 1 {
        output.push(data.clone());
    } else {
        permute_heap(k - 1, data, output);

        for index in 0..k - 1 {
            if k % 2 == 0 {
                data.swap(index, k - 1)
            } else {
                data.swap(0, k - 1);
            }
            permute_heap(k - 1, data, output)
        }
    }
}

#[derive(Debug)]
struct Path {
    nodes: Vec<String>,
    weights: i32,
}

struct UndirectedGraph {
    nodes: Vec<String>,
    lines: HashMap<(String, String), i32>,
}

impl UndirectedGraph {
    fn add_lines(&mut self, node1: String, node2: String, weight: i32) {
        if !self.nodes.iter().any(|x| x == &node1) {
            self.nodes.push(node1.clone());
        }

        if !self.nodes.iter().any(|x| x == &node2) {
            self.nodes.push(node2.clone());
        }

        if !self
            .lines
            .iter()
            .any(|((k1, k2), _)| (k1, k2) == (&node1, &node2))
            && !self
                .lines
                .iter()
                .any(|((k1, k2), _)| (k1, k2) == (&node2, &node1))
        {
            self.lines.insert((node1.clone(), node2.clone()), weight);
            self.lines.insert((node2.clone(), node1.clone()), weight);
        } else {
            *self.lines.get_mut(&(node1.clone(), node2.clone())).unwrap() += weight;
            *self.lines.get_mut(&(node2.clone(), node1.clone())).unwrap() += weight;
        }
    }

    fn find_optimal_seating_arrangment(&self) {
        let mut permutations: Vec<Vec<String>> = Vec::new();
        permute_heap(self.nodes.len(), &mut self.nodes.clone(), &mut permutations);

        let mut longest_path = Path {
            nodes: vec![],
            weights: 0,
        };
        for path in permutations.iter() {
            let mut total_weights: i32 = 0;
            for index in 0..path.len() - 1 {
                let weight = self
                    .lines
                    .get(&(path[index].clone(), path[index + 1].clone()))
                    .unwrap();
                total_weights += weight;
            }

            let weight = self
                .lines
                .get(&(path[0].clone(), path[path.len() - 1].clone()))
                .unwrap();
            total_weights += weight;

            if total_weights > longest_path.weights {
                longest_path.nodes = path.clone();
                longest_path.weights = total_weights;
            }
        }

        println!("Optimal: {:#?}", longest_path);
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let part_2_enable = env::args().nth(2).or(None).is_some();
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut graph = UndirectedGraph {
        nodes: Vec::new(),
        lines: HashMap::new(),
    };
    for input in inputs.lines() {
        let commands = input.split(' ').collect::<Vec<&str>>();
        let node1 = commands[0];
        let mut node2 = commands[10];
        node2 = &node2[0..node2.len() - 1];
        let negative = commands[2] == "lose";
        let mut weight = commands[3].parse::<i32>().unwrap();
        if negative {
            weight *= -1;
        }

        graph.add_lines(String::from(node1), String::from(node2), weight);
    }

    if part_2_enable {
        for value in graph.nodes.clone().iter() {
            graph.add_lines(String::from("Me"), value.clone(), 0);
        }
    }

    graph.find_optimal_seating_arrangment();
}
