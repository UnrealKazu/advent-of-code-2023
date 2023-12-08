use std::{
    collections::HashMap,
    fs::{self},
};

use regex::Regex;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> (String, HashMap<String, Node>) {
    let node_reg = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();

    let directions = lines.first().unwrap().as_str().to_string();

    let mut map: HashMap<String, Node> = HashMap::new();

    for line in lines.iter().skip(2) {
        let caps = node_reg.captures(line.as_str()).unwrap();

        let hash = caps.get(1).unwrap().as_str();

        map.entry(hash.to_string()).or_insert(Node {
            left: caps.get(2).unwrap().as_str().to_string(),
            right: caps.get(3).unwrap().as_str().to_string(),
        });
    }

    (directions, map)
}

fn calculate_steps(dir: String, map: HashMap<String, Node>) -> u32 {
    let mut key: &String = &String::from("AAA");
    let mut cur: &Node = &map[key];
    let mut steps = 0;

    while key != "ZZZ" {
        for c in dir.chars() {
            match c {
                'R' => key = &cur.right,
                'L' => key = &cur.left,
                _ => panic!("Unexpected node entry"),
            }

            steps += 1;

            if key == "ZZZ" {
                break;
            }

            cur = &map[key];
        }
    }

    steps
}

fn main() {
    let input = parse_input(read_lines("./puzzle.input"));
    let steps = calculate_steps(input.0, input.1);

    println!("Number of steps {}", steps);
}
