use std::{fmt::Display, fs::File, io::{self, BufRead}};


const FILE_PATH: &str = "code.txt";
const MAX: u32 = 99;
const MIN: u32 = 0;

enum Direction {
    L,
    R
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Direction::L => 'L',
            Direction::R => 'R',
        };
        write!(f, "{}", val)
    }
}

fn main() {
    let code = parse_code();
    let mut current_number = 50;
    let mut zero_count: u32= 0;
    for instruction in code {
        let (new_number, zero_addition) = next_number(instruction, current_number);
        current_number = new_number;
        zero_count += zero_addition;
    }
    println!("The secret code is: {}", zero_count);
}

fn next_number(instruction: (Direction, u32), current_number: u32) -> (u32, u32) {
    let (dir, amount) = instruction;
    let sign: i32 = match dir {
        Direction::L => -1,
        Direction::R => 1,
    };
    let mut next_number: i32 = current_number.try_into().unwrap();
    let mut zero_count: u32 = 0;
    for _i in 0..amount {
        next_number += sign;
        if next_number > MAX.try_into().unwrap() {
            next_number = 0;
        } else if next_number < MIN.try_into().unwrap() {
            next_number = MAX.try_into().unwrap();
        }
        if next_number == 0 {
            zero_count += 1;
        }
    }
    return (next_number.try_into().unwrap(), zero_count);
}

fn parse_code() -> Vec<(Direction, u32)> {
    let file = File::open(FILE_PATH).unwrap();
    let mut result = Vec::<(Direction, u32)>::new();
    for line_result in io::BufReader::new(file).lines() {
        let line = line_result.unwrap();
        let dir = parse_direction(&line);
        let amount = parse_amount(&line);
        result.push((dir, amount));
    }
    return result;

}

fn parse_direction(line: &str) -> Direction {
    let dir = line.chars().next().unwrap();
    return match dir {
        'L' => Direction::L,
        'R' => Direction::R,
        _ => panic!("invalid direction")
    }
}

fn parse_amount(line: &str) -> u32 {
    return line[1..].parse().unwrap();
}
