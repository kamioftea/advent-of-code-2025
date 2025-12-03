//! This is my solution for [Advent of Code - Day 3: _Lobby_](https://adventofcode.com/2025/day/3)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input.txt").expect("Failed to read file");

    let battery_banks = parse_input(&contents);

    println!(
        "The total output joltage is {}",
        sum_highest_joltage(&battery_banks)
    );
}

type BatteryBank = Vec<u32>;

fn parse_input(input: &String) -> Vec<BatteryBank> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|battery| battery.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn find_highest_joltage(bank: &BatteryBank) -> u32 {
    let mut first_max = &0;
    let mut second_max = &0;

    for (battery_a, battery_b) in bank.into_iter().tuple_windows() {
        if battery_a > first_max {
            first_max = battery_a;
            second_max = battery_b;
        } else if battery_b > second_max {
            second_max = battery_b
        }
    }

    first_max * 10 + second_max
}

fn sum_highest_joltage(banks: &Vec<BatteryBank>) -> u32 {
    banks.iter().map(find_highest_joltage).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;

    fn example_banks() -> Vec<BatteryBank> {
        vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ]
    }

    #[test]
    fn can_parse_input() {
        let example_input = "987654321111111\n\
            811111111111119\n\
            234234234234278\n\
            818181911112111"
            .to_string();

        assert_eq!(parse_input(&example_input), example_banks());
    }

    #[test]
    fn can_find_highest_joltage_in_battery_bank() {
        assert_eq!(
            find_highest_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            98
        );
        assert_eq!(
            find_highest_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            89
        );
        assert_eq!(
            find_highest_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            78
        );
        assert_eq!(
            find_highest_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            92
        );
    }

    #[test]
    fn can_sum_highest_joltages() {
        assert_eq!(sum_highest_joltage(&example_banks()), 357)
    }
}
