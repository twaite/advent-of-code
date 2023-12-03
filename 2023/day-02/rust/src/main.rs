mod game;

use game::Game;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let count = count_ids_of_valid_games(&input);
    let power = sum_powers_of_valid_games(&input);

    println!("Count of IDs from valid games: {}", count);
    println!("Power of cubes from valid games: {}", power);
}

fn get_games(input: &str) -> Vec<Game> {
    return input
        .lines()
        .map(|game| game.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();
}

fn count_ids_of_valid_games(input: &str) -> u32 {
    return get_games(input)
        .iter()
        .filter(|game| game.is_valid())
        .map(|game| game.id)
        .sum();
}

fn sum_powers_of_valid_games(input: &str) -> u32 {
    return get_games(input).iter().map(|game| game.get_power()).sum();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

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
    fn test_count_ids_of_valid_games(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::count_ids_of_valid_games(input), expected);
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        2286
    )]
    fn test_(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::sum_powers_of_valid_games(input), expected);
    }
}
