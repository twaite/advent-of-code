use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let result = get_multiples_of_races(input.as_str());

    println!("Result: {}", result);
}

fn get_multiples_of_races(input: &str) -> u32 {
    let re = regex::Regex::new(r"Time:\s+(?P<times>.+?)\s+Distance:\s+(?P<distances>.+)").unwrap();

    let captures = re.captures(input).unwrap();
    let times = captures
        .name("times")
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    captures
        .name("distances")
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|t| t.parse::<u32>().unwrap())
        .zip(times)
        .map(|(d, t)| Record::new(t, d).count_ways_to_win())
        .fold(1, |acc, n| acc * n)
}

struct Record {
    time: u32,
    distance: u32,
}

impl Record {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, distance }
    }

    fn count_ways_to_win(&self) -> u32 {
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
    fn test_count_ids_of_valid_games(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(crate::get_multiples_of_races(input), expected);
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn test_count_ways_to_win(#[case] time: u32, #[case] distance: u32, #[case] expected: u32) {
        assert_eq!(
            crate::Record::new(time, distance).count_ways_to_win(),
            expected
        );
    }
}
