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
    weights: usize,
}

struct UndirectedGraph {
    nodes: Vec<String>,
    lines: HashMap<(String, String), usize>,
}

impl UndirectedGraph {
    fn add_lines(&mut self, node1: String, node2: String, weight: usize) {
        if !self.nodes.iter().any(|x| x == &node1) {
            self.nodes.push(node1.clone())
        }

        if !self.nodes.iter().any(|x| x == &node2) {
            self.nodes.push(node2.clone())
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
        }
    }

    fn find_shortest_and_longest_path(&self) {
        let mut permutations: Vec<Vec<String>> = Vec::new();
        permute_heap(self.nodes.len(), &mut self.nodes.clone(), &mut permutations);

        let mut shortest_path = Path {
            nodes: vec![],
            weights: 1000,
        };
        let mut longest_path = Path {
            nodes: vec![],
            weights: 0,
        };
        for path in permutations.iter() {
            let mut total_weights: usize = 0;
            for index in 0..path.len() - 1 {
                let weight = self
                    .lines
                    .get(&(path[index].clone(), path[index + 1].clone()))
                    .unwrap();
                total_weights += weight;
            }

            if total_weights < shortest_path.weights {
                shortest_path.nodes = path.clone();
                shortest_path.weights = total_weights;
            }

            if total_weights > longest_path.weights {
                longest_path.nodes = path.clone();
                longest_path.weights = total_weights;
            }
        }

        println!("Shortest Path: {:#?}", shortest_path);
        println!("Longest Path: {:#?}", longest_path);
    }
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let inputs = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let mut graph = UndirectedGraph {
        nodes: Vec::new(),
        lines: HashMap::new(),
    };
    for input in inputs.lines() {
        let commands = input.split(' ').collect::<Vec<&str>>();
        let node1 = commands[0];
        let node2 = commands[2];
        let weight = commands[4].parse::<usize>().unwrap();

        graph.add_lines(String::from(node1), String::from(node2), weight);
    }

    graph.find_shortest_and_longest_path();
}
