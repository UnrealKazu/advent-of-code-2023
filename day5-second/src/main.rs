use std::fs;

use std::cmp;

use regex::Regex;

#[derive(Debug)]
struct Seed {
    start: u64,
    range: u64,
    init_map_num: usize,
}

struct Map {
    source: u64,
    dest: u64,
    range: u64,
}

struct Almanac {
    seeds: Vec<Seed>,
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
    let dec_reg = Regex::new(r"[\d]+ [\d]+").unwrap();
    let map_reg = Regex::new(r"([\d]+) ([\d]+) ([\d]+)").unwrap();

    // first line are the seeds
    let seeds_str = lines.get(0).unwrap();

    let seeds_num: Vec<&str> = dec_reg.find_iter(seeds_str).map(|f| f.as_str()).collect();

    let seeds: Vec<Seed> = seeds_num
        .iter()
        .map(|s| {
            let spl: Vec<&str> = s.split(' ').collect();

            Seed {
                start: spl[0].parse().unwrap(),
                range: spl[1].parse().unwrap(),
                init_map_num: 0,
            }
        })
        .collect();

    let mut al = Almanac {
        seeds,
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

// processes the given seed, changes its start and range if needed,
// and optionally returns split off seeds
fn process_seed(seed: &mut Seed, map: &Map, map_num: usize) -> (bool, Vec<Seed>) {
    // there are four possible scenarios that need to be actively handled
    // 1: the seed's range overlaps entirely with the map
    // 2: the seed's range starts before the map, but ends in the map
    // 3: the seed's range starts in the map, but ends after the map
    // 4: the seed's range is entirely contained within the map

    let mut new_seeds: Vec<Seed> = Vec::new();
    let mut adjusted = false;

    // case #1
    if seed.start < map.source && seed.start + seed.range > map.source + map.range {
        // seed is completely overlapping, two new seeds are needed

        if seed.start < map.source {
            // create a new seed that runs up to the start of the map
            new_seeds.push(Seed {
                start: seed.start,
                range: map.source - seed.start,
                init_map_num: map_num,
            });
        }

        // create a seed that covers the tail of the map
        new_seeds.push(Seed {
            start: map.source + map.range,
            range: (seed.start + seed.range) - (map.source + map.range),
            init_map_num: map_num,
        });

        // current seed will be adjusted to cover the entire map,
        // also, immediately assume destination
        seed.start = map.dest;
        seed.range = map.range;

        adjusted = true;
    }
    // case #2
    else if seed.start < map.source
        && seed.start + seed.range > map.source
        && seed.start + seed.range < map.source + map.range
    {
        // create a seed that covers the part before the map
        new_seeds.push(Seed {
            start: seed.start,
            range: map.source - seed.start,
            init_map_num: map_num,
        });

        // current seed will be adjusted to cover the section inside the map
        // also, immediately assume destination
        seed.range = (seed.start + seed.range) - map.source;
        seed.start = map.dest;

        adjusted = true;
    }
    // case #3
    else if seed.start > map.source
        && seed.start < map.source + map.range
        && seed.start + seed.range > map.source + map.range
    {
        // create a new seed that covers the tail of the map
        new_seeds.push(Seed {
            start: map.source + map.range,
            range: (seed.start + seed.range) - (map.source + map.range),
            init_map_num: map_num,
        });

        // current seed will be adjusted to cover the section inside the map
        // also, immediately assume destination
        seed.range = (map.source + map.range) - seed.start;
        seed.start = map.dest + (seed.start - map.source);

        adjusted = true;
    } else if seed.start >= map.source && seed.start + seed.range < map.source + map.range {
        // no need to split off new seeds, only need to correct current seed

        seed.start = map.dest;
        // no need to adjust the range, as it fell entirely inside the map

        adjusted = true;
    }

    (adjusted, new_seeds)
}

fn get_lowest_location_number(mut al: Almanac) -> u64 {
    let mut lowest = u64::MAX;

    while let Some(mut seed) = al.seeds.pop() {
        println!("Size of seed stack {}", al.seeds.len());

        for (i, vec_map) in al.maps.iter().enumerate() {
            if i < seed.init_map_num {
                continue;
            }

            for m in vec_map {
                println!(
                    "Checking seed {} - {} with map {} - {} dest {}",
                    seed.start, seed.range, m.source, m.range, m.dest
                );
                let mut new_seeds = process_seed(&mut seed, m, i);

                if !new_seeds.1.is_empty() {
                    for ns in &new_seeds.1 {
                        println!(
                            "Added seed {} - {} with map num {}",
                            ns.start, ns.range, ns.init_map_num
                        );
                    }
                }

                al.seeds.append(&mut new_seeds.1);

                if new_seeds.0 {
                    println!(
                        "Seed adjusted to {} - {} for map {} - {} dest {}",
                        seed.start, seed.range, m.source, m.range, m.dest
                    );
                    // this map was applied, so move over to next map
                    println!("Breaking this shit!");
                    break;
                }
            }
        }

        // check if the final trace (i.e. the location), is lower than what we already know
        lowest = cmp::min(lowest, seed.start);
    }

    lowest
}

fn main() {
    let al = process_input(read_lines("./example.input"));
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

        assert_eq!(lowest_loc, 46);
    }

    #[test]
    fn test_overlapping_seed() {
        /*
           seed: 75 - 15 range (so max 90)
           map: 80 - 5 range (so max 85), with dest 150

           => split into
                   seed 75 - 5 range
                   seed 80 - 5 range -> current seed, adjusted to dest 150
                   seed 85 - 5 range
        */

        let mut s = Seed {
            start: 75,
            range: 15,
            init_map_num: 0,
        };

        let m = Map {
            source: 80,
            dest: 150,
            range: 5,
        };

        let new = process_seed(&mut s, &m, 0).1;

        assert_eq!(new.len(), 2);

        let s1 = new.first().unwrap();
        let s2 = new.get(1).unwrap();

        // check new split off seeds
        assert_eq!(s1.start, 75);
        assert_eq!(s1.range, 5);

        assert_eq!(s2.start, 85);
        assert_eq!(s2.range, 5);

        // check if the existing seed has been correctly altered
        assert_eq!(s.start, 150);
        assert_eq!(s.range, 5);
    }

    #[test]
    fn test_seed_ending_in_map() {
        /*
            seed: 55 - 25 range (so max 80)
            map: 70 - 15 range (so max 85), with dest 30

            => split into
                    seed 55 - 15 range
                    seed 70 - 10 range -> current seed, adjusted to dest 30
        */

        let mut s = Seed {
            start: 55,
            range: 25,
            init_map_num: 0,
        };

        let m = Map {
            source: 70,
            dest: 30,
            range: 15,
        };

        let new = process_seed(&mut s, &m, 0).1;

        assert_eq!(new.len(), 1);

        let s1 = new.first().unwrap();

        // check new split off seed
        assert_eq!(s1.start, 55);
        assert_eq!(s1.range, 15);

        // check if the existing seed has been correctly altered
        assert_eq!(s.start, 30);
        assert_eq!(s.range, 10);
    }

    #[test]
    fn test_seed_starting_in_map() {
        /*
           seed: 75 - 25 range (so max 100)
           map: 70 - 15 range (so max 85), with dest 120

           => split into
               seed 75 - 10 range -> current seed, adjusted to dest 120
               seed 85 - 15 range
        */

        let mut s = Seed {
            start: 75,
            range: 25,
            init_map_num: 0,
        };

        let m = Map {
            source: 70,
            dest: 120,
            range: 15,
        };

        let new = process_seed(&mut s, &m, 0).1;

        assert_eq!(new.len(), 1);

        let s1 = new.first().unwrap();

        // check new split off seed
        assert_eq!(s1.start, 85);
        assert_eq!(s1.range, 15);

        // check if the existing seed has been correctly altered
        assert_eq!(s.start, 125);
        assert_eq!(s.range, 10);
    }

    #[test]
    fn test_seed_contained_in_map() {
        /*
           seed: 75 - 5 range (so max 80)
           map: 70 - 15 range (so max 85), with dest 40

           => no splits needed, only correction of the current seed
        */

        let mut s = Seed {
            start: 75,
            range: 5,
            init_map_num: 0,
        };

        let m = Map {
            source: 70,
            dest: 40,
            range: 15,
        };

        let new = process_seed(&mut s, &m, 0).1;

        assert_eq!(new.len(), 0);

        // check if the existing seed has been correctly altered
        assert_eq!(s.start, 40);
        assert_eq!(s.range, 5);
    }
}
