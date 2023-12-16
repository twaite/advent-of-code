use std::env;
use std::fs;
use std::vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let sum = sum_initialization_sequence(input.as_str());

    println!("Sum of initialization sequence: {}", sum);
}

fn sum_initialization_sequence(input: &str) -> u64 {
    input.split(',').fold(0, |acc, n| acc + hash(n))
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |mut acc, c| {
        acc += c as u64;
        acc *= 17;
        acc = acc % 256;
        acc
    })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_get_steps_to_farthest_pipe(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(crate::hash(input), expected);
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 1320)]
    fn test_sum_initialization_sequence(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(crate::sum_initialization_sequence(input), expected);
    }
}
