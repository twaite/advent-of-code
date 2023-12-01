use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
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
    let re = Regex::new(r"\d").unwrap();
    let mut number_map = HashMap::new();
    number_map.insert("one", "1");
    number_map.insert("two", "2");
    number_map.insert("three", "3");
    number_map.insert("four", "4");
    number_map.insert("five", "5");
    number_map.insert("six", "6");
    number_map.insert("seven", "7");
    number_map.insert("eight", "8");
    number_map.insert("nine", "9");

    let number_string_regex = number_map
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>()
        .join("|");

    let regex_string = String::from(r"\d|") + &number_string_regex;
    let re = Regex::new(&regex_string).unwrap();

    return input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
        })
        .map(|nums| {
            let mut first = nums[0].to_string();
            let mut last = nums.iter().last().unwrap().to_string();

            first = match number_map.get(first.as_str()) {
                Some(v) => v.to_string(),
                None => first,
            };

            last = match number_map.get(last.as_str()) {
                Some(v) => v.to_string(),
                None => last,
            };

            first + &last
        })
        .map(|line| line.parse::<u32>().unwrap())
        .sum();
}

#[cfg(test)]
mod tests {

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
