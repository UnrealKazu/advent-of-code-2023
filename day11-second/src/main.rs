use std::{
    fmt,
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::Instant,
};

use arrayvec::ArrayVec;
use regex::Regex;

const EXPANSION_RATE: usize = 1_000_000;
const X_SIZE: usize = 140; /* 10x10 for example, 140x140 for puzzle */
const Y_SIZE: usize = 140;

#[derive(Clone, Copy)]
struct SuperChar {
    char: char,
    empty: bool,
}

struct Universe {
    map: ArrayVec<ArrayVec<SuperChar, Y_SIZE>, X_SIZE>,
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
                if v2.empty {
                    write!(f, "X")?;
                } else {
                    write!(f, "{}", v2.char)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Universe {
    // loop over all lines and mark them if they're empty
    fn mark_empty_lines(&mut self) {
        let empty_reg = Regex::new(r"^\.+$").unwrap();

        for line in &mut self.map {
            let str_line: String = line.iter().map(|l| l.char).collect();
            if empty_reg.is_match(&str_line) {
                // line is empty, so mark all SuperChar as such
                for l in line {
                    l.empty = true;
                }
            }
        }
    }

    fn transpose_map(&mut self) {
        assert!(!self.map.is_empty());
        self.map = (0..self.map[0].len())
            .map(|i| {
                self.map
                    .iter()
                    .map(|inner| inner[i])
                    .collect::<ArrayVec<SuperChar, Y_SIZE>>()
            })
            .collect()
    }

    fn store_galaxies(&mut self) {
        let mut i = 0;
        for (x, vec) in self.map.iter().enumerate() {
            for (y, c) in vec.iter().enumerate() {
                if c.char == '#' {
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
                // loop over the entire x1 to x2 and y1 to y2 to check if there are empty cells
                let mut nrof_empty = 0;

                let x_lower;
                let x_upper;

                if gal1.position.x < gal2.position.x {
                    x_lower = gal1.position.x + 1;
                    x_upper = gal2.position.x;
                } else {
                    x_lower = gal2.position.x + 1;
                    x_upper = gal1.position.x;
                }

                // check a vertical line from x1 to x2 to see if we're crossing any empty cells
                for x in x_lower..x_upper {
                    if self.map.get(x).unwrap().get(gal2.position.y).unwrap().empty {
                        nrof_empty += 1;
                    }
                }

                let y_lower;
                let y_upper;

                if gal1.position.y < gal2.position.y {
                    y_lower = gal1.position.y + 1;
                    y_upper = gal2.position.y;
                } else {
                    y_lower = gal2.position.y + 1;
                    y_upper = gal1.position.y;
                }

                // check a horizontal line from y1 to y2 to see if we're crossing any empty cells
                for y in y_lower..y_upper {
                    if self.map.get(gal1.position.x).unwrap().get(y).unwrap().empty {
                        nrof_empty += 1;
                    }
                }

                let non_empty_length = path_length(gal1, gal2);

                // add the empty cells as expansion rate factor to the direct path
                sum_min_length += non_empty_length + (nrof_empty * (EXPANSION_RATE - 1))
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
        map: ArrayVec::new(),
        galaxies: vec![],
    };

    if let Ok(lines) = read_lines(path) {
        let empty_reg = Regex::new(r"^\.+$").unwrap();

        for line_raw in lines {
            let line = line_raw.unwrap();

            let empty = empty_reg.is_match(&line);
            universe
                .map
                .push(line.chars().map(|l| SuperChar { char: l, empty }).collect());
        }
    }

    universe.transpose_map();
    universe.mark_empty_lines();
    universe.transpose_map();

    universe.store_galaxies();

    universe
}

fn main() {
    let now = Instant::now();

    let universe = parse_input("./puzzle.input");

    let sum = universe.get_sum_shortest_paths();

    println!("Sum of shortest pair paths is {}", sum);

    println!("Duration: {}", now.elapsed().as_millis());
}
