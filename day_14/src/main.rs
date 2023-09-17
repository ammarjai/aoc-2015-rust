use std::env;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
enum ReindeerState {
    Flying,
    Rest,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
    state: ReindeerState,
    counter: usize,
    distance: usize,
    points: usize,
}

fn main() {
    let input_file_path = env::args().nth(1).expect("Missing file path argument!");
    let total_time = env::args().nth(2).expect("Missing total time arguement!");
    let input = fs::read_to_string(input_file_path).expect("Invalid file path argument!");

    let total_time = total_time.parse::<usize>().unwrap();
    let mut reindeers: Vec<Reindeer> = Vec::new();
    for line in input.lines() {
        let commands = line.split(' ').collect::<Vec<&str>>();
        reindeers.push(Reindeer {
            name: commands[0].to_string(),
            speed: commands[3].parse::<usize>().unwrap(),
            fly_time: commands[6].parse::<usize>().unwrap(),
            rest_time: commands[13].parse::<usize>().unwrap(),
            state: ReindeerState::Flying,
            counter: commands[6].parse::<usize>().unwrap(),
            distance: 0,
            points: 0,
        })
    }

    for _ in 0..total_time {
        for reindeer in reindeers.iter_mut() {
            match reindeer.state {
                ReindeerState::Flying => {
                    reindeer.counter -= 1;
                    reindeer.distance += reindeer.speed;

                    if reindeer.counter == 0 {
                        reindeer.state = ReindeerState::Rest;
                        reindeer.counter = reindeer.rest_time;
                    }
                }
                ReindeerState::Rest => {
                    reindeer.counter -= 1;
                    if reindeer.counter == 0 {
                        reindeer.state = ReindeerState::Flying;
                        reindeer.counter = reindeer.fly_time;
                    }
                }
            }
        }

        let leading_distance = reindeers
            .iter()
            .max_by_key(|d| d.distance)
            .unwrap()
            .distance;
        reindeers
            .iter_mut()
            .filter(|d| d.distance.eq(&leading_distance))
            .for_each(|d| d.points += 1);
    }

    println!(
        "Longest Distance: {:#?}",
        reindeers
            .iter()
            .max_by(|x, y| x.distance.cmp(&y.distance))
            .unwrap()
    );
    println!(
        "Highest Points: {:#?}",
        reindeers
            .iter()
            .max_by(|x, y| x.points.cmp(&y.points))
            .unwrap()
    );
}
