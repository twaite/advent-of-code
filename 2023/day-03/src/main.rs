use std::env;
use std::fs;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let sum = sum_of_part_numbers(&input);

    println!("Sum of part #s: {}", sum);
}

fn sum_of_part_numbers(input: &str) -> u32 {
    let schematic: Schematic = Schematic::from_str(input).unwrap();

    println!(
        "Parts {:?}",
        schematic
            .parts
            .iter()
            .filter(|p| p.is_valid())
            .map(|p| p.number.to_string() + " " + &p.symbols.iter().collect::<String>())
            .collect::<Vec<String>>()
    );

    return schematic
        .parts
        .iter()
        .filter(|p| p.is_valid())
        .fold(0, |acc, part| acc + part.number);
}

#[derive(Debug, PartialEq)]
struct Part {
    number: u32,
    start_idx: usize,
    len: usize,
    symbols: Vec<char>,
}

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && c != '.';
}

#[derive(Debug)]
struct ParsePartError;

impl Part {
    fn is_valid(&self) -> bool {
        return !self.symbols.is_empty();
    }

    fn from_slice(slice: &[&str]) -> Option<Vec<Part>> {
        if slice.len() < 3 {
            return None;
        }

        let above = slice[0];
        let line = slice[1];
        let below = slice[2];

        let digit_regex = Regex::new(r"\d+").unwrap();

        let parts: Vec<Part> = digit_regex
            .find_iter(line)
            .map(|part_num| {
                let number = part_num.as_str().parse::<u32>().unwrap();
                let start_idx = part_num.start();
                let len = part_num.end() - start_idx;
                let mut symbols: Vec<char> = vec![];

                let (skip, take) = match (start_idx, len) {
                    (0, _) => (0, len + 2),
                    (1, _) => (0, len + 2),
                    (_, _) => (start_idx - 1, len + 2),
                };

                println!("start_idx: {}, skip: {}, take: {}", start_idx, skip, take);
                println!(
                    "above: {:?}",
                    above
                        .chars()
                        .skip(skip)
                        .take(take)
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                );
                println!(
                    "below: {:?}",
                    below
                        .chars()
                        .skip(skip)
                        .take(take)
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                );

                /* Above */
                above.chars().skip(skip).take(take).for_each(|c| {
                    if is_symbol(c) {
                        symbols.push(c)
                    }
                });

                /* Left */
                if start_idx > 0 {
                    let char_before_num = line.chars().nth(start_idx - 1);
                    if char_before_num.is_some() && is_symbol(char_before_num.unwrap()) {
                        symbols.push(char_before_num.unwrap());
                    }
                }

                /* Right */
                let char_after_num = line.chars().nth(part_num.end());
                if char_after_num.is_some() && is_symbol(char_after_num.unwrap()) {
                    symbols.push(char_after_num.unwrap());
                }

                /* Below */
                below.chars().skip(skip).take(take).for_each(|c| {
                    if is_symbol(c) {
                        symbols.push(c)
                    }
                });

                return Part {
                    number,
                    start_idx,
                    len,
                    symbols,
                };
            })
            .collect();

        if parts.is_empty() {
            return None;
        }

        return Some(parts);
    }
}

struct Schematic {
    parts: Vec<Part>,
}

#[derive(Debug)]
struct ParseSchematicError;

impl FromStr for Schematic {
    type Err = ParseSchematicError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // note we are appending empty lines at the beginning and end so that
        // the windows iterator starts and ends with the first and last lines
        // in the middle of our slice
        let parts = format!("\n{}\n\n", input)
            .lines()
            .collect::<Vec<&str>>()
            .windows(3)
            .map(|w| Part::from_slice(w))
            .filter(|p| p.is_some())
            .flat_map(|p| p.unwrap())
            .collect();

        Ok(Schematic { parts })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::sum_of_part_numbers;

    #[rstest]
    #[case(crate::Part { number: 1, start_idx: 0, len: 1, symbols: vec!['1'] }, true)]
    fn test_part_is_valid(#[case] part: crate::Part, #[case] expected: bool) {
        assert_eq!(part.is_valid(), expected);
    }

    #[rstest]
    #[case(crate::Part { number: 37, start_idx: 6, len: 2, symbols: vec![] }, false)]
    fn test_part_is_not_valid(#[case] part: crate::Part, #[case] expected: bool) {
        assert_eq!(part.is_valid(), expected);
    }

    #[rstest]
    #[case(&["..*........", ".1..........", "..........."], 1, vec!['*'])]
    #[case(&["...........", ".1..........", "..*........"], 1, vec!['*'])]
    #[case(&["", ".1..........", "..*........"], 1, vec!['*'])]
    #[case(&[".........*.", "...........1", "..........."], 1, vec!['*'])] // end of line and diagonal above
    #[case(&["...........", "...........1", ".........*."], 1, vec!['*'])] // end of line and diagonal below
    #[case(&[".412*.........", ".....880......", "..........340."], 880, vec!['*'])]
    #[case(&["..........1", "..324567886p", "9./...3#3..."], 324567886, vec!['p', '/', '#'])]
    #[case(&["...........", "123/........", "..........."], 123, vec!['/'])]
    #[case(&["...........", "........*456", "..........."], 456, vec!['*'])]
    #[case(&["11111111111", "...12345....", "11111111111"], 12345, vec![])]
    #[case(&["11111111111", "..#12345....", "11111111111"], 12345, vec!['#'])]
    #[case(&[".*.........", "1...........", "..........."], 1, vec!['*'])] // start of line and diagonal above
    #[case(&["...........", "1...........", ".*........."], 1, vec!['*'])] // start of line and diagonal below
    #[case(&["..........*", "...........1", "..........."], 1, vec!['*'])] // end of line and above
    #[case(&["...........", "...........1", "..........*"], 1, vec!['*'])] // end of line and below
    #[case(&["*..........", "1...........", "..........."], 1, vec!['*'])] // start of line and above
    #[case(&["...........", "1...........", "*.........."], 1, vec!['*'])] // start of line and below
    #[case(&["*..........", ".1..........", "..........."], 1, vec!['*'])]
    #[case(&["...........", ".1..........", "*.........."], 1, vec!['*'])]
    #[case(&[".*.........", ".1..........", ""], 1, vec!['*'])]
    #[case(&["....@..", ".....78", ""], 78, vec!['@'])]
    fn test_part_from_slice_valid(
        #[case] input: &[&str],
        #[case] expected_number: u32,
        #[case] expected_symbols: Vec<char>,
    ) {
        let result = crate::Part::from_slice(input).unwrap();
        let first = result.first().unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(first.number, expected_number);
        assert_eq!(first.symbols, expected_symbols);
    }

    #[rstest]
    #[case( &["467..114..", "...*......", "..35..633."])]
    fn test_part_from_slice_invalid(#[case] input: &[&str]) {
        assert_eq!(crate::Part::from_slice(input), None);
    }

    #[rstest]
    #[case(
        "..........1
..324567886p
9./...3#3...",
        324567893
    )]
    #[case(
        ".../...
.21.@.1.
67..^.4.",
        21
    )]
    #[case(
        "#.....
..4....
...5...
....@..
.....78",
        83
    )]
    #[case(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        4361
    )]
    fn test_sum_of_part_numbers(#[case] input: &str, #[case] expected: u32) {
        // normal test
        assert_eq!(sum_of_part_numbers(input), expected);
    }
}
