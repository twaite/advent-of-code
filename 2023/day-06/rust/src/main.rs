use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let result = get_multiples_of_races(input.as_str());
    let single_result = get_single_race(input.as_str());

    println!("Result of multiples: {}", result);
    println!("Result of single: {}", single_result);
}

fn get_multiples_of_races(input: &str) -> u64 {
    let re = Regex::new(r"Time:\s+(?P<times>.+?)\s+Distance:\s+(?P<distances>.+)").unwrap();

    let captures = re.captures(input).unwrap();
    let times = captures
        .name("times")
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    captures
        .name("distances")
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .zip(times)
        .map(|(d, t)| Record::new(t, d).count_ways_to_win())
        .fold(1, |acc, n| acc * n)
}

fn get_single_race(input: &str) -> u64 {
    let re = Regex::new(r"Time:\s+(?P<time>.+?)\s+Distance:\s+(?P<distance>.+)").unwrap();
    let captures = re.captures(input).unwrap();
    let time = captures
        .name("time")
        .unwrap()
        .as_str()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = captures
        .name("distance")
        .unwrap()
        .as_str()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    Record::new(time, distance).count_ways_to_win()
}

struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn count_ways_to_win(&self) -> u64 {
        let mut count = 0;

        for i in 1..self.time {
            let time_remaining = self.time - i;

            if time_remaining > 0 && i * time_remaining > self.distance {
                count += 1
            }
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "Time:      7  15   30
        Distance:  9  40  200",
        288
    )]
    fn test_get_multiples_of_races(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(crate::get_multiples_of_races(input), expected);
    }

    #[rstest]
    #[case(
        "Time:      7  15   30
        Distance:  9  40  200",
        71503
    )]
    fn test_get_single_race(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(crate::get_single_race(input), expected);
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn test_count_ways_to_win(#[case] time: u64, #[case] distance: u64, #[case] expected: u64) {
        assert_eq!(
            crate::Record::new(time, distance).count_ways_to_win(),
            expected
        );
    }
}
