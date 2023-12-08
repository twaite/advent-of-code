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

    println!("Total winnings: {}", result);
}

fn get_total_winnings(input: &str) -> u32 {
    let mut hands = input
        .lines()
        .map(|line| Hand::from_str(line).unwrap())
        .collect::<Vec<Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.bid * (idx as u32 + 1))
        .sum()
}

fn get_card_value(card: char) -> Result<u32, &'static str> {
    match card {
        'A' => Ok(14),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' => Ok(11),
        'T' => Ok(10),
        '2'..='9' => Ok(card.to_digit(10).unwrap()),
        _ => Err("Invalid card"),
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl Hand {
    fn get_hand_strength(&self) -> u32 {
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_hand_strength().cmp(&other.get_hand_strength())
    }
}

#[derive(Debug)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"([2-9TJQKA]{5}) (\d+)").unwrap();

        let captures = re.captures(s.trim()).unwrap();

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
    #[case(
        "2345A 1
        Q2KJJ 13
        Q2Q2Q 19
        T3T3J 17
        T3Q33 11
        2345J 3
        J345A 2
        32T3K 5
        T55J5 29
        KK677 7
        KTJJT 34
        QQQJA 31
        JJJJJ 37
        JAAAA 43
        AAAAJ 59
        AAAAA 61
        2AAAA 23
        2JJJJ 53
        JJJJ2 41",
        6592
    )]
    #[case(
        "627Q8 1
        A26Q7 2
        2K637 3",
        11
    )]
    #[case(
        "AAAQQ 1
        22288 2
        33232 3",
        11
    )]
    #[case(
        "QQQQQ 1
        AAAAA 2
        QQQQQ 3
        AAAAA 4",
        1 * 1 + 2 * 3 + 3 * 2 + 4 * 4
    )]
    #[case(
        "23232 1
        KQJT9 2",
        4
    )]
    #[case(
        "9TTTT 1
        98888 2",
        4
    )]
    fn test_get_total_winnings(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(crate::get_total_winnings(input), expected);
    }

    #[rstest]
    #[case("423Q5 765", 42425)]
    #[case("22345 765", 1022345)]
    #[case("2233A 765", 2022344)]
    #[case("24332 765", 2024332)]
    #[case("222J5 765", 3022315)]
    #[case("A2225 765", 3142225)]
    #[case("22233 765", 4022233)]
    #[case("32223 765", 4032223)]
    #[case("QQQQ4 765", 5133324)]
    #[case("JJ2JJ 765", 5121321)]
    #[case("22222 765", 6022222)]
    #[case("KKKKK 765", 6144443)]
    #[case("AAAAA 765", 6155554)]
    fn test_get_hand_strength(#[case] input: crate::Hand, #[case] expected: u32) {
        assert_eq!(input.get_hand_strength(), expected);
    }
}
