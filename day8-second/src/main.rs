use std::{
    cmp,
    collections::HashMap,
    fs::{self},
};

use regex::Regex;

struct Ghost {
    cur_dir: String,
    steps: u32,
    min_steps: u32,
}

struct Movements {
    directions: String,
    ghosts: Vec<Ghost>,
    map: HashMap<String, Node>,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> Movements {
    let node_reg = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

    let directions = lines.first().unwrap().as_str().to_string();

    let mut movements: Movements = Movements {
        ghosts: vec![],
        directions,
        map: HashMap::new(),
    };

    for line in lines.iter().skip(2) {
        let caps = node_reg.captures(line.as_str()).unwrap();

        let hash = caps.get(1).unwrap().as_str().to_string();

        if hash.ends_with('A') {
            movements.ghosts.push(Ghost {
                cur_dir: hash.clone(),
                steps: 0,
                min_steps: u32::MAX,
            })
        }

        movements.map.entry(hash).or_insert(Node {
            left: caps.get(2).unwrap().as_str().to_string(),
            right: caps.get(3).unwrap().as_str().to_string(),
        });
    }

    movements
}

fn calculate_steps(mut movements: Movements) -> u64 {
    let mut finished_ghosts: Vec<Ghost> = Vec::new();

    while !movements.ghosts.is_empty() {
        for c in movements.directions.chars() {
            let mut removable: Vec<usize> = Vec::new();

            for (i, ghost) in movements.ghosts.iter_mut().enumerate() {
                let cur_dir: &Node = &movements.map[&ghost.cur_dir];
                match c {
                    'R' => ghost.cur_dir = cur_dir.right.to_string(),
                    'L' => ghost.cur_dir = cur_dir.left.to_string(),
                    _ => panic!("Unexpected direction entry"),
                }

                ghost.steps += 1;

                if ghost.cur_dir.ends_with('Z') {
                    if ghost.min_steps != ghost.steps {
                        // we've completed the loop once, continue once more to see if we are in a stable loop
                        ghost.min_steps = cmp::min(ghost.min_steps, ghost.steps);
                        ghost.steps = 0;
                    } else {
                        // we've completed the loop twice with the same nrof steps, so remove it
                        removable.push(i);
                    }
                }
            }

            // remove all ghosts that have reached their destination twice
            for i in removable {
                finished_ghosts.push(movements.ghosts.swap_remove(i));
            }
        }
    }

    // now we know the minimum nrof steps each ghost needs to get to the destination
    // because they're looping, we need to know the least common multiple of all these steps
    // which will return a number of steps that will be the exact step on which all ghosts
    // end up at their destination
    let mut lcm_number = u64::from(finished_ghosts.first().unwrap().min_steps);
    for g in finished_ghosts.iter().skip(1) {
        lcm_number = lcm(lcm_number, u64::from(g.min_steps));
    }

    lcm_number
}

fn main() {
    let movements = parse_input(read_lines("./puzzle.input"));
    let steps = calculate_steps(movements);

    println!("Number of steps {}", steps);
}
