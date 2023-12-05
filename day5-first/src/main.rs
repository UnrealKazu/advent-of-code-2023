use std::fs;

use std::cmp;

use regex::Regex;

struct Map {
    source: u64,
    dest: u64,
    range: u64,
}

struct Almanac {
    seeds: Vec<u64>,
    maps: [Vec<Map>; 7],
}

fn read_lines(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn process_input(lines: Vec<String>) -> Almanac {
    let dec_reg = Regex::new(r"[\d]+").unwrap();
    let map_reg = Regex::new(r"([\d]+) ([\d]+) ([\d]+)").unwrap();

    // first line are the seeds
    let seeds = lines.get(0).unwrap();

    let seeds_num: Vec<u64> = dec_reg
        .find_iter(seeds)
        .map(|f| f.as_str().parse().unwrap())
        .collect();

    let mut al = Almanac {
        seeds: seeds_num,
        maps: Default::default(),
    };

    // get all specific maps, starting at line 3
    let mut map_num = 0;
    for i in 2..lines.len() {
        let line = lines.get(i).unwrap();

        if line.is_empty() {
            continue;
        }

        // increment the map index if we're past the first map (seed-to-soil)
        if line.contains("map") {
            // new map to use
            if i > 2 {
                map_num += 1;
            }
            continue;
        }

        let caps = map_reg.captures(line).unwrap();

        al.maps[map_num].push(Map {
            source: caps.get(2).unwrap().as_str().parse().unwrap(),
            dest: caps.get(1).unwrap().as_str().parse().unwrap(),
            range: caps.get(3).unwrap().as_str().parse().unwrap(),
        });
    }

    al
}

fn get_lowest_location_number(al: Almanac) -> u64 {
    let mut lowest = u64::MAX;

    for s in al.seeds {
        let mut trace = s;
        for (_i, vec_map) in al.maps.iter().enumerate() {
            for m in vec_map {
                if trace >= m.source && trace < m.source + m.range {
                    // this seed/trace falls within the range,
                    // so we can map it to its corresponding destination
                    trace = m.dest + (trace - m.source);
                    //println!("seed {} has new map num {} from map {}", s, trace, i);
                    break;
                }
            }
        }

        //println!("seed {} has final location num {}", s, trace);

        // check if the final trace (i.e. the location), is lower than what we already know
        lowest = cmp::min(lowest, trace);
    }

    lowest
}

fn main() {
    let al = process_input(read_lines("./puzzle.input"));
    let lowest_loc = get_lowest_location_number(al);

    println!("Lowest location number is {}", lowest_loc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let al = process_input(read_lines("./example.input"));
        let lowest_loc = get_lowest_location_number(al);

        assert_eq!(lowest_loc, 35);
    }
}
