use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let result = get_closest_seed_location(&input);

    println!("Result: {}", result);
}

fn get_closest_seed_location(input: &str) -> u32 {
    let almanac = Almanac::from_str(input).unwrap();
    println!("{:?}", almanac);
    todo!();
}

struct AlmanacLocation {
    seed: u32,
    soil: u32,
    fertilizer: u32,
    water: u32,
    light: u32,
    temperature: u32,
    humidity: u32,
    location: u32,
}

#[derive(Debug)]
struct AlmanacMapEntry {
    desination_start_idx: u32,
    start_idx: u32,
    range_length: u32,
}

#[derive(Debug)]
struct AlmanacMap {
    to: String,
    entries: Vec<AlmanacMapEntry>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: HashMap<String, AlmanacMap>,
}

#[derive(Debug)]
struct AlmanacParsingError;

impl FromStr for Almanac {
    type Err = AlmanacParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let re = regex::Regex::new(r"^(?P<name>[a-z\-]+):\s*(?P<values>[0-9\s]+)$").unwrap();

        let seed_and_maps_regex =
            Regex::new(r"(?s)^seeds:\s*(?P<seeds>.*?)\n(?P<maps>.*)").unwrap();
        let map_regex =
            Regex::new(r"(?s)(?P<map_name>.* map):\n(?P<map_data>(?:\d+ \d+ \d+\n?)+)").unwrap();

        let captures = seed_and_maps_regex.captures(s).ok_or(AlmanacParsingError)?;

        let seeds = captures
            .name("seeds")
            .ok_or(AlmanacParsingError)?
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let maps_string = captures.name("maps").ok_or(AlmanacParsingError)?.as_str();

        println!("maps_string: {:?}", seeds);

        let maps = map_regex
            .captures(maps_string)
            .ok_or(AlmanacParsingError)
            .into_iter()
            .map(|captures| {
                let map_title = captures.name("map_name").unwrap().as_str().split("-to-");
                let map_data = captures.name("map_data").unwrap().as_str();

                println!("title: {:?}", map_title);
                println!("data: {:?}", map_data);

                let from = map_title.to_owned().next().unwrap().to_string();
                let to = map_title.last().unwrap().to_string();

                let entries = map_data
                    .lines()
                    .map(|line| {
                        let mut values = line.split_whitespace();

                        AlmanacMapEntry {
                            desination_start_idx: values.next().unwrap().parse::<u32>().unwrap(),
                            start_idx: values.next().unwrap().parse::<u32>().unwrap(),
                            range_length: values.next().unwrap().parse::<u32>().unwrap(),
                        }
                    })
                    .collect();

                let map = AlmanacMap { to, entries };

                Ok((from, map))
            })
            .collect::<Result<HashMap<String, AlmanacMap>, AlmanacParsingError>>()
            .unwrap();

        return Ok(Almanac { seeds, maps });
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
        13
    )]
    fn test_count_ids_of_valid_games(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(crate::get_closest_seed_location(input), expected);
    }
}
