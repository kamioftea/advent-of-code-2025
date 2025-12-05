//! This is my solution for [Advent of Code - Day 5: _Cafeteria_](https://adventofcode.com/2025/day/5)
//!
//! - [`parse_input`] delegates to [`parse_ranges`] and [`parse_ids`]. Notably both return sorted ids/ranges.
//! - [`count_fresh_ids`] solves part one
//! - [`count_possible_fresh_ids`] solves part two

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-5-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 5.
pub fn run() {
    let contents = fs::read_to_string("res/day-5-input.txt").expect("Failed to read file");

    let (ranges, ids) = parse_input(&contents);

    println!("There are {} fresh IDs", count_fresh_ids(&ranges, &ids));
    println!(
        "There are {} possible fresh IDs",
        count_possible_fresh_ids(&ranges)
    );
}

/// An inclusive range of ids that are fresh
type IdRange = (u64, u64);

/// Parse each line in the format `{min}-{max}` as an [`IdRange`], and return them in sorted order (default sorting
/// for pairs is by first entry, then by second)
fn parse_ranges(input: &str) -> Vec<IdRange> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once("-")
                .expect("ranges must be two numbers separated by a `-`");

            (start.parse().unwrap(), end.parse().unwrap())
        })
        .sorted()
        .collect()
}

/// Parse each line as a numeric id, and return the ids in ascending order
fn parse_ids(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect()
}

/// The input is in two sections, split by a blank line. Delegate each section to a dedicated parser.
fn parse_input(input: &String) -> (Vec<IdRange>, Vec<u64>) {
    let (range_input, id_input) = input
        .split_once("\n\n")
        .expect("Input should be two sections separated by a blank line");

    (parse_ranges(range_input), parse_ids(id_input))
}

/// Assumes the input ranges and ids are sorted. Use a loop to iterate the ranges and ids in step. Counting where the
/// current id is in the current range.
fn count_fresh_ids(ranges: &Vec<IdRange>, ids: &Vec<u64>) -> u64 {
    let mut fresh_count = 0;
    let mut range_index = 0;
    let mut id_index = 0;

    loop {
        if range_index == ranges.len() || id_index == ids.len() {
            break;
        }

        let (min, max) = ranges[range_index];
        let id = ids[id_index];

        if id <= max {
            id_index += 1;
            if id >= min {
                fresh_count += 1;
            }
        } else {
            range_index += 1;
        }
    }

    fresh_count
}

/// Assumes the passed ranges are sorted. Return the count of ids included within one of the ranges
fn count_possible_fresh_ids(ranges: &Vec<IdRange>) -> u64 {
    let mut total_ids = 0;
    let mut id_threshold = 0;

    for &(min, max) in ranges {
        let lower_bound = min.max(id_threshold);
        if lower_bound <= max {
            total_ids += max - lower_bound + 1;
        }
        id_threshold = id_threshold.max(max + 1)
    }

    total_ids
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;

    fn sample_data() -> (Vec<IdRange>, Vec<u64>) {
        (
            vec![(3, 5), (10, 14), (12, 18), (16, 20)],
            vec![1, 5, 8, 11, 17, 32],
        )
    }

    #[test]
    fn can_parse_input() {
        let input = "\
3-5
10-14
16-20
12-18

1
8
17
11
5
32
"
        .to_string();

        assert_eq!(parse_input(&input), sample_data());
    }

    #[test]
    fn can_count_fresh_ids() {
        let (ranges, ids) = sample_data();
        assert_eq!(count_fresh_ids(&ranges, &ids), 3);
    }

    #[test]
    fn can_count_possible_fresh_ids() {
        let (ranges, _) = sample_data();
        assert_eq!(count_possible_fresh_ids(&ranges), 14);
    }
}
