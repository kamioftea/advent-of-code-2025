//! This is my solution for [Advent of Code - Day 7: _Laboratories_](https://adventofcode.com/2025/day/7)
//!
//!

use std::collections::HashSet;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-7-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 7.
pub fn run() {
    let contents = fs::read_to_string("res/day-7-input.txt").expect("Failed to read file");

    let manifold = TachyonManifold::from(&contents);

    println!(
        "The tachyon beam was split {} times",
        manifold.count_splits()
    );

    println!(
        "The tachyon particle could take {} paths",
        manifold.count_paths()
    );
}

type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct TachyonManifold {
    start: Coordinate,
    splitters: HashSet<Coordinate>,
    width: usize,
    height: usize,
}

impl From<&String> for TachyonManifold {
    fn from(input: &String) -> TachyonManifold {
        let mut start = None;
        let mut splitters = HashSet::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => start = Some((x, y)),
                    '^' => {
                        splitters.insert((x, y));
                    }
                    _ => {}
                }
            }
            width = line.len();
            height += 1;
        }

        TachyonManifold {
            start: start.expect("Thin input should include a start position"),
            splitters,
            width,
            height,
        }
    }
}

impl TachyonManifold {
    fn count_splits(&self) -> usize {
        let mut splits = 0;
        let (initial_beam, start_row) = self.start;
        let mut beams: HashSet<usize> = vec![initial_beam].into_iter().collect();

        for y in start_row..self.height {
            for x in beams.clone() {
                if self.splitters.contains(&(x, y)) {
                    beams.remove(&x);
                    beams.insert(x - 1);
                    beams.insert(x + 1);
                    splits += 1
                }
            }
        }

        splits
    }

    fn count_paths(&self) -> usize {
        let (initial_beam, start_row) = self.start;
        let mut beams: Vec<usize> = vec![0; self.width + 1].into_iter().collect();
        beams[initial_beam] = 1;

        for y in start_row..self.height {
            for (x, paths) in beams.clone().into_iter().enumerate() {
                if self.splitters.contains(&(x, y)) {
                    beams[x] = 0;
                    beams[x - 1] += paths;
                    beams[x + 1] += paths;
                }
            }
        }

        beams.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::*;

    fn sample_input() -> String {
        "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .to_string()
    }

    #[test]
    fn can_parse_input() {
        let manifold = TachyonManifold::from(&sample_input());

        assert_eq!(manifold.start, (7, 0));

        assert_eq!(manifold.splitters.len(), 22);
        assert!(manifold.splitters.contains(&(7, 2)));
        assert!(manifold.splitters.contains(&(6, 4)));
        assert!(manifold.splitters.contains(&(8, 4)));

        assert_eq!(manifold.width, 15);
        assert_eq!(manifold.height, 16);
    }

    #[test]
    fn can_split_beams() {
        let manifold = TachyonManifold::from(&sample_input());

        assert_eq!(manifold.count_splits(), 21);
    }

    #[test]
    fn can_count_possible_paths() {
        let manifold = TachyonManifold::from(&sample_input());

        assert_eq!(manifold.count_paths(), 40);
    }
}
