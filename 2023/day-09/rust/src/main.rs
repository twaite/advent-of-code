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

    let next = sum_next_predictions(input.as_str());
    let prev = sum_prev_predictions(input.as_str());

    println!("Sum of next predictions: {}", next);
    println!("Sum of past predictions: {}", prev);
}

fn sum_next_predictions(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_next_number(parse_line(line)))
        .sum()
}

fn sum_prev_predictions(input: &str) -> i32 {
    input
        .lines()
        .map(|line| predict_prev_number(parse_line(line)))
        .sum()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| i32::from_str(s).unwrap())
        .collect()
}

fn predict_next_number(numbers: Vec<i32>) -> i32 {
    let mut groups = vec![numbers];

    while groups.last().unwrap().iter().any(|&v| v != 0) {
        let next = groups
            .last()
            .unwrap()
            .as_slice()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<i32>>();

        groups.push(next);
    }

    groups.into_iter().fold(0, |acc, f| acc + f.last().unwrap())
}

fn predict_prev_number(numbers: Vec<i32>) -> i32 {
    predict_next_number(numbers.into_iter().rev().collect())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45",
        114
    )]
    fn test_sum_next_predictions(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(crate::sum_next_predictions(input), expected);
    }

    #[rstest]
    #[case(vec![0, 3, 6, 9, 12, 15], 18)]
    #[case(vec![1, 3, 6, 10, 15, 21], 28)]
    #[case(vec![10, 13, 16, 21, 30, 45], 68)]
    #[case(vec![-2, -1, 0, 1, 2, 3, 4, 5, 6, 7], 8)]
    #[case(vec![-4, -2, 0, 2, 4], 6)]
    #[case(vec![4, 2, 0, -2, -4], -6)]
    fn test_predict_next_number(#[case] input: Vec<i32>, #[case] expected: i32) {
        assert_eq!(crate::predict_next_number(input), expected);
    }

    #[rstest]
    #[case(vec![10, 13, 16, 21, 30, 45], 5)]
    fn test_predict_prev_number(#[case] input: Vec<i32>, #[case] expected: i32) {
        assert_eq!(crate::predict_prev_number(input), expected);
    }
}
