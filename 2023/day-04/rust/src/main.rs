use std::collections::HashSet;
use std::env;
use std::fs;
use std::result;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let result = sum_card_points(&input);

    println!("Result: {}", result);
}

fn sum_card_points(input: &str) -> u32 {
    return input
        .lines()
        .map(|l| Card::from_str(l).unwrap().get_points())
        .sum();
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

#[derive(Debug)]
struct CardParseError;

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Card\s+(?P<id>\d+):\s+(?P<winning_numbers>(\d+\s*)+)\|\s+(?P<numbers>(\d+\s*)+)",
        )
        .unwrap();

        let captures: regex::Captures<'_> = re.captures(s).unwrap();

        let id = captures["id"].parse::<u32>().unwrap();
        let winning_numbers: Vec<u32> = captures["winning_numbers"]
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let numbers: Vec<u32> = captures["numbers"]
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        return Ok(Card {
            id,
            winning_numbers,
            numbers,
        });
    }
}

impl Card {
    fn get_points(&self) -> u32 {
        let winning_set: HashSet<u32> = self.winning_numbers.clone().into_iter().collect();
        let numbers_set: HashSet<u32> = self.numbers.clone().into_iter().collect();

        let count = winning_set.intersection(&numbers_set).count() as u32;
        let base: u32 = 2;

        return match count {
            0 => 0,
            1 => 1,
            _ => base.pow(count - 1),
        };
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        13
    )]
    fn test_count_ids_of_valid_games(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::sum_card_points(input), expected);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", crate::Card {
        id: 1,
        winning_numbers: vec![41, 48, 83, 86, 17],
        numbers: vec![83, 86,  6, 31, 17,  9, 48, 53]
    })]
    fn test_card_parse(#[case] input: crate::Card, #[case] expected: crate::Card) {
        assert_eq!(input, expected);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    fn test_card_get_points(#[case] input: crate::Card, #[case] expected: u32) {
        assert_eq!(input.get_points(), expected);
    }
}
