use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

use regex::Regex;

struct Universe {
    map: Vec<Vec<char>>,
    galaxies: Vec<Galaxy>,
}

struct Galaxy {
    _id: u16,
    position: Position,
}

struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Universe {
    // writes the map such that it looks like the same image as the input
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v1 in &self.map {
            for v2 in v1 {
                write!(f, "{}", v2)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Universe {
    // loop over all lines and expand them (i.e. double them) if they're empty
    fn expand_empty_lines(&mut self) {
        let empty_reg = Regex::new(r"^\.+$").unwrap();

        let mut new_map: Vec<Vec<char>> = vec![];

        for line in &self.map {
            new_map.push(line.clone());

            let str_line: String = line.iter().collect();
            if empty_reg.is_match(&str_line) {
                // again an empty line, so push it again
                new_map.push(line.clone());
            }
        }

        self.map = new_map;
    }

    fn transpose_map(&mut self) {
        assert!(!self.map.is_empty());
        self.map = (0..self.map[0].len())
            .map(|i| self.map.iter().map(|inner| inner[i]).collect::<Vec<char>>())
            .collect()
    }

    fn store_galaxies(&mut self) {
        let mut i = 0;
        for (x, vec) in self.map.iter().enumerate() {
            for (y, c) in vec.iter().enumerate() {
                if *c == '#' {
                    self.galaxies.push(Galaxy {
                        _id: i + 1,
                        position: Position { x, y },
                    });

                    i += 1;
                }
            }
        }
    }

    fn get_sum_shortest_paths(&self) -> usize {
        let mut sum_min_length = 0;

        let path_length = |gal1: &Galaxy, gal2: &Galaxy| {
            usize::abs_diff(gal1.position.x, gal2.position.x)
                + usize::abs_diff(gal1.position.y, gal2.position.y)
        };

        for (i, gal1) in self.galaxies.iter().enumerate() {
            for gal2 in self.galaxies.iter().skip(i + 1) {
                sum_min_length += path_length(gal1, gal2);
            }
        }

        sum_min_length
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(path: &str) -> Universe {
    let mut universe = Universe {
        map: vec![],
        galaxies: vec![],
    };

    if let Ok(lines) = read_lines(path) {
        let empty_reg = Regex::new(r"^\.+$").unwrap();

        for line_raw in lines {
            let line = line_raw.unwrap();

            universe.map.push(line.chars().collect());
            if empty_reg.is_match(&line) {
                // this is an empty line, so push it again to double it
                universe.map.push(line.chars().collect());
            }
        }
    }

    universe.transpose_map();
    universe.expand_empty_lines();
    universe.transpose_map();

    universe.store_galaxies();

    universe
}

fn main() {
    let now = Instant::now();

    let universe = parse_input("./puzzle.input");

    let sum = universe.get_sum_shortest_paths();

    println!("Sum of shortest pair paths is {}", sum);

    println!("Duration: {}", now.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_universe() -> Universe {
        Universe {
            map: vec![
                vec![
                    '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.',
                ],
                vec![
                    '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.',
                ],
                vec![
                    '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                ],
                vec![
                    '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.',
                ],
                vec![
                    '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.',
                ],
            ],
            galaxies: vec![],
        }
    }

    #[test]
    fn test_get_galaxies() {
        let mut universe = get_test_universe();
        universe.store_galaxies();

        assert_eq!(universe.galaxies.len(), 9);
    }

    #[test]
    fn test_get_pair_path() {
        let mut universe = get_test_universe();
        universe.store_galaxies();

        let min_sum = universe.get_sum_shortest_paths();

        assert_eq!(min_sum, 374);
    }
}
