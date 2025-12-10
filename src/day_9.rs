//! This is my solution for [Advent of Code - Day 9: _Movie Theater_](https://adventofcode.com/2025/day/9)
//!
//!

use crate::day_9::LineSegment::{Horizontal, Vertical};
use crate::day_9::Turn::{Anticlockwise, Clockwise};
use itertools::Itertools;
use std::collections::HashMap;
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

    println!(
        "The largest interior rectangle has area {}",
        find_largest_interior_rectangle(&tiles)
    );
}

type Tile = (usize, usize);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Turn {
    Clockwise,
    Anticlockwise,
}

impl Turn {
    fn from(
        inwards_start: usize,
        inwards_end: usize,
        outwards_start: usize,
        outwards_end: usize,
    ) -> Turn {
        if (inwards_start < inwards_end) == (outwards_start < outwards_end) {
            Clockwise
        } else {
            Anticlockwise
        }
    }

    fn inverse(&self) -> Turn {
        match self {
            Clockwise => Anticlockwise,
            Anticlockwise => Clockwise,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum LineSegment {
    Horizontal { row: usize, cols: (usize, usize) },
    Vertical { col: usize, rows: (usize, usize) },
}

impl LineSegment {
    fn inverse(&self) -> Self {
        match self {
            &Horizontal {
                row,
                cols: (start, end),
            } => Horizontal {
                row,
                cols: (end, start),
            },
            &Vertical {
                col,
                rows: (start, end),
            } => Vertical {
                col,
                rows: (end, start),
            },
        }
    }

    fn normalise(&self) -> Self {
        match self {
            &Horizontal {
                cols: (start, end), ..
            }
            | &Vertical {
                rows: (start, end), ..
            } if start > end => self.inverse(),
            _ => self.clone(),
        }
    }

    fn intersections(&self, bounds: &Vec<BorderSegment>) -> usize {
        bounds
            .iter()
            .filter(|border_segment| border_segment.intersects(self))
            .count()
    }

    fn coords(&self) -> Vec<Tile> {
        match self {
            &Horizontal {
                row,
                cols: (start, end),
            } => (start..=end).map(|col| (row, col)).collect(),
            &Vertical {
                col,
                rows: (start, end),
            } => (start..=end).map(|row| (row, col)).collect(),
        }
    }

    fn is_wholly_within(
        &self,
        bounds: &Vec<BorderSegment>,
        cache: &mut HashMap<Tile, bool>,
    ) -> bool {
        self.coords().iter().all(|(row, col)| {
            if let Some(&result) = cache.get(&(*row, *col)) {
                return result;
            }

            let line_to_start = Horizontal {
                row: *row,
                cols: (0, *col),
            };

            let result = bounds.iter().any(|segment| segment.line.contains(row, col))
                || (line_to_start.intersections(bounds) % 2 == 1);

            cache.insert((*row, *col), result);

            result
        })
    }

    fn contains(&self, that_row: &usize, that_col: &usize) -> bool {
        match self {
            Horizontal {
                row: this_row,
                cols: (start, end),
            } if this_row == that_row => start <= that_col && end >= that_col,
            Vertical {
                col: this_col,
                rows: (start, end),
            } if this_col == that_col => start <= that_row && end >= that_row,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct BorderSegment {
    line: LineSegment,
    ends: (Turn, Turn),
}

impl BorderSegment {
    fn normalise(&self) -> BorderSegment {
        let normalised_line = self.line.normalise();

        if normalised_line == self.line {
            self.clone()
        } else {
            BorderSegment {
                line: self.line.inverse(),
                ends: (self.ends.1.inverse(), self.ends.0.inverse()),
            }
        }
    }

    fn intersects(&self, line: &LineSegment) -> bool {
        match (self.line, *line) {
            (
                Horizontal {
                    row: border_fixed,
                    cols: (border_start, border_end),
                },
                Horizontal {
                    row: line_fixed,
                    cols: (line_start, line_end),
                },
            )
            | (
                Vertical {
                    col: border_fixed,
                    rows: (border_start, border_end),
                },
                Vertical {
                    col: line_fixed,
                    rows: (line_start, line_end),
                },
            ) => {
                if border_fixed == line_fixed && line_start < border_start && line_end > border_end
                {
                    self.ends.0 != self.ends.1
                } else {
                    false
                }
            }
            (
                Horizontal {
                    row,
                    cols: (col_start, col_end),
                },
                Vertical {
                    col,
                    rows: (row_start, row_end),
                },
            )
            | (
                Vertical {
                    col,
                    rows: (row_start, row_end),
                },
                Horizontal {
                    row,
                    cols: (col_start, col_end),
                },
            ) => row > row_start && row < row_end && col > col_start && col < col_end,
        }
    }
}

fn parse_input(input: &String) -> Vec<Tile> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn rectangle_area((&(x_a, y_a), &(x_b, y_b)): (&Tile, &Tile)) -> usize {
    (x_a.abs_diff(x_b) + 1) * (y_a.abs_diff(y_b) + 1)
}

fn find_largest_rectangle(tiles: &Vec<Tile>) -> usize {
    tiles
        .iter()
        .tuple_combinations()
        .map(rectangle_area)
        .max()
        .unwrap()
}

fn get_boundary_segments(tiles: &Vec<Tile>) -> Vec<BorderSegment> {
    tiles
        .iter()
        .cloned()
        .circular_tuple_windows()
        .map(
            |(
                (prev_col, prev_row),
                (start_col, start_row),
                (end_col, end_row),
                (next_col, next_row),
            )| {
                if start_col == end_col {
                    BorderSegment {
                        line: Vertical {
                            col: start_col,
                            rows: (start_row, end_row),
                        },
                        ends: (
                            Turn::from(prev_col, start_col, start_row, end_row),
                            Turn::from(start_row, end_row, end_col, next_col).inverse(),
                        ),
                    }
                    .normalise()
                } else {
                    BorderSegment {
                        line: Horizontal {
                            row: start_row,
                            cols: (start_col, end_col),
                        },
                        ends: (
                            Turn::from(prev_row, start_row, start_col, end_col).inverse(),
                            Turn::from(start_col, end_col, end_row, next_row),
                        ),
                    }
                }
                .normalise()
            },
        )
        .collect()
}

fn is_rectangle_fully_within_boundary(
    corner_a: &Tile,
    corner_b: &Tile,
    segments: &Vec<BorderSegment>,
    cache: &mut HashMap<Tile, bool>,
) -> bool {
    let rectangle_segments = get_boundary_segments(&vec![
        *corner_a,
        (corner_a.0, corner_b.1),
        *corner_b,
        (corner_b.0, corner_a.1),
    ]);

    rectangle_segments
        .iter()
        .all(|segment| segment.line.is_wholly_within(segments, cache))
}

fn find_largest_interior_rectangle(tiles: &Vec<Tile>) -> usize {
    let segments = get_boundary_segments(tiles);
    let mut cache: HashMap<Tile, bool> = HashMap::new();

    tiles
        .iter()
        .tuple_combinations()
        .map(|(corner_a, corner_b)| (rectangle_area((corner_a, corner_b)), corner_a, corner_b))
        .sorted_by_key(|&(area, _, _)| area)
        .rev()
        .find(|&(_, corner_a, corner_b)| {
            is_rectangle_fully_within_boundary(corner_a, corner_b, &segments, &mut cache)
        })
        .unwrap()
        .0
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

    fn sample_segments() -> Vec<BorderSegment> {
        vec![
            BorderSegment {
                line: Vertical {
                    col: 11,
                    rows: (1, 7),
                },
                ends: (Clockwise, Clockwise),
            },
            BorderSegment {
                line: Horizontal {
                    row: 7,
                    cols: (9, 11),
                },
                ends: (Anticlockwise, Anticlockwise),
            },
            BorderSegment {
                line: Vertical {
                    col: 9,
                    rows: (5, 7),
                },
                ends: (Clockwise, Anticlockwise),
            },
            BorderSegment {
                line: Horizontal {
                    row: 5,
                    cols: (2, 9),
                },
                ends: (Anticlockwise, Clockwise),
            },
            BorderSegment {
                line: Vertical {
                    col: 2,
                    rows: (3, 5),
                },
                ends: (Anticlockwise, Anticlockwise),
            },
            BorderSegment {
                line: Horizontal {
                    row: 3,
                    cols: (2, 7),
                },
                ends: (Clockwise, Anticlockwise),
            },
            BorderSegment {
                line: Vertical {
                    col: 7,
                    rows: (1, 3),
                },
                ends: (Anticlockwise, Clockwise),
            },
            BorderSegment {
                line: Horizontal {
                    row: 1,
                    cols: (7, 11),
                },
                ends: (Clockwise, Clockwise),
            },
        ]
    }

    #[test]
    fn can_find_largest_rectangle() {
        assert_eq!(find_largest_rectangle(&sample_tiles()), 50);
        assert_eq!(find_largest_interior_rectangle(&sample_tiles()), 24);
    }

    #[test]
    fn can_find_boundary() {
        assert_eq!(get_boundary_segments(&sample_tiles()), sample_segments());
    }

    #[test]
    fn can_determine_if_rectangle_is_inside_the_lines() {
        let sample_segments = sample_segments();

        assert!(is_rectangle_fully_within_boundary(
            &(7, 3),
            &(11, 1),
            &sample_segments,
            &mut HashMap::new()
        ));
        assert!(is_rectangle_fully_within_boundary(
            &(9, 5),
            &(9, 7),
            &sample_segments,
            &mut HashMap::new()
        ));
        assert!(is_rectangle_fully_within_boundary(
            &(9, 5),
            &(2, 3),
            &sample_segments,
            &mut HashMap::new()
        ));
        assert!(!is_rectangle_fully_within_boundary(
            &(2, 5),
            &(11, 1),
            &sample_segments,
            &mut HashMap::new()
        ));
        assert!(!is_rectangle_fully_within_boundary(
            &(2, 5),
            &(7, 1),
            &sample_segments,
            &mut HashMap::new()
        ));
    }

    #[test]
    fn test_concave() {
        let tiles: Vec<Tile> = vec![
            (1, 1),
            (9, 1),
            (9, 9),
            (0, 9),
            (0, 6),
            (7, 6),
            (7, 4),
            (1, 4),
        ];
        let segments = get_boundary_segments(&tiles);

        assert!(!is_rectangle_fully_within_boundary(
            &(1, 1),
            &(9, 9),
            &segments,
            &mut HashMap::new()
        ));
    }

    #[test]
    fn test_h() {
        let tiles: Vec<Tile> = vec![
            (1, 0),
            (3, 0),
            (3, 6),
            (16, 6),
            (16, 0),
            (18, 0),
            (18, 9),
            (13, 9),
            (13, 7),
            (6, 7),
            (6, 9),
            (1, 9),
        ];

        let segments = get_boundary_segments(&tiles);

        assert!(!is_rectangle_fully_within_boundary(
            &(1, 9),
            &(18, 0),
            &segments,
            &mut HashMap::new()
        ));
        assert_eq!(find_largest_interior_rectangle(&tiles), 30);
    }
}
