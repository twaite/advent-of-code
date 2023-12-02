use regex::Regex;
use std::env;
use std::fs;

static NUMBER_MAP: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub enum Order {
    First,
    Last,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if (!(args.len() == 2)) {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let result = part_one(&input);
    println!("Total Part One: {}", result);

    let result2 = part_two(&input);
    println!("Total Part Two: {}", result2);
}

pub fn part_one(input: &str) -> u32 {
    let re = Regex::new(r"\d").unwrap();

    return input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
        })
        .map(|nums| nums[0].to_string() + nums.iter().last().unwrap())
        .map(|line| line.parse::<u32>().unwrap())
        .sum();
}

pub fn part_two(input: &str) -> u32 {
    return input
        .lines()
        .map(|line| {
            let first = find_index(line, &Order::First).unwrap();
            let last = find_index(line, &Order::Last).unwrap();

            return first * 10 + last;
        })
        .sum();
}

pub fn find_index(input: &str, order: &Order) -> Option<u32> {
    let re = Regex::new(r"\d").unwrap();

    let digit_idx_iter = re.find_iter(input).map(|m| m.start());
    let string_idx_iter = NUMBER_MAP
        .iter()
        .enumerate()
        .map(|(num, str)| (num, input.find(str)))
        .filter(|(_, idx)| idx.is_some())
        .map(|(num, idx)| (num as u32 + 1, idx.unwrap()));

    let digit_idx = match order {
        Order::First => digit_idx_iter.min(),
        Order::Last => digit_idx_iter.max(),
    };

    let string_idx = match order {
        Order::First => string_idx_iter.min_by(|(_, idx1), (_, idx2)| idx1.cmp(idx2)),
        Order::Last => string_idx_iter.max_by(|(_, idx1), (_, idx2)| idx1.cmp(idx2)),
    };

    return match (digit_idx, string_idx, order) {
        (Some(d), Some((num, s)), order) => match order {
            Order::First => {
                if d < s {
                    input.chars().nth(d).unwrap().to_digit(10)
                } else {
                    Some(num)
                }
            }
            Order::Last => {
                if d > s {
                    input.chars().nth(d).unwrap().to_digit(10)
                } else {
                    Some(num)
                }
            }
        },
        (Some(d), None, _) => input.chars().nth(d).unwrap().to_digit(10),
        (None, Some((num, _)), _) => Some(num),
        (None, None, _) => None,
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_find_first_number() {
        assert_eq!(crate::find_index("1abc2", &crate::Order::First), Some(1));
        assert_eq!(
            crate::find_index("pqr3stu8vwx", &crate::Order::First),
            Some(3)
        );
        assert_eq!(
            crate::find_index("abcone234", &crate::Order::First),
            Some(1)
        );
        assert_eq!(crate::find_index("eight", &crate::Order::First), Some(8));
        assert_eq!(
            crate::find_index("eightwod4e5f", &crate::Order::First),
            Some(8)
        );
    }

    #[test]
    fn test_find_last_number() {
        assert_eq!(crate::find_index("1abc2", &crate::Order::Last), Some(2));
        assert_eq!(
            crate::find_index("1abckjsad2oaisdfh", &crate::Order::Last),
            Some(2)
        );
        assert_eq!(
            crate::find_index("1ab9afjk83", &crate::Order::Last),
            Some(3)
        );
        assert_eq!(
            crate::find_index("1abc2sdfkjofourthreeight", &crate::Order::Last),
            Some(8)
        );
    }

    #[test]
    fn test_part_one() {
        const TEST_INPUT: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(crate::part_one(TEST_INPUT), 142);
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(crate::part_two(INPUT), 281);
    }
}
