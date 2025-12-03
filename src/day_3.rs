//! This is my solution for [Advent of Code - Day 3: _Lobby_](https://adventofcode.com/2025/day/3)
//!
//! - [`parse_input`] converts the input string into a list of [`BatteryBank`]s, which are `Vec`s of numbers
//! - [`find_highest_joltage`] finds the highest joltage for a given bank, taking the length of number to find
//! - [`sum_highest_joltage`] solves both parts, taking the length of number to differentiate the two parts

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-3-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 3.
pub fn run() {
    let contents = fs::read_to_string("res/day-3-input.txt").expect("Failed to read file");

    let battery_banks = parse_input(&contents);

    println!(
        "The total output joltage for 2 batteries is {}",
        sum_highest_joltage(&battery_banks, 2)
    );

    println!(
        "The total output joltage for 12 batteries is {}",
        sum_highest_joltage(&battery_banks, 12)
    );
}

/// Represents a bank of batteries that combine to provide a joltage
type BatteryBank = Vec<u32>;

/// Parse input so that each line is a [`BatteryBank`], taking the digits as the individual batteries
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

/// The highest joltage is the highest number of length `digits` that can be formed from the batteries in the battery
/// bank, taken in order.
fn find_highest_joltage(bank: &BatteryBank, digits: usize) -> u64 {
    let mut max_digits = vec![0; digits];

    for start in 0..=(bank.len() - digits) {
        let mut set_digit = false;
        for current in 0..digits {
            if max_digits[current] < bank[start + current] || set_digit {
                set_digit = true;
                max_digits[current] = bank[start + current];
            }
        }
    }

    max_digits
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit as u64)
}

/// Solves both parts, part 1 `digits` = 2, part 2 `digits` = 12.
fn sum_highest_joltage(banks: &Vec<BatteryBank>, digits: usize) -> u64 {
    banks
        .iter()
        .map(|bank| find_highest_joltage(bank, digits))
        .sum()
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
            find_highest_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
            98
        );
        assert_eq!(
            find_highest_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
            89
        );
        assert_eq!(
            find_highest_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
            78
        );
        assert_eq!(
            find_highest_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
            92
        );

        assert_eq!(
            find_highest_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(
            find_highest_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            811111111119
        );
        assert_eq!(
            find_highest_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434234234278
        );
        assert_eq!(
            find_highest_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            888911112111
        );
    }

    #[test]
    fn can_sum_highest_joltages() {
        assert_eq!(sum_highest_joltage(&example_banks(), 2), 357);
        assert_eq!(sum_highest_joltage(&example_banks(), 12), 3121910778619);
    }
}
