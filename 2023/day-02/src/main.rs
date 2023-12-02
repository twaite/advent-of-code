use regex::Regex;
use rstest::rstest;
use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let result = get_valid_games(&input);

    print!("Number of valid games: {}", result);
}

fn get_valid_games(input: &str) -> u32 {
    return input
        .lines()
        .map(|game| game.parse::<Game>().unwrap())
        .filter(|game| game.is_valid())
        .map(|game| game.id)
        .sum();
}

/**
 * Peek
 */

#[derive(Debug, PartialEq)]
struct Throw {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseThrowError;

impl FromStr for Throw {
    type Err = ParseThrowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+) (red|blue|green)").unwrap();
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for cap in re.captures_iter(s) {
            let count = cap[1].parse::<u32>().unwrap();
            match &cap[2] {
                "red" => red = count,
                "blue" => blue = count,
                "green" => green = count,
                _ => panic!("Invalid color"),
            }
        }

        return Ok(Throw { red, blue, green });
    }
}

/**
 * Game
 */

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    throws: Vec<Throw>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Game (?P<name>\d+): (?P<throws>.*)").unwrap();
        let caps = re.captures(s).unwrap();
        let id = caps["name"].parse::<u32>().unwrap();
        let throws = caps["throws"]
            .split("; ")
            .map(|throw| throw.parse::<Throw>().unwrap())
            .collect::<Vec<Throw>>();

        return Ok(Game { id, throws });
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        let red_total = 12;
        let green_total = 13;
        let blue_total = 14;

        return self.throws.iter().all(|throw| {
            throw.red < red_total && throw.green < green_total && throw.blue < blue_total
        });
    }
}

/**
 * Tests
 */

#[rstest]
#[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
#[case(
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    true
)]
#[case(
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    false
)]
#[case(
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    false
)]
#[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]

fn test_game_is_valid(#[case] input: Game, #[case] expected: bool) {
    assert_eq!(input.is_valid(), expected);
}

#[rstest]
#[case(
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    8
)]
#[case(
    "Game 20: 4 green, 3 blue, 1 red; 9 red, 14 blue, 9 green; 1 blue, 17 red, 2 green; 8 red, 13 blue, 8 green; 7 red, 2 green, 20 blue; 6 green, 13 red, 5 blue
Game 21: 7 red, 1 blue; 1 blue, 5 red, 4 green; 5 green, 5 red; 7 red, 2 green; 4 green, 2 red, 1 blue
Game 22: 6 red, 8 green, 18 blue; 2 green, 7 blue, 2 red; 18 blue, 8 green, 1 red; 10 red, 7 green, 20 blue; 5 blue, 10 green, 4 red
Game 23: 2 green, 2 red, 15 blue; 2 red, 6 green, 4 blue; 8 red, 5 green
Game 28: 9 red, 7 blue; 6 blue, 11 red; 10 red, 10 blue, 3 green",
    49
)]
fn test_get_valid_games(#[case] input: &str, #[case] expected: u32) {
    assert_eq!(get_valid_games(input), expected);
}

#[rstest]
fn test_part_two() {
    assert_eq!(true, true);
}
