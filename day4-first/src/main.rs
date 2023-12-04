use std::collections::HashMap;
use std::io::{self, BufRead};
use std::time::Instant;
use std::{fs::File, path::Path};

use regex::Regex;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_card_score(line: &str) -> i32 {
    let card_reg = Regex::new(r"Card[\s\d]+: ([\d ]+)\| ([\d ]+)").unwrap();
    let num_reg = Regex::new(r"[\d]+").unwrap();

    let caps = card_reg.captures(line).unwrap();
    //println!("first group is {}", caps.get(1).unwrap().as_str());
    //println!("second group is {}", caps.get(2).unwrap().as_str());

    // we use a hashmap for constant time lookup for all our scratched numbers
    // so map the winning numbers into that hashmap
    let wins: HashMap<&str, bool> = num_reg
        .find_iter(caps.get(1).unwrap().as_str())
        .map(|m| (m.as_str(), true))
        .collect();

    let mut finds = 0;

    // and then do a lookup on the scratched numbers
    for scratched in num_reg.find_iter(caps.get(2).unwrap().as_str()) {
        if wins.contains_key(scratched.as_str()) {
            finds += 1;
        }
    }

    if finds == 0 {
        return 0;
    }

    let base: i32 = 2;
    base.pow(finds - 1)
}

fn parse_cards(lines: io::Lines<io::BufReader<File>>) -> i32 {
    let mut score = 0;

    for line in lines {
        score += get_card_score(&line.unwrap());
    }

    score
}

fn main() {
    let now = Instant::now();

    if let Ok(lines) = read_lines("./puzzle.input") {
        println!("Total win sum is {}", parse_cards(lines));
    }

    println!("Duration: {}", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_card_score_should_return_correct_score() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let score = get_card_score(line);

        assert_eq!(score, 8);
    }

    #[test]
    fn test_get_card_score_with_zero_score_should_return_correctly() {
        let line = "Card 1: 41 48 83 86 17 | 1 2 3 4 5";

        let score = get_card_score(line);

        assert_eq!(score, 0);
    }
}
