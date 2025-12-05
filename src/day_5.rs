//! This is my solution for [Advent of Code - Day 5: _Cafeteria_](https://adventofcode.com/2025/day/5)
//!
//!

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
}

type IdRange = (u64, u64);

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

fn parse_ids(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .sorted()
        .collect()
}

fn parse_input(input: &String) -> (Vec<IdRange>, Vec<u64>) {
    let (range_input, id_input) = input
        .split_once("\n\n")
        .expect("Input should be two sections separated by a blank line");

    (parse_ranges(range_input), parse_ids(id_input))
}

fn count_fresh_ids(ranges: &Vec<IdRange>, ids: &Vec<u64>) -> u64 {
    let mut fresh_count = 0;
    let mut ranges = ranges.clone();
    let mut ids = ids.clone();

    loop {
        if ranges.is_empty() || ids.is_empty() {
            break;
        }

        let (min, max) = ranges[0];
        let id = ids[0];

        if id <= max {
            ids.remove(0);
            if id >= min {
                fresh_count += 1;
            }
        } else {
            ranges.remove(0);
        }
    }

    fresh_count
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
}
