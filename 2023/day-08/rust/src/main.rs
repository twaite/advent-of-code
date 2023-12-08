use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let result = count_steps(input.as_str());

    println!("Result: {}", result);
}

fn count_steps(input: &str) -> u32 {
    let map = DesertMap::from_str(input).unwrap();

    let mut idx = String::from("AAA");
    let mut acc = 0_u32;
    let mut i = 0_usize;

    while idx != "ZZZ" {
        let (left, right) = map.documents.get(&idx).unwrap();
        match map.directions.get(i % map.directions.len()).unwrap() {
            Direction::Left => {
                idx = left.to_string();
                acc += 1;
            }
            Direction::Right => {
                idx = right.to_string();
                acc += 1;
            }
        }
        i += 1;
    }

    acc
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct DirectionParseError;

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(DirectionParseError),
        }
    }
}

#[derive(Debug)]
struct DesertMap {
    directions: Vec<Direction>,
    documents: HashMap<String, (String, String)>,
}

#[derive(Debug)]
struct MapParseError;

impl FromStr for DesertMap {
    type Err = MapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map_regex = regex::Regex::new(r"(?<directions>[RL]+)\s*(?s:(?<documents>.*))").unwrap();
        let doc_regex = regex::Regex::new(
            r"\s*(?P<idx>[A-Z]{3})\s*=\s*\((?P<left>[A-Z]{3}),\s*(?P<right>[A-Z]{3})\)\s*",
        )
        .unwrap();

        let map_cap = map_regex.captures(s).unwrap();
        let directions = map_cap
            .name("directions")
            .unwrap()
            .as_str()
            .chars()
            .map(|c| c.to_string().parse::<Direction>().unwrap())
            .collect::<Vec<Direction>>();

        let doc_cap = map_cap.name("documents").unwrap().as_str();

        let documents = doc_cap.trim().lines().into_iter().fold(
            HashMap::<String, (String, String)>::new(),
            |mut acc: HashMap<String, (String, String)>, next| {
                let doc_cap = doc_regex.captures(next).unwrap();

                let idx = doc_cap.name("idx").unwrap().as_str().to_string();
                let left = doc_cap.name("left").unwrap().as_str().to_string();
                let right = doc_cap.name("right").unwrap().as_str().to_string();
                acc.insert(idx, (left, right));

                acc
            },
        );

        Ok(DesertMap {
            directions,
            documents,
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)",
        2
    )]
    #[case(
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)",
        6
    )]
    fn test_get_total_winnings(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(crate::count_steps(input), expected);
    }
}
