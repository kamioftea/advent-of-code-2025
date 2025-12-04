//! This is my solution for [Advent of Code - Day 4: _Printing Department_](https://adventofcode.com/2025/day/4)
//!
//!

use std::collections::HashSet;
use std::fs;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct Grid {
    rolls: HashSet<Coordinate>,
}

impl From<&String> for Grid {
    fn from(value: &String) -> Self {
        let rolls = value
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, cell)| cell == '@')
                    .map(move |(x, _)| Coordinate { x, y })
            })
            .collect();

        Grid { rolls }
    }
}

impl Grid {
    fn has_roll_at(&self, x: usize, y: usize) -> bool {
        self.rolls.contains(&Coordinate { x, y })
    }

    fn neighbour_count(&self, x: usize, y: usize) -> usize {
        (y.checked_sub(1).unwrap_or(0)..=(y + 1))
            .flat_map(|y1| {
                (x.checked_sub(1).unwrap_or(0)..=(x + 1))
                    .filter(move |&x1| x != x1 || y != y1)
                    .map(move |x1| Coordinate { x: x1, y: y1 })
                    .filter(|coord| self.rolls.contains(coord))
            })
            .count()
    }

    fn count_accessible_rolls(&self) -> usize {
        self.rolls
            .iter()
            .filter(|&roll| self.neighbour_count(roll.x, roll.y) < 4)
            .count()
    }
}
/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input.txt").expect("Failed to read file");

    let grid = Grid::from(&contents);

    println!("{} rolls are accessible", grid.count_accessible_rolls());
}

#[cfg(test)]
mod tests {
    use crate::day_4::Grid;

    fn sample_input() -> String {
        "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.\n"
            .to_string()
    }

    fn sample_grid() -> Grid {
        Grid::from(&sample_input())
    }

    #[test]
    fn can_parse_input() {
        let grid = Grid::from(&sample_input());

        assert_eq!(grid.rolls.len(), 71);

        assert!(grid.has_roll_at(2, 0));
        assert!(grid.has_roll_at(8, 9));

        assert!(!grid.has_roll_at(0, 0));
        assert!(!grid.has_roll_at(9, 9));
    }

    #[test]
    fn can_count_filled_neighbours() {
        let grid = sample_grid();

        assert_eq!(grid.neighbour_count(0, 0), 2);
        assert_eq!(grid.neighbour_count(4, 4), 8);
    }

    #[test]
    fn can_count_accessible_rolls() {
        assert_eq!(sample_grid().count_accessible_rolls(), 13);
    }
}
