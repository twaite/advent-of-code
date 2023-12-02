use regex::Regex;
use std::str::FromStr;

/**
 * Throw
 */

#[derive(Debug, PartialEq, Clone)]
pub struct Throw {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseThrowError;

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
pub struct Game {
    pub id: u32,
    throws: Vec<Throw>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGameError;

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
    pub fn is_valid(&self) -> bool {
        let red_total = 12;
        let green_total = 13;
        let blue_total = 14;

        return self.throws.iter().all(|throw| {
            throw.red <= red_total && throw.green <= green_total && throw.blue <= blue_total
        });
    }

    pub fn get_power(&self) -> u32 {
        let max_throws = self.throws.iter().fold(
            Throw {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, next| {
                let mut throw = acc.clone();

                if next.red > acc.red {
                    throw.red = next.red;
                }

                if next.green > acc.green {
                    throw.green = next.green;
                }

                if next.blue > acc.blue {
                    throw.blue = next.blue;
                }

                return throw;
            },
        );

        return max_throws.red * max_throws.green * max_throws.blue;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

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
    #[case("Game 5: 6 red, 1 blue, 13 green; 2 blue, 1 red, 2 green", true)]

    fn test_game_is_valid(#[case] input: crate::Game, #[case] expected: bool) {
        assert_eq!(input.is_valid(), expected);
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_get_power(#[case] input: crate::Game, #[case] expected: u32) {
        assert_eq!(input.get_power(), expected);
    }
}
