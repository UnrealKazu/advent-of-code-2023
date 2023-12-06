use std::fs;

use regex::Regex;

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn process_input(input: Vec<String>) -> (u64, u64) {
    let dec_reg = Regex::new(r"[\d]+").unwrap();

    let times: u64 = dec_reg
        .find_iter(input.first().unwrap())
        .map(|f| f.as_str().to_string())
        .reduce(|a: String, b: String| a + &b)
        .unwrap()
        .parse()
        .unwrap();

    let distances: u64 = dec_reg
        .find_iter(input.get(1).unwrap())
        .map(|f| f.as_str().to_string())
        .reduce(|a: String, b: String| a + &b)
        .unwrap()
        .parse()
        .unwrap();

    (times, distances)
}

fn process_races(race: (u64, u64)) -> u64 {
    let mut mult = 0;

    for i in (1..race.0 - 1).rev() {
        if (race.0 - i) * i > race.1 {
            mult += 1;
        }
    }

    mult
}

fn main() {
    let race = process_input(read_lines("./puzzle.input"));
    let num = process_races(race);

    println!("Race multiplication number {}", num);
}
