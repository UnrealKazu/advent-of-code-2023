use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_card_wins(line: &str) -> usize {
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

    finds
}

fn parse_cards(lines: &[String], start: usize, stop: usize) -> usize {
    let mut nrof = 0;

    let subset = &lines[start..stop];
    for (i, line) in subset.iter().enumerate() {
        // count the card we're currently looping over, as it adds to the pile
        nrof += 1;

        let wins: usize = get_card_wins(line);

        println!("Card {} has by default {} wins", start + i + 1, wins);

        if wins > 0 {
            // the current card has wins, so we are going to check those wins for nested wins
            // to our current start index we add the current index, plus one because we want to skip
            // the current card in the subsequent checks
            let extra = parse_cards(lines, start + i + 1, start + i + 1 + wins);
            nrof += extra;

            //println!("With an additional {} wins for card {}", extra, start + 1);
        }
    }

    nrof
}

fn main() {
    let lines = read_lines("./puzzle.input");
    println!("Total win sum is {}", parse_cards(&lines, 0, lines.len()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_card_score_should_return_correct_score() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let score = get_card_wins(line);

        assert_eq!(score, 4);
    }

    #[test]
    fn test_get_card_score_with_zero_score_should_return_correctly() {
        let line = "Card 1: 41 48 83 86 17 | 1 2 3 4 5";

        let score = get_card_wins(line);

        assert_eq!(score, 0);
    }

    #[test]
    fn test_lines_one_card() {
        let lines: [String; 1] = [String::from(
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11    0",
        )];

        let num = parse_cards(&lines, 0, lines.len());

        assert_eq!(num, 1)
    }

    #[test]
    fn test_lines_two_cards() {
        let lines: [String; 2] = [
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let num = parse_cards(&lines, 0, lines.len());

        assert_eq!(num, 2)
    }

    #[test]
    fn test_lines_three_cards() {
        let lines: [String; 3] = [
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let num = parse_cards(&lines, 0, lines.len());

        assert_eq!(num, 4)
    }

    #[test]
    fn test_lines_four_cards() {
        let lines: [String; 4] = [
            String::from("Card 1:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 2: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 3: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 4: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let num = parse_cards(&lines, 0, lines.len());

        assert_eq!(num, 8)
    }
}
