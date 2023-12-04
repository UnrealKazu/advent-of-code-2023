use std::collections::HashMap;
use std::fs;
use std::time::Instant;

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

fn parse_cards(
    lines: &[String],
    map: &mut HashMap<usize, usize>,
    start: usize,
    stop: usize,
) -> usize {
    let mut nrof = 0;

    let subset = &lines[start..stop];
    for (i, line) in subset.iter().enumerate() {
        let cur_key = start + i + 1;

        if map.contains_key(&cur_key) {
            // we've seen this card before, so we know what its eventual score is
            nrof += *map.get(&cur_key).unwrap();
            continue;
        }

        // we haven't seen this card before, so let's calculate the number of wins on it
        let wins: usize = get_card_wins(line);

        let mut extra = 0;
        if wins > 0 {
            // the current card has wins, so we are going to check those wins for nested wins

            // to our current start index we add the current index, plus one because we want to skip
            // the current card in the subsequent checks
            extra = parse_cards(lines, map, start + i + 1, start + i + 1 + wins);
        }

        // finally, we store the calculated value for this card, so that we can use it in the future as well
        map.entry(cur_key).or_insert_with(|| 1 + extra);

        // and we need to make sure that we also update our return value with the same value
        nrof += 1 + extra;
    }

    nrof
}

fn main() {
    let now = Instant::now();

    let lines = read_lines("./puzzle.input");
    let mut map: HashMap<usize, usize> = HashMap::new();
    println!(
        "Total count is {}",
        parse_cards(&lines, &mut map, 0, lines.len())
    );

    println!("Duration: {}", now.elapsed().as_millis());
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

        let mut map: HashMap<usize, usize> = HashMap::new();

        let num = parse_cards(&lines, &mut map, 0, lines.len());

        assert_eq!(num, 1)
    }

    #[test]
    fn test_lines_two_cards() {
        let lines: [String; 2] = [
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let mut map: HashMap<usize, usize> = HashMap::new();

        let num = parse_cards(&lines, &mut map, 0, lines.len());

        assert_eq!(num, 2)
    }

    #[test]
    fn test_lines_three_cards() {
        let lines: [String; 3] = [
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let mut map: HashMap<usize, usize> = HashMap::new();

        let num = parse_cards(&lines, &mut map, 0, lines.len());

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

        let mut map: HashMap<usize, usize> = HashMap::new();

        let num = parse_cards(&lines, &mut map, 0, lines.len());

        assert_eq!(num, 8)
    }

    #[test]
    fn test_lines_example_cards() {
        let lines: [String; 6] = [
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let mut map: HashMap<usize, usize> = HashMap::new();

        let num = parse_cards(&lines, &mut map, 0, lines.len());

        assert_eq!(num, 30)
    }

    #[test]
    fn test_part_of_puzzle_input() {
        // card 198 - 63 68  -> 2
        // card 199 - /      -> 0
        // card 200 - 49     -> 1
        // card 201 - 33     -> 1
        // card 202 - /      -> 0
        let lines: [String; 5] = [
            String::from("Card 198: 71 62 73 96 79 63 41 17 56 68 | 95 77 16 70 29 68 66 63 98 80 20 18 31 34 52  5 42 22 49  6 25 38 51 75 50"),
            String::from("Card 199: 70 84 46 98 44 45 16 36 29 99 | 78 21 92 77 32 91 22 90 76 74 42 55 51 69 94 64 26 65 41 97 10 34 15 35  9"),
            String::from("Card 200: 96 60 87 21 80 48 44 69  3 49 |  2 65 66 94 55 62 72 52 86 15 30 71 45 82 49 47 81 33 14 42  4  1 51 75 34"),
            String::from("Card 201: 55 53 33 19  1 70 17 61  2 72 | 62  6 30 86 45 71 46 33 15 90 73 37 18 12 68 87 89 49  8 60 52 22 51 25 74"),
            String::from("Card 202:  5 47 96 53 54 14 77 29 12  3 | 26 71 91 86 59 70 78  8 83 92 35 64  9 79 84 34 36 93 90 40 16 44 51  6  4"),
        ];

        let mut map: HashMap<usize, usize> = HashMap::new();

        let num = parse_cards(&lines, &mut map, 0, lines.len());

        assert_eq!(num, 12)
    }
}
