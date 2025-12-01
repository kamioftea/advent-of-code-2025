//! This is my solution for [Advent of Code - Day 1: _Secret Entrance_](https://adventofcode.com/2025/day/1)
//!
//!

use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

type Instruction = (Direction, u32);

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-1-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 1.
pub fn run() {
    let contents = fs::read_to_string("res/day-1-input.txt").expect("Failed to read file");
    let instructions = parse_input(contents);

    println!(
        "The dial stops on zero {} times",
        count_zero_positions(instructions)
    )
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line.split_at(1) {
            ("R", dist) => (Direction::Right, dist.parse::<u32>().unwrap()),
            ("L", dist) => (Direction::Left, dist.parse::<u32>().unwrap()),
            _ => unreachable!("Direction is always L or R"),
        })
        .collect()
}

fn rotate_dial(position: u32, instruction: Instruction) -> u32 {
    match instruction {
        (Direction::Right, dist) => (position + dist) % 100,
        (Direction::Left, dist) => (position + 100 - (dist % 100)) % 100,
    }
}

fn count_zero_positions(instructions: Vec<Instruction>) -> usize {
    let mut position = 50;
    let mut count = 0;
    for instruction in instructions {
        position = rotate_dial(position, instruction);
        if position == 0 {
            count += 1
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::day_1::Direction::{Left, Right};
    use crate::day_1::*;
    
    fn sample_input() -> String {
        "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string()
    }

    fn sample_instructions() -> Vec<Instruction> {
        vec![
            (Left, 68),
            (Left, 30),
            (Right, 48),
            (Left, 5),
            (Right, 60),
            (Left, 55),
            (Left, 1),
            (Left, 99),
            (Right, 14),
            (Left, 82),
        ]
    }

    #[test]
    fn can_parse_input() {
        assert_eq!(parse_input(sample_input()), sample_instructions());
    }

    #[test]
    fn can_turn_dial() {
        assert_eq!(rotate_dial(11, (Right, 8)), 19);
        assert_eq!(rotate_dial(19, (Left, 19)), 0);

        assert_eq!(rotate_dial(5, (Left, 10)), 95);
        assert_eq!(rotate_dial(95, (Right, 5)), 0);
    }

    #[test]
    fn can_count_zero_positions() {
        assert_eq!(count_zero_positions(sample_instructions()), 3)
    }
}
