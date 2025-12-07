//! This is my solution for [Advent of Code - Day 6: _Trash Compactor_](https://adventofcode.com/2025/day/6)
//!
//! The majority of today's effort and the difference between parts is in the parsing of the input.
//! - [`parse_input`] parses the input in the format for part one
//! - [`parse_cephalopod_maths] parses the input in the format for part two
//! - [`Expression`] describes a parse expression, with [`Expression::result`] evaluating the expression
//! - [`sum_results`] combines the expression results into the puzzle solutions

use crate::day_6::Expression::*;
use std::fs;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-6-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 6.
pub fn run() {
    let contents = fs::read_to_string("res/day-6-input.txt").expect("Failed to read file");

    println!(
        "The total of all expressions is {}",
        sum_results(&parse_input(&contents))
    );

    println!(
        "The total of all cephalopod expressions is {}",
        sum_results(&parse_cephalopod_maths(&contents))
    );
}

/// Describes an expression as the operator and the list of parameters
#[derive(Debug, Eq, PartialEq)]
enum Expression {
    Add(Vec<u64>),
    Mul(Vec<u64>),
}

impl Expression {
    /// Evaluates the expression
    fn result(&self) -> u64 {
        match self {
            Add(nums) => nums.iter().sum(),
            Mul(nums) => nums.iter().product(),
        }
    }
}

/// Parse input as a columns of left to right numbers with the operator on the final row, each column representing an
/// expression
fn parse_input(input: &String) -> Vec<Expression> {
    let mut rows: Vec<Vec<&str>> = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let ops = rows
        .pop()
        .expect("Input must include a final line of operators");

    ops.into_iter()
        .enumerate()
        .map(|(idx, op)| {
            let nums = rows.iter().map(|row| row[idx].parse().unwrap()).collect();

            match op {
                "+" => Add(nums),
                "*" => Expression::Mul(nums),
                _ => unreachable!("Operators must be + or *"),
            }
        })
        .collect()
}

/// Parse the input as blocks of columnar numbers, with most significant digits at the top, missing digits are ignored
fn parse_cephalopod_maths(input: &String) -> Vec<Expression> {
    let mut rows: Vec<Vec<char>> = input
        .trim_end_matches("\n")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let ops = rows
        .pop()
        .expect("Input must include a final line of operators");
    let mut current_nums = Vec::new();
    let mut expressions = Vec::new();

    for (idx, op) in ops.iter().enumerate().rev() {
        let num = rows.iter().fold(0, |acc, row| {
            match row.get(idx).unwrap_or(&' ').to_digit(10) {
                Some(digit) => acc * 10 + digit as u64,
                None => acc,
            }
        });

        if num > 0 {
            current_nums.push(num)
        }

        match op {
            '+' => {
                expressions.push(Add(current_nums));
                current_nums = Vec::new();
            }
            '*' => {
                expressions.push(Mul(current_nums));
                current_nums = Vec::new();
            }
            _ => {}
        }
    }

    expressions
}

/// Reduce the parsed expressions into the puzzle solution
fn sum_results(expressions: &Vec<Expression>) -> u64 {
    expressions.iter().map(Expression::result).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_6::*;

    fn example_input() -> String {
        "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  \n"
            .to_string()
    }

    fn sample_expressions() -> Vec<Expression> {
        vec![
            Mul(vec![123, 45, 6]),
            Add(vec![328, 64, 98]),
            Mul(vec![51, 387, 215]),
            Add(vec![64, 23, 314]),
        ]
    }

    fn sample_cephalopod_expressions() -> Vec<Expression> {
        vec![
            Add(vec![4, 431, 623]),
            Mul(vec![175, 581, 32]),
            Add(vec![8, 248, 369]),
            Mul(vec![356, 24, 1]),
        ]
    }

    #[test]
    fn can_parse_input() {
        assert_eq!(parse_input(&example_input()), sample_expressions())
    }

    #[test]
    fn can_parse_cephalopod_math() {
        assert_eq!(
            parse_cephalopod_maths(&example_input()),
            sample_cephalopod_expressions()
        )
    }

    #[test]
    fn can_calculate_results() {
        assert_eq!(Mul(vec![123, 45, 6]).result(), 33210);
        assert_eq!(Add(vec![328, 64, 98]).result(), 490);
        assert_eq!(Mul(vec![51, 387, 215]).result(), 4243455);
        assert_eq!(Add(vec![64, 23, 314]).result(), 401);
    }

    #[test]
    fn can_sum_results() {
        assert_eq!(sum_results(&sample_expressions()), 4277556);
        assert_eq!(sum_results(&sample_cephalopod_expressions()), 3263827);
    }
}
