//! This is my solution for [Advent of Code - Day 2: _Gift Shop_](https://adventofcode.com/2025/day/2)
//!
//! - [`parse_input`] splits the range list and parses each to a pair of `u64`s
//! - [`find_invalid_ids_for_repeats`] finds invalid ids that repeat a given number of times
//! - [`sum_invalid_id_pairs`] solves part 1 by mapping the ranges with [`find_invalid_ids_for_repeats`] specifically
//!   for `repeats == 2`, and summing the values
//! - [`find_invalid_ids_for_range`] finds invalid ids for all repeat variants from 2 up to the length of the maximum
//!   value, and returns the unique ids
//! - [`sum_invalid_ids`] solves part 2 by mapping the ranges with [`find_invalid_ids_for_range`] and summing

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let contents = fs::read_to_string("res/day-2-input.txt").expect("Failed to read file");
    let ranges = parse_input(&contents);

    println!(
        "The sum of invalid id pairs is {}",
        sum_invalid_id_pairs(&ranges)
    );

    println!("The sum of all invalid ids is {}", sum_invalid_ids(&ranges));
}

/// Represents a range of numbers to check for invalid ids in the format `(min, max)` inclusive.
type IdRange = (u64, u64);

/// The input is a comma-separated list of ranges in the format `{min}-{max}`, which are each parsed into an [`IdRange`]
///
/// `parse_input(&"1-3,10-11".to_string())` would give `vec![(1,3), (10,11)]`.
fn parse_input(input: &String) -> Vec<IdRange> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect()
}

/// Finds all ids in a given range that are composed of a smaller number repeated `repeats` times. Whitespace is trimmed
/// to account for the trailing newline in the puzzle input file.
///
/// - `123123` is invalid when repeats is `2` because it is `123` twice in a row,
/// - `121212` is invalid when repeats is `3`
///   and so on.
///
/// Starting from the first part of the number only works if the range has numbers with a uniform magnitude. This
/// splits ranges up to enforce this,
fn find_invalid_ids_for_repeats(&(min, max): &IdRange, repeats: u32) -> Vec<u64> {
    fn find_invalid_ids_for_subrange(&(min, max): &IdRange, repeats: u32) -> Vec<u64> {
        let starting_exponent = (min.ilog10()) / repeats;
        let first_part_of_number = min / 10u64.pow(min.ilog10() - starting_exponent);

        (first_part_of_number..)
            .map(|base| {
                format!("{base}")
                    .repeat(repeats as usize)
                    .parse::<u64>()
                    .ok()
            })
            .while_some()
            .skip_while(|&invalid_id| invalid_id < min)
            .take_while(|&invalid_id| invalid_id <= max)
            .collect()
    }

    (min.ilog10()..=max.ilog10())
        .filter(|exponent| (exponent + 1) % repeats == 0)
        .flat_map(|exponent| {
            let subrange = (
                min.max(10u64.pow(exponent)),
                max.min(10u64.pow(exponent + 1) - 1),
            );
            find_invalid_ids_for_subrange(&subrange, repeats)
        })
        .collect()
}

/// Iterate through all the possible repeat variants for a range, and return the unique invalid ids
fn find_invalid_ids_for_range(range: &IdRange) -> Vec<u64> {
    (2..=range.1.ilog10() + 1)
        .flat_map(|repeats| find_invalid_ids_for_repeats(range, repeats))
        .unique()
        .collect()
}

/// Solves part 1 by summing ids that are invalid due to being composed of a pair of numbers
fn sum_invalid_id_pairs(ranges: &Vec<IdRange>) -> u64 {
    ranges
        .iter()
        .flat_map(|range| find_invalid_ids_for_repeats(range, 2))
        .sum()
}

/// Solves part 2 by summing ids that are invalid due to being composed of any repeating pattern of numbers
fn sum_invalid_ids(ranges: &Vec<IdRange>) -> u64 {
    ranges
        .iter()
        .flat_map(|range| find_invalid_ids_for_range(range))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;
    use crate::helpers::test::assert_contains_in_any_order;

    fn sample_input() -> String {
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
        565653-565659,824824821-824824827,2121212118-2121212124\n"
        .to_string()
    }

    fn sample_ranges() -> Vec<IdRange> {
        vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124),
        ]
    }

    #[test]
    fn can_parse_input() {
        assert_eq!(parse_input(&sample_input()), sample_ranges());
    }

    #[test]
    fn can_find_invalid_pair_ids() {
        vec![
            ((11, 22), vec![11, 22]),
            ((95, 115), vec![99]),
            ((998, 1012), vec![1010]),
            ((1188511880, 1188511890), vec![1188511885]),
            ((222220, 222224), vec![222222]),
            ((1698522, 1698528), Vec::<u64>::new()),
            ((38593856, 38593862), vec![38593859]),
            ((565653, 565659), Vec::<u64>::new()),
            ((824824821, 824824827), Vec::<u64>::new()),
            ((2121212118, 2121212124), Vec::<u64>::new()),
        ]
        .into_iter()
        .for_each(|(range, invalid_ids)| {
            assert_eq!(
                find_invalid_ids_for_repeats(&range, 2),
                invalid_ids,
                "find_invalid_ids_for_repeats(&{range:?}, 2) should produce {invalid_ids:?}"
            );
        });
    }

    #[test]
    fn can_find_all_invalid_ids() {
        vec![
            ((11, 22), vec![11, 22]),
            ((95, 115), vec![99, 111]),
            ((998, 1012), vec![999, 1010]),
            ((1188511880, 1188511890), vec![1188511885]),
            ((222220, 222224), vec![222222]),
            ((1698522, 1698528), Vec::<u64>::new()),
            ((38593856, 38593862), vec![38593859]),
            ((565653, 565659), vec![565656]),
            ((824824821, 824824827), vec![824824824]),
            ((2121212118, 2121212124), vec![2121212121]),
        ]
        .into_iter()
        .for_each(|(range, invalid_ids)| {
            assert_contains_in_any_order(find_invalid_ids_for_range(&range), invalid_ids);
        });
    }

    #[test]
    fn can_sum_invalid_ids_for_pairs_in_range_list() {
        assert_eq!(sum_invalid_id_pairs(&sample_ranges()), 1227775554);
    }

    #[test]
    fn can_sum_invalid_ids_for_range_list() {
        assert_eq!(sum_invalid_ids(&sample_ranges()), 4174379265);
    }
}
