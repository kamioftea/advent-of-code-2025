//! This is my solution for [Advent of Code - Day 2: _Gift Shop_](https://adventofcode.com/2025/day/2)
//!
//!

use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-2-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 2.
pub fn run() {
    let contents = fs::read_to_string("res/day-2-input.txt").expect("Failed to read file");
    let ranges = parse_input(&contents);

    println!("The sum of invalid ids is {}", sum_invalid_ids(&ranges));
}

type IdRange = (u64, u64);

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

fn find_invalid_ids(&(min, max): &IdRange) -> Vec<u64> {
    let starting_magnitude = (min.ilog10()) / 2;
    let first_half_of_number = min / 10u64.pow(starting_magnitude + 1);
    let first_even_power_of_ten = 10u64.pow(starting_magnitude);
    let start = (first_half_of_number).max(first_even_power_of_ten);

    (start..)
        .map(|base| format!("{base}{base}").parse::<u64>().unwrap_or(u64::MAX))
        .skip_while(|&invalid_id| invalid_id < min)
        .take_while(|&invalid_id| invalid_id <= max)
        .collect()
}

fn sum_invalid_ids(ranges: &Vec<IdRange>) -> u64 {
    ranges.iter().flat_map(find_invalid_ids).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;
    
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
    fn can_find_invalid_ids() {
        assert_eq!(find_invalid_ids(&(11, 22)), vec![11, 22]);
        assert_eq!(find_invalid_ids(&(95, 115)), vec![99]);
        assert_eq!(find_invalid_ids(&(998, 1012)), vec![1010]);
        assert_eq!(
            find_invalid_ids(&(1188511880, 1188511890)),
            vec![1188511885]
        );
        assert_eq!(find_invalid_ids(&(222220, 222224)), vec![222222]);
        assert_eq!(find_invalid_ids(&(1698522, 1698528)), Vec::<u64>::new());
        assert_eq!(find_invalid_ids(&(446443, 446449)), vec![446446]);
        assert_eq!(find_invalid_ids(&(38593856, 38593862)), vec![38593859]);
        assert_eq!(find_invalid_ids(&(565653, 565659)), Vec::<u64>::new());
        assert_eq!(find_invalid_ids(&(824824821, 824824827)), Vec::<u64>::new());
        assert_eq!(
            find_invalid_ids(&(2121212118, 2121212124)),
            Vec::<u64>::new()
        );
    }

    #[test]
    fn can_sum_invalid_ids_for_range_list() {
        assert_eq!(sum_invalid_ids(&sample_ranges()), 1227775554);
    }
}
