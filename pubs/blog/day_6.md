---
day: 6
tags: [ post ]
header: 'Day 6: Trash Compactor'
---

Today I have some maths homework to solve. The main complication is the input format requiring transposing the rows
of input into expressions arranged in columns.

## Parse input

This is most of the work for solving part one. I can use `String::split_whitespace()` to account for the variable
width of the numbers. I can then iterate through any one of those enumerated so that I have an index for the other rows.

First some types and tests.

```rust
#[derive(Debug, Eq, PartialEq)]
enum Expression {
    Add(Vec<u64>),
    Mul(Vec<u64>),
}

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

#[test]
fn can_parse_input() {
    assert_eq!(parse_input(&example_input()), sample_expressions())
}
```

Each row can be split on whitespace regardless. I then need to extract the row of operators, and might as well use that
as the row to iterate over. The test input has three rows of numbers, but my puzzle input four. So it needs to accept
an arbitrary amount.

```rust
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
```

## Part one

Once I have the expressions, I need to evaluate them ...

```rust
#[test]
fn can_calculate_results() {
    assert_eq!(Mul(vec![123, 45, 6]).result(), 33210);
    assert_eq!(Add(vec![328, 64, 98]).result(), 490);
    assert_eq!(Mul(vec![51, 387, 215]).result(), 4243455);
    assert_eq!(Add(vec![64, 23, 314]).result(), 401);
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
```

... and then sum the results.

```rust
#[test]
fn can_sum_results() {
    assert_eq!(sum_results(&sample_expressions()), 4277556);
}
```

## Part two

The twist today is I need to parse the input in a different way, parsing each column of the input as number, and an
entire column of spaces breaking up the expressions (that take up variable numbers of columns each). The operator can
also be used as a catch for the break between expressions, which makes it easier to parse it right to left.

```rust
fn sample_cephalopod_expressions() -> Vec<Expression> {
    vec![
        Add(vec![4, 431, 623]),
        Mul(vec![175, 581, 32]),
        Add(vec![8, 248, 369]),
        Mul(vec![356, 24, 1]),
    ]
}

#[test]
fn can_parse_cephalopod_math() {
    assert_eq!(
        parse_cephalopod_maths(&example_input()),
        sample_cephalopod_expressions()
    )
}

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
```

There was some awkwardness here, caused by a combination of my IDE and/or rustfmt stripping the trailing whitespace
from the test input, and me missing that `trim()` would be trimming the spaces as well as the newline. Once those were
resolved the expression parser and totalling functions don't need any changes.
