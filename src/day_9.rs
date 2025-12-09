//! This is my solution for [Advent of Code - Day 9: _Movie Theater_](https://adventofcode.com/2025/day/9)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-9-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 9.
pub fn run() {
    let contents = fs::read_to_string("res/day-9-input.txt").expect("Failed to read file");

    let tiles = parse_input(&contents);

    println!(
        "The largest rectangle has area {}",
        find_largest_rectangle(&tiles)
    );
}

type Tile = (usize, usize);

fn parse_input(input: &String) -> Vec<Tile> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn find_largest_rectangle(tiles: &Vec<Tile>) -> usize {
    tiles
        .iter()
        .tuple_combinations()
        .map(|(&(x_a, y_a), &(x_b, y_b))| (x_a.abs_diff(x_b) + 1) * (y_a.abs_diff(y_b) + 1))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day_9::*;

    fn sample_tiles() -> Vec<Tile> {
        vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ]
    }

    #[test]
    fn can_parse_input() {
        let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
        .to_string();

        assert_eq!(parse_input(&input), sample_tiles());
    }

    #[test]
    fn can_find_largest_rectangle() {
        assert_eq!(find_largest_rectangle(&sample_tiles()), 50);
    }
}
