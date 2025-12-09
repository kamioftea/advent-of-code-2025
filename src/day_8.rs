//! This is my solution for [Advent of Code - Day 8: _Playground_](https://adventofcode.com/2025/day/8)
//!
//!

use itertools::Itertools;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-8-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 8.
pub fn run() {
    let contents = fs::read_to_string("res/day-8-input.txt").expect("Failed to read file");

    let junction_boxes = parse_input(&contents);

    println!(
        "After 1000 connections the product of the three largest is {}",
        product_of_3_largest_circuits_after_n_connections(&junction_boxes, 1000)
    );

    println!(
        "The product of x-coordinates of the final connection is {}",
        find_x_product_of_final_connection(&junction_boxes)
    );
}

type JunctionBox = (i64, i64, i64);

fn parse_input(input: &String) -> Vec<JunctionBox> {
    input
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
        .collect()
}

fn connection_distance((x_a, y_a, z_a): &JunctionBox, (x_b, y_b, z_b): &JunctionBox) -> i64 {
    ((x_a - x_b).pow(2) + (y_a - y_b).pow(2) + (z_a - z_b).pow(2)).isqrt()
}

fn order_possible_connections(junction_boxes: &Vec<JunctionBox>) -> Vec<(usize, usize)> {
    junction_boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|&((_, a), (_, b))| connection_distance(a, b))
        .map(|((idx_a, _), (idx_b, _))| (idx_a, idx_b))
        .collect()
}

fn circuits_after_n_connections(
    junction_boxes: &Vec<JunctionBox>,
    target_connections: u32,
) -> Vec<usize> {
    let mut circuits: Vec<usize> = junction_boxes
        .iter()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect();

    let mut connection_count = 0;

    for (a, b) in order_possible_connections(junction_boxes) {
        let circuit_a = circuits[a];
        let circuit_b = circuits[b];

        if circuit_a != circuit_b {
            circuits.iter_mut().for_each(|circuit_id| {
                if circuit_id == &circuit_b {
                    *circuit_id = circuit_a
                }
            });
        }

        connection_count += 1;
        if connection_count == target_connections {
            break;
        }
    }

    circuits.into_iter().counts().values().cloned().collect()
}

fn product_of_3_largest_circuits_after_n_connections(
    junction_boxes: &Vec<JunctionBox>,
    connection_count: u32,
) -> usize {
    circuits_after_n_connections(junction_boxes, connection_count)
        .iter()
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn find_final_connection(junction_boxes: &Vec<JunctionBox>) -> (usize, usize) {
    let mut circuits: Vec<usize> = junction_boxes
        .iter()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect();

    for (a, b) in order_possible_connections(junction_boxes) {
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

fn find_x_product_of_final_connection(junction_boxes: &Vec<JunctionBox>) -> i64 {
    let (a, b) = find_final_connection(junction_boxes);

    junction_boxes[a].0 * junction_boxes[b].0
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

    #[test]
    fn can_parse_coordinates() {
        assert_eq!(parse_input(&sample_input()), sample_junction_boxes())
    }

    #[test]
    fn can_find_shortest_connections() {
        let shortest_connections: Vec<(usize, usize)> =
            order_possible_connections(&sample_junction_boxes())
                .into_iter()
                .take(4)
                .collect();

        assert_eq!(
            shortest_connections,
            vec![(0, 19), (0, 7), (2, 13), (7, 19)]
        )
    }

    #[test]
    fn can_merge_circuits() {
        assert_eq!(
            circuits_after_n_connections(&sample_junction_boxes(), 10).len(),
            11
        );

        assert_eq!(
            product_of_3_largest_circuits_after_n_connections(&sample_junction_boxes(), 10),
            40
        );
    }

    #[test]
    fn can_find_connection_that_merges_circuit() {
        assert_eq!(find_final_connection(&sample_junction_boxes()), (10, 12));
        assert_eq!(
            find_x_product_of_final_connection(&sample_junction_boxes()),
            25272
        );
    }
}
