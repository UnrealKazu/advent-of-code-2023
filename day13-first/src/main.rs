use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, time::Instant,
};

const HORIZONTAL_REFLECTION_FACTOR: usize = 100;

struct Pattern {
    _num: usize,
    hor_map: Vec<String>,
    hor_pairs: Vec<(usize, usize)>,
    ver_map: Vec<String>,
    ver_pairs: Vec<(usize, usize)>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_pattern_summary(pattern: &Pattern) -> usize {
    let check_pairs = |map: &Vec<String>, pairs: &Vec<(usize, usize)>| -> Option<usize> {
        'pair_loop: for pair in pairs {
            // walk back from first entry in pair
            let it1 = map[0..pair.0].iter().rev();
            // walk forwards from second entry in pair
            let it2 = map[pair.1 + 1..].iter();
    
            for (s1, s2) in it1.zip(it2) {
                //println!("Checking s1 {} and s2 {}", s1, s2);
    
                if !s1.eq(s2) {
                    // this pair is no longer reflective, so we can stop checking the other strings
                    continue 'pair_loop;
                }
            }
            
            // if we're here, that means we have a reflective pair
            return Some(pair.0)
        }

        return None
    };

    if let Some(hor_pair) = check_pairs(&pattern.hor_map, &pattern.hor_pairs) {
        return (hor_pair + 1) * HORIZONTAL_REFLECTION_FACTOR;
    }

    if let Some(ver_pair) = check_pairs(&pattern.ver_map, &pattern.ver_pairs) {
        return ver_pair + 1;
    }
    
    panic!("This shouldn't happen. Pattern {} has no solution", pattern._num);
}

fn get_patterns(path: &str) -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();

    if let Ok(lines) = read_lines(path) {
        let mut hor_map: Vec<String> = Vec::new();
        let mut hor_pairs: Vec<(usize, usize)> = Vec::new();
        let mut ver_map: Vec<String> = Vec::new();
        let mut ver_pairs: Vec<(usize, usize)> = Vec::new();

        let mut prev_line = String::new();
        let mut hor_index = 0;

        for line_raw in lines {
            let line = line_raw.unwrap();

            if line.is_empty() {
                // now that the verticals are done as well, check for their pairs
                let mut it = ver_map.iter().enumerate().peekable();
                
                while let Some((i, s1)) = it.next() {
                    if let Some((j, s2)) = it.peek() {
                        if s1.eq(*s2) {
                            ver_pairs.push((i,*j));
                        }
                    }
                }

                // done with this pattern, so push it
                patterns.push(Pattern { _num: patterns.len(), hor_map, hor_pairs, ver_map, ver_pairs });

                // and then reset all important variables
                hor_map = Vec::new();
                hor_pairs = Vec::new();
                ver_map = Vec::new();
                ver_pairs = Vec::new();

                prev_line = String::new();
                hor_index = 0;

                continue;
            }

            // process the horizontal line

            hor_map.push(line.clone());

            if line.eq(&prev_line) {
                hor_pairs.push((hor_index-1, hor_index));
            }

            // process the line as vertical

            if ver_map.is_empty() {
                // initialize with length of string
                for _ in 0..line.len() {
                    ver_map.push(String::new());
                }
            }

            // push the current line as separate chars to each vertical vec String
            for (c, s) in line.chars().zip(ver_map.iter_mut()) {
                s.push(c);
            }

            // prepare for next it
            hor_index += 1;
            prev_line = line;
        }
    }

    patterns
}

fn main() {
    let now = Instant::now();

    let patterns = get_patterns("./puzzle.input");

    let sum = patterns.iter().fold(0, |acc, pat| acc + get_pattern_summary(pat));

    println!("Total sum for all patterns is {}", sum);

    println!("Duration: {}us", now.elapsed().as_micros());
}