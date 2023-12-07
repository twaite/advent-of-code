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

    let result = get_total_winnings(input.as_str());
}

fn get_total_winnings(input: &str) -> u32 {
    todo!();
}

fn get_card_value(card: char) -> Result<u32, &'static str> {
    match card {
        'A' => Ok(13),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' => Ok(11),
        'T' => Ok(10),
        '2'..='9' => Ok(card.to_digit(10).unwrap()),
        _ => Err("Invalid card"),
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl Hand {
    fn get_hand_strength(&self) -> u32 {
        println!("{:?}", self);

        let mut values = self
            .cards
            .iter()
            .fold(HashMap::<&char, u32>::new(), |mut acc, card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            });

        values.retain(|_, v| v > &mut 0);

        let mut mapped: Vec<(&char, u32)> = values.into_iter().map(|(k, v)| (k, v)).collect();

        mapped.sort_by(|a, b| b.1.cmp(&a.1));

        // calculate strength based off hand
        let hand_value = match mapped.as_slice() {
            // five of a kind
            [(_, 5)] => 6_000_000,
            // four of a kind
            [(_, 4), ..] => 5_000_000,
            // full house
            [(_, 3), (_, 2)] => 4_000_000,
            // three of a kind
            [(_, 3), ..] => 3_000_000,
            // two pair
            [(_, 2), (_, 2), ..] => 2_000_000,
            // one pair
            [(_, 2), ..] => 1_000_000,
            // high card
            _ => 0,
        };

        return hand_value + self.get_high_card_value();
    }

    fn get_high_card_value(&self) -> u32 {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| get_card_value(*card).unwrap() * 10_u32.pow(idx as u32))
            .sum()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // TODO: fallback to card values
        self.get_hand_strength() == other.get_hand_strength()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_strength = self.get_hand_strength();
        let other_strength = other.get_hand_strength();

        self_strength.partial_cmp(&other_strength)
    }
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"([2-9TJQKA]{5}) (\d+)").unwrap();

        let captures = re.captures(s).unwrap();

        Ok(Hand {
            cards: captures.get(1).unwrap().as_str().chars().collect(),
            bid: captures.get(2).unwrap().as_str().parse().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483",
        6440
    )]
    fn test_get_total_winnings(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(crate::get_total_winnings(input), expected);
    }

    #[rstest]
    #[case("423Q5 765", 42425)]
    #[case("22345 765", 1022345)]
    #[case("2233A 765", 2022343)]
    #[case("24332 765", 2024332)]
    #[case("222J5 765", 3022315)]
    #[case("A2225 765", 3132225)]
    #[case("22233 765", 4022233)]
    #[case("32223 765", 4032223)]
    #[case("QQQQ4 765", 5133324)]
    #[case("JJ2JJ 765", 5121321)]
    #[case("22222 765", 6022222)]
    #[case("KKKKK 765", 6144443)]
    fn test_get_hand_strength(#[case] input: crate::Hand, #[case] expected: u32) {
        assert_eq!(input.get_hand_strength(), expected);
    }
}
