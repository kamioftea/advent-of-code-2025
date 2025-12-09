//! This is my solution for [Advent of Code - Day 8: _Playground_](https://adventofcode.com/2025/day/8)
//!
//! [`parse_input`] turns the puzzle input into a list of [`JunctionBox`]es. Given both parts need the possible
//! connections, shortest first, I calculate the permutations and sort them once, grouping them with the associated
//! boxes in a [`DecorationProject`].
//!
//! - [`circuits_after_n_connections`] does the bulk of the work for part one, clustering the boxes into circuits using
//!   thr first `n` connections
//! - [`product_of_3_largest_circuits_after_n_connections`] wraps [`circuits_after_n_connections`] to produce the puzzle
//!   solution
//! - [`find_final_connection`] does most of the work for part two, merging the circuits until there is only one, then
//!   returning that final connection
//! - [`find_x_product_of_final_connection`] calculates the puzzle solution from the connection provided by
//!   [`find_final_connection`]

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-8-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 8.
pub fn run() {
    let contents = fs::read_to_string("res/day-8-input.txt").expect("Failed to read file");

    let project = parse_input(&contents);

    println!(
        "After 1000 connections the product of the three largest is {}",
        product_of_3_largest_circuits_after_n_connections(&project, 1000)
    );

    println!(
        "The product of x-coordinates of the final connection is {}",
        find_x_product_of_final_connection(&project)
    );
}

/// The location of a junction box in 3D space
type JunctionBox = (i64, i64, i64);

/// A connection between two [`JunctionBox`]es represented by the index of each in the list of junction boxes
type Connection = (usize, usize);

/// Combines the junction boxes with all their possible connections, sorted shortest first
struct DecorationProject {
    boxes: Vec<JunctionBox>,
    connections: Vec<Connection>,
}

/// Parses each line as a [`JunctionBox`] in the format `{x},{y},{z}`. Delegates to [`order_possible_connections`]
/// to build the connections once here so I don't have to repeat that expensive operation for both parts.
fn parse_input(input: &String) -> DecorationProject {
    let boxes = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .splitn(3, ',')
                .map(|num| num.parse().unwrap())
                .tuples()
                .next()
                .unwrap();

            (x, y, z)
        })
        .collect();

    let connections = order_possible_connections(&boxes);

    DecorationProject { boxes, connections }
}

/// The Euclidean distance between the two boxes in 3D space
fn connection_distance((x_a, y_a, z_a): &JunctionBox, (x_b, y_b, z_b): &JunctionBox) -> i64 {
    ((x_a - x_b).pow(2) + (y_a - y_b).pow(2) + (z_a - z_b).pow(2)).isqrt()
}

/// Calculate all the possible connections between boxes, and return these sorted shortest first, represented by a pair
/// of indices into the list of boxes
fn order_possible_connections(junction_boxes: &Vec<JunctionBox>) -> Vec<Connection> {
    junction_boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|&((_, a), (_, b))| connection_distance(a, b))
        .map(|((idx_a, _), (idx_b, _))| (idx_a, idx_b))
        .collect()
}

/// The bulk of the work for part one, combine junction boxes into circuits by joining `target_connections` connections,
/// smallest first. Returns the list of circuit sizes.
fn circuits_after_n_connections(
    project: &DecorationProject,
    target_connections: usize,
) -> Vec<usize> {
    let mut circuits: Vec<usize> = project
        .boxes
        .iter()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect();

    for &(a, b) in project.connections.iter().take(target_connections) {
        let circuit_a = circuits[a];
        let circuit_b = circuits[b];

        if circuit_a != circuit_b {
            circuits.iter_mut().for_each(|circuit_id| {
                if circuit_id == &circuit_b {
                    *circuit_id = circuit_a
                }
            });
        }
    }

    circuits.into_iter().counts().values().cloned().collect()
}

/// Use [`circuits_after_n_connections`] to find the circuit sizes. The puzzle solution is then the product of the
/// largest three.
fn product_of_3_largest_circuits_after_n_connections(
    project: &DecorationProject,
    connection_count: usize,
) -> usize {
    circuits_after_n_connections(project, connection_count)
        .iter()
        .sorted()
        .rev()
        .take(3)
        .product()
}

/// The bulk of the work for part two, combine junction boxes into a single circuit, combining by the smallest
/// connections until all junction boxes are included. Returns that final connection.
fn find_final_connection(project: &DecorationProject) -> Connection {
    let mut circuits: Vec<usize> = project
        .boxes
        .iter()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect();

    for &(a, b) in project.connections.iter() {
        let circuit_a = circuits[a];
        let circuit_b = circuits[b];

        if circuit_a != circuit_b {
            circuits.iter_mut().for_each(|circuit_id| {
                if circuit_id == &circuit_b {
                    *circuit_id = circuit_a
                }
            });
        }

        if circuits.iter().unique().count() == 1 {
            return (a, b);
        }
    }

    unreachable!()
}

/// Delegates to [`find_final_connection`], then calculates the puzzle solution by multiplying the x coordinates.
fn find_x_product_of_final_connection(project: &DecorationProject) -> i64 {
    let (a, b) = find_final_connection(project);

    project.boxes[a].0 * project.boxes[b].0
}

#[cfg(test)]
mod tests {
    use crate::day_8::*;

    fn sample_input() -> String {
        "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            .to_string()
    }

    fn sample_junction_boxes() -> Vec<JunctionBox> {
        vec![
            (162, 817, 812),
            (57, 618, 57),
            (906, 360, 560),
            (592, 479, 940),
            (352, 342, 300),
            (466, 668, 158),
            (542, 29, 236),
            (431, 825, 988),
            (739, 650, 466),
            (52, 470, 668),
            (216, 146, 977),
            (819, 987, 18),
            (117, 168, 530),
            (805, 96, 715),
            (346, 949, 466),
            (970, 615, 88),
            (941, 993, 340),
            (862, 61, 35),
            (984, 92, 344),
            (425, 690, 689),
        ]
    }

    fn sample_project() -> DecorationProject {
        let boxes = sample_junction_boxes();
        let connections = order_possible_connections(&boxes);

        DecorationProject { boxes, connections }
    }

    #[test]
    fn can_parse_coordinates() {
        let project = parse_input(&sample_input());
        assert_eq!(project.boxes, sample_junction_boxes());

        let shortest_connections: Vec<Connection> =
            project.connections.into_iter().take(4).collect();

        assert_eq!(
            shortest_connections,
            vec![(0, 19), (0, 7), (2, 13), (7, 19)]
        )
    }

    #[test]
    fn can_find_shortest_connections() {}

    #[test]
    fn can_merge_circuits() {
        assert_eq!(
            circuits_after_n_connections(&sample_project(), 10).len(),
            11
        );

        assert_eq!(
            product_of_3_largest_circuits_after_n_connections(&sample_project(), 10),
            40
        );
    }

    #[test]
    fn can_find_connection_that_merges_circuit() {
        assert_eq!(find_final_connection(&sample_project()), (10, 12));
        assert_eq!(find_x_product_of_final_connection(&sample_project()), 25272);
    }
}
