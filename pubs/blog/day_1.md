---
day: 1
tags: [ post ]
header: 'Day 1: Secret Entrance'
---

I had mostly sorted my repository setup over the weekend, so I was able to jump straight into solving day 1. Today we're
turning safe dial instructions into a password (number) to gain access to Santa's secret North Pole base.

## Parsing the input

I decided to represent the instructions as a direction (`Left` or `Right`) and a distance. It would be possible to
represent direction with signed integers, and `Clockwise` or `AntiClockwise` make more sense to me, but I'd rather
keep to the language of the domain.

```rust
#[derive(Debug, Eq, PartialEq)]
enum Direction {
    /// Turn dial clockwise
    Right,
    /// Turn dial anticlockwise
    Left,
}

type Instruction = (Direction, u32);
```

The puzzle provides some sample input for testing.

```rust
fn sample_input() -> String {
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_string()
}

fn sample_instructions() -> Vec<Instruction> {
    vec![
        (Left, 68),
        (Left, 30),
        (Right, 48),
        (Left, 5),
        (Right, 60),
        (Left, 55),
        (Left, 1),
        (Left, 99),
        (Right, 14),
        (Left, 82),
    ]
}

#[test]
fn can_parse_input() {
    assert_eq!(parse_input(sample_input()), sample_instructions());
}
```

Parsing can then be done by matching on the letter at the start of each line, and parsing the rest as a number.

```rust
fn parse_input(input: String) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (direction, distance_str) = line.split_at(1);
            let distance = distance_str.parse::<u32>().unwrap();
            
            match direction {
                "R" => (Direction::Right, distance),
                "L" => (Direction::Left, distance),
                _ => unreachable!("Direction is always L or R"),
            }
        })
        .collect()
}
```

## Part one

The password for part 1 is the number of times the dial is at 0 once an instruction has been completed. I'm going to
break this down into getting a new position given a dial and a starting point, then apply that as I loop through the
instructions, counting any 0's seen.

There are some examples in the puzzle input that can be used as tests.

```rust
#[test]
fn can_turn_dial() {
    assert_eq!(turn_dial(11, (Right, 8)), 19);
    assert_eq!(turn_dial(19, (Left, 19)), 0);
    
    assert_eq!(turn_dial(5, (Left, 10)), 95);
    assert_eq!(turn_dial(95, (Right, 5)), 0);
}
```

I can implement this by matching on the direction, and doing some modulo arithmetic. Since we're using unsigned integers
we need to make sure things stay positive when turning the dial left, i.e. towards 0.

- Add 100 to the position, which doesn't change the result % 100, but means a partial turn is always ends up positive
- Some of the puzzle input lines have turns > 100 (which are not represented in the puzzle example) - so the turn
  distance also needs to be `% 100` so that only the final partial turn is applied, keepin things positive.

```rust
fn turn_dial(position: u32, instruction: Instruction) -> u32 {
    match instruction {
        (Direction::Right, dist) => (position + dist) % 100,
        (Direction::Left, dist) => (position + 100 - (dist % 100)) % 100,
    }
}
```

The puzzle instructions give an expected result for the sample instructions we encoded for the parsing test.

```rust
#[test]
fn can_count_zero_positions() {
    assert_eq!(count_zero_positions(sample_instructions()), 3)
}
```

The initial temptation is to reach for a reducer of some form, and maybe some `Itertools` magic - but as I start writing
it I quickly decide that a for loop is going to be more readable.

```rust
fn count_zero_positions(instructions: Vec<Instruction>) -> usize {
    let mut position = 50;
    let mut count = 0;
    for instruction in instructions {
        position = turn_dial(position, instruction);
        if position == 0 {
            count += 1
        }
    }
    
    count
}
```

## Part two

The twist is that I also need to keep track of times the dial passes zero, possibly multiple times per instruction for
those where the distance exceeds 99.

There is another example for the sample instruction list.

```rust
#[test]
fn can_count_zero_passes() {
    assert_eq!(count_zero_passes(&sample_instructions()), 6)
}
```

I realise I'm going to need to return the number of times the dial passed zero alongside the new position when
applying an instruction. I need to update the `can_turn_dial` test to match the function now returning a pair, and to
cover cases where the dial passes 0 multiple times, and starts at 0.

```rust
#[test]
fn can_turn_dial() {
    assert_eq!(turn_dial(11, &(Right, 8)).1, 19);
    assert_eq!(turn_dial(19, &(Left, 19)).1, 0);
    
    assert_eq!(turn_dial(5, &(Left, 10)).1, 95);
    assert_eq!(turn_dial(95, &(Right, 5)).1, 0);
    
    assert_eq!(turn_dial(50, &(Right, 949)), (9, 99));
    assert_eq!(turn_dial(50, &(Right, 950)), (10, 0));
    
    assert_eq!(turn_dial(50, &(Left, 949)), (9, 1));
    assert_eq!(turn_dial(50, &(Left, 950)), (10, 0));
    
    assert_eq!(turn_dial(0, &(Right, 10)), (0, 10));
    assert_eq!(turn_dial(0, &(Left, 10)), (0, 90));
}
```

I initially add the updates to the existing match statement in `turn_dial`. This leaves it messy and hard to read,
so I break it out into smaller steps.

```rust
fn turn_dial(position: u32, (direction, distance): &Instruction) -> (u32, u32) {
    let full_rotations = distance / 100;
    let remaining_distance = distance % 100;
    
    let delta = match direction {
        Direction::Right => remaining_distance,
        Direction::Left => 100 - remaining_distance,
    };
    let new_position = position + delta;
    
    let passes_zero_again = position > 0
        && match direction {
        Direction::Right => new_position >= 100,
        Direction::Left => new_position <= 100,
    };
    
    (
        full_rotations + if passes_zero_again { 1 } else { 0 },
        new_position % 100,
    )
}
```

That working, I need a version of the instruction loop that counts all zeros.

```rust
fn count_zero_passes(instructions: &Vec<Instruction>) -> u32 {
    let mut position = 50;
    let mut count = 0;
    for instruction in instructions {
        let (zero_count, new_pos) = turn_dial(position, instruction);
        count += zero_count;
        position = new_pos;
    }
    
    count
}
```

Which solves part 2.
