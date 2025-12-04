//! This is my solution for [Advent of Code - Day 4: _Printing Department_](https://adventofcode.com/2025/day/4)
//!
//! Today's solution is implemented by the [`PrintingDepartment`] struct, which holds the set of [`Roll`] locations.
//!
//! - [`PrintingDepartment::neighbour_count`] is used to determine which rolls are accessible and removable
//! - [`PrintingDepartment::count_accessible_rolls`] solves part one by counting the rolls with 3 or fewer
//!   neighbouring rolls
//! - [`PrintingDepartment::remove_accessible_rolls`] removes those rolls that are accessible from the department
//! - [`PrintingDepartment::count_removable_rolls`] recursively calls [`PrintingDepartment::remove_accessible_rolls`]
//!   until the system is stable and no further rolls are accessible, returning the total count of rolls that could be
//!   removed.

use std::collections::HashSet;
use std::fs;

/// represents the location of a roll of paper on the printing department floor
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Roll {
    x: usize,
    y: usize,
}

/// represents a printing department by the list of where rolls of paper are located
#[derive(Debug, Eq, PartialEq, Clone)]
struct PrintingDepartment {
    rolls: HashSet<Roll>,
}

impl From<&String> for PrintingDepartment {
    /// Interpret the puzzle input as a grid representing the department floor. The origin (0,0) is in the top-left,
    /// The position of `@` symbols represent where the rolls of paper are located
    fn from(value: &String) -> Self {
        let rolls = value
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, cell)| cell == '@')
                    .map(move |(x, _)| Roll { x, y })
            })
            .collect();

        PrintingDepartment { rolls }
    }
}

impl PrintingDepartment {
    /// Given a location on the department floor, how many of the up to 8 adjacent locations have rolls of paper.
    /// The room's walls are counted as empty.
    fn neighbour_count(&self, x: usize, y: usize) -> usize {
        (y.checked_sub(1).unwrap_or(0)..=(y + 1))
            .flat_map(|y1| {
                (x.checked_sub(1).unwrap_or(0)..=(x + 1))
                    .filter(move |&x1| x != x1 || y != y1)
                    .map(move |x1| Roll { x: x1, y: y1 })
                    .filter(|coord| self.rolls.contains(coord))
            })
            .count()
    }

    /// Solves part one - How many of the rolls have less than four neighbours
    fn count_accessible_rolls(&self) -> usize {
        self.rolls
            .iter()
            .filter(|roll| self.neighbour_count(roll.x, roll.y) < 4)
            .count()
    }

    /// Returns a copy of the grid with [`accessible_rolls`](PrintingDepartment::count_accessible_rolls) removed.
    fn remove_accessible_rolls(&self) -> PrintingDepartment {
        let rolls = self
            .rolls
            .iter()
            .filter(|roll| self.neighbour_count(roll.x, roll.y) >= 4)
            .cloned()
            .collect();

        PrintingDepartment { rolls }
    }

    /// Solves part two - Remove rolls recursively until all rolls are unremovable, returning how many rolls could be
    /// removed
    fn count_removable_rolls(&self) -> usize {
        let next = self.remove_accessible_rolls();
        let removed = self.rolls.len() - next.rolls.len();

        if removed == 0 {
            0
        } else {
            removed + next.count_removable_rolls()
        }
    }
}

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-4-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 4.
pub fn run() {
    let contents = fs::read_to_string("res/day-4-input.txt").expect("Failed to read file");

    let dept = PrintingDepartment::from(&contents);

    println!("{} rolls are accessible", dept.count_accessible_rolls());

    println!("{} rolls can be removed", dept.count_removable_rolls());
}

#[cfg(test)]
mod tests {
    use crate::day_4::{PrintingDepartment, Roll};

    impl PrintingDepartment {
        fn has_roll_at(&self, x: usize, y: usize) -> bool {
            self.rolls.contains(&Roll { x, y })
        }
    }

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

    fn sample_dept() -> PrintingDepartment {
        PrintingDepartment::from(&sample_input())
    }

    #[test]
    fn can_parse_input() {
        let dept = PrintingDepartment::from(&sample_input());

        assert_eq!(dept.rolls.len(), 71);

        assert!(dept.has_roll_at(2, 0));
        assert!(dept.has_roll_at(8, 9));
        assert!(dept.has_roll_at(9, 7));

        assert!(!dept.has_roll_at(0, 0));
        assert!(!dept.has_roll_at(9, 9));
        assert!(!dept.has_roll_at(10, 10));
    }

    #[test]
    fn can_count_filled_neighbours() {
        let dept = sample_dept();

        assert_eq!(dept.neighbour_count(0, 0), 2);
        assert_eq!(dept.neighbour_count(4, 4), 8);
    }

    #[test]
    fn can_count_accessible_rolls() {
        assert_eq!(sample_dept().count_accessible_rolls(), 13);
    }

    #[test]
    fn can_remove_accessible_rolls() {
        let dept = sample_dept();

        let next_dept = dept.remove_accessible_rolls();

        assert_eq!(next_dept.rolls.len(), 58);
        assert_eq!(next_dept.count_accessible_rolls(), 12);
    }

    #[test]
    fn can_count_possible_removals() {
        assert_eq!(sample_dept().count_removable_rolls(), 43);
    }
}
