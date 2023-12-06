use std::fs;

use regex::Regex;

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn process_input(input: Vec<String>) -> Vec<(u16, u16)> {
    let dec_reg = Regex::new(r"[\d]+").unwrap();

    let times: Vec<u16> = dec_reg
        .find_iter(input.first().unwrap())
        .map(|f| f.as_str().parse().unwrap())
        .collect();

    let distances: Vec<u16> = dec_reg
        .find_iter(input.get(1).unwrap())
        .map(|f| f.as_str().parse().unwrap())
        .collect();

    let races: Vec<(u16, u16)> = times.into_iter().zip(distances).collect();

    races
}

fn process_races(races: Vec<(u16, u16)>) -> u32 {
    let mut mult = 0;

    for (n, race) in races.iter().enumerate() {
        let mut nrof_wins = 0;

        for i in (1..race.0 - 1).rev() {
            if (race.0 - i) * i > race.1 {
                nrof_wins += 1;
            }
        }

        if n == 0 {
            mult = nrof_wins;
        } else {
            mult *= nrof_wins;
        }
    }

    mult
}

fn main() {
    let races = process_input(read_lines("./puzzle.input"));
    let num = process_races(races);

    println!("Race multiplication number {}", num);
}
