use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
}

fn sum_of_part_numbers(input: &str) -> u32 {
    let schematic: Schematic = Schematic::from_str(input).unwrap();

    // set the line length
    todo!()
}

struct Part {
    number: u32,
    start_idx: usize,
    len: usize,
    symbols: Vec<char>,
}

#[derive(Debug)]
struct ParsePartError;

impl Part {
    fn is_valid(&self) -> bool {
        return !self.symbols.is_empty();
    }

    fn from_slice(slice: &[&str]) -> Result<Part, ParsePartError> {
        // TODO
        return Ok(Part {
            number: 0,
            start_idx: 0,
            len: 0,
            symbols: vec![],
        });
    }
}

struct Schematic {
    parts: Vec<Part>,
    line_len: usize,
}

#[derive(Debug)]
struct ParseSchematicError;

impl FromStr for Schematic {
    type Err = ParseSchematicError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // regex to find parts
        let re = regex::Regex::new(r"\d+").unwrap();

        let line_len = input.find('\n').unwrap() + 1;

        let test = input.lines().collect::<Vec<&str>>().windows(3).map(|w| {});

        // for each line
        let parts: Vec<Part> = input
            .lines()
            .enumerate()
            .map(|(line_num, line)| {
                // returns parts

                return re
                    .find(line)
                    .iter()
                    .map(|c| {
                        let number = c.as_str().parse::<u32>().unwrap();
                        let start_idx = c.start();
                        let len = c.as_str().len();
                        let mut symbols: Vec<char> = Vec::new();

                        fn is_symbol(c: &char) -> bool {
                            return !c.is_digit(10) && !c.eq(&'.');
                        }

                        // Check the line above for symbols
                        if line_num > 0 {
                            let line_above = input.lines().nth(line_num - 1).unwrap().chars();

                            let start_idx_above = std::cmp::max(start_idx - 1, 0);
                            let end_idx_below: usize = std::cmp::min(line_len, start_idx + len + 1);

                            line_above
                                .skip(start_idx_above)
                                .take(end_idx_below - start_idx_above)
                                .filter(is_symbol)
                                .for_each(|c| symbols.push(c));
                        }

                        // Check before and after
                        if start_idx > 0 {
                            let before = line.chars().nth(start_idx - 1).unwrap();
                            if is_symbol(&before) {
                                symbols.push(before);
                            }
                        }

                        if start_idx + len < line_len {
                            let after = line.chars().nth(start_idx + len + 1).unwrap();
                            if is_symbol(&after) {
                                symbols.push(after);
                            }
                        }

                        // Check the line below for symbols
                        let next_line = input.lines().nth(line_num + 1);

                        if (next_line.is_some()) {
                            let line_below = next_line.unwrap().chars();

                            let start_idx_below = std::cmp::max(start_idx - 1, 0);
                            let end_idx_below: usize = std::cmp::min(line_len, start_idx + len + 1);

                            line_below
                                .skip(start_idx_below)
                                .take(end_idx_below - start_idx_below)
                                .filter(is_symbol)
                                .for_each(|c| symbols.push(c));
                        }

                        return Part {
                            number,
                            start_idx,
                            len,
                            // TODO: symbols
                            symbols: vec![],
                        };
                    })
                    .collect::<Vec<_>>();
            })
            .flat_map(|parts| parts)
            .collect();

        Ok(Schematic { parts, line_len })
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
    #[case(
        "..........1
        ..324567886p
        9./...3#3...",
        324567893
    )]
    fn test1(#[case] input: &str, #[case] expected: u32) {
        // normal test
        assert_eq!(sum_of_part_numbers(input), expected);
    }

    #[rstest]
    #[case(
        ".../...
        .21.@.1.
        67..^.4.",
        0
    )]
    fn test2(#[case] input: &str, #[case] expected: u32) {
        // there are no valid parts, sum better be ZERO
        assert_eq!(sum_of_part_numbers(input), expected);
    }

    #[rstest]
    #[case(
        "#.....
        ..4....
        ...5...
        ....@..
        .....78",
        87
    )]
    fn test3(#[case] input: &str, #[case] expected: u32) {
        // diagonals galore
        assert_eq!(sum_of_part_numbers(input), expected);
    }

    #[rstest]
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
    fn exampleTest(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(sum_of_part_numbers(input), expected);
    }
}
