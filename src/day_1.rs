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
        count_zero_positions(&instructions)
    );

    println!(
        "The dial passes zero {} times",
        count_zero_passes(&instructions)
    );
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

fn rotate_dial(position: u32, (direction, distance): &Instruction) -> (u32, u32) {
    let full_rotations = distance / 100;
    let remaining_distance = distance % 100;
    let delta = match direction {
        Direction::Right => remaining_distance,
        Direction::Left => 100 - remaining_distance,
    };

    let new_position = position + delta;
    let passes_zero_again = position > 0
        && match direction {
            Direction::Right => new_position >= 100,
            Direction::Left => new_position <= 100,
        };

    (
        full_rotations + if passes_zero_again { 1 } else { 0 },
        new_position % 100,
    )
}

fn count_zero_positions(instructions: &Vec<Instruction>) -> u32 {
    let mut position = 50;
    let mut count = 0;
    for instruction in instructions {
        (_, position) = rotate_dial(position, instruction);
        if position == 0 {
            count += 1
        }
    }

    count
}

fn count_zero_passes(instructions: &Vec<Instruction>) -> u32 {
    let mut position = 50;
    let mut count = 0;
    for instruction in instructions {
        let (passes, new_pos) = rotate_dial(position, instruction);
        count += passes;
        position = new_pos;
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
        assert_eq!(rotate_dial(11, &(Right, 8)).1, 19);
        assert_eq!(rotate_dial(19, &(Left, 19)).1, 0);

        assert_eq!(rotate_dial(5, &(Left, 10)).1, 95);
        assert_eq!(rotate_dial(95, &(Right, 5)).1, 0);

        assert_eq!(rotate_dial(50, &(Right, 949)), (9, 99));
        assert_eq!(rotate_dial(50, &(Right, 950)), (10, 0));

        assert_eq!(rotate_dial(50, &(Left, 949)), (9, 1));
        assert_eq!(rotate_dial(50, &(Left, 950)), (10, 0));
    }

    #[test]
    fn can_count_zero_positions() {
        assert_eq!(count_zero_positions(&sample_instructions()), 3)
    }

    #[test]
    fn can_count_zero_passes() {
        assert_eq!(count_zero_passes(&sample_instructions()), 6)
    }
}
