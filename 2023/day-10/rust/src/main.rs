use std::env;
use std::fs;
use std::vec;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide a file name as an argument");
    }

    let input = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let next = get_steps_to_farthest_pipe(input.as_str());
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct GetNextDirectionError;

fn get_value_in_direction(
    map: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    direction: &Direction,
) -> Result<(char, usize, usize), GetNextDirectionError> {
    match direction {
        Direction::Up => {
            if row == 0 {
                Err(GetNextDirectionError)
            } else {
                Ok((map[row - 1][col], row - 1, col))
            }
        }
        Direction::Down => {
            if row == map.len() - 1 {
                Err(GetNextDirectionError)
            } else {
                Ok((map[row + 1][col], row + 1, col))
            }
        }
        Direction::Left => {
            if col == 0 {
                Err(GetNextDirectionError)
            } else {
                Ok((map[row][col - 1], row, col - 1))
            }
        }
        Direction::Right => {
            if col == map[0].len() - 1 {
                Err(GetNextDirectionError)
            } else {
                Ok((map[row][col + 1], row, col + 1))
            }
        }
        _ => Err(GetNextDirectionError),
    }
}

fn get_steps_to_farthest_pipe(input: &str) -> u32 {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let index_of_start = input.find("S").unwrap();
    let start_row = index_of_start / map[0].len();
    let start_col = index_of_start % map[0].len();
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut next = ('S', start_row, start_col);

    for direction in directions {
        let start = get_value_in_direction(&map, start_row, start_col, &direction);

        if start.is_ok() {
            let (value, row, col) = start.unwrap();

            if value != '.' {
                next = (value, row, col);
                break;
            }
        }
    }

    let mut pipes = Vec::<char>::new();

    while next.0 != 'S' {
        match next.0 {
            '|' => break,
            '-' => break,
            '7' => break,
            'J' => break,
            'F' => break,
            'L' => break,
            _ => (),
        }

        next = get_value_in_direction(&map, next.1, next.2, &direction).unwrap();

        pipes.push(next.0);
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
.S-7.
.|.|.
.L-J.
.....",
        4
    )]
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        8
    )]
    fn test_get_steps_to_farthest_pipe(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(crate::get_steps_to_farthest_pipe(input), expected);
    }
}
