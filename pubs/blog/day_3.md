---
day: 3
tags: [ post ]
header: 'Day 3: Lobby'
---

Given a list of battery banks, represented by lines of digits, find and sum the highest "joltages" for each bank.

## Parse input

Continuing from previous days, the parsing is fairly simple, turning the input in to a list of `BatteryBanks`, that
are in turn lists of digits. The test comes from the puzzle examples, and I can use a nested loop to parse the input.

```rust
type BatteryBank = Vec<u32>;

fn example_banks() -> Vec<BatteryBank> {
    vec![
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
        vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
        vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
    ]
}

#[test]
fn can_parse_input() {
    let example_input = "987654321111111\n\
            811111111111119\n\
            234234234234278\n\
            818181911112111"
        .to_string();
    
    assert_eq!(parse_input(&example_input), example_banks());
}

fn parse_input(input: &String) -> Vec<BatteryBank> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|battery| battery.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}
```

## Part one

I'm keen to try and do this in a single pass of a battery bank. I can track the maximum in a single pass, but need to
avoid including the last digit. If I find a new maximum, any existing second digit maximum is invalid as the number
starting with the new maximum is always going to be greater.

First a test.

```rust
#[test]
fn can_find_highest_joltage_in_battery_bank() {
    assert_eq!(
        find_highest_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
        98
    );
    assert_eq!(
        find_highest_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
        89
    );
    assert_eq!(
        find_highest_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
        78
    );
    assert_eq!(
        find_highest_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        92
    );
}
```

I can use `Itertools::tuple_windows` to walk through the bank in pairs, and track both maximums in parallel,
resetting both if a new maximum is found.

```rust
fn find_highest_joltage(bank: &BatteryBank) -> u32 {
    let mut first_max = &0;
    let mut second_max = &0;
    
    for (battery_a, battery_b) in bank.into_iter().tuple_windows() {
        if battery_a > first_max {
            first_max = battery_a;
            second_max = battery_b;
        } else if battery_b > second_max {
            second_max = battery_b
        }
    }
    
    first_max * 10 + second_max
}
```

Summing the results can be done with built-in iterators.

```rust
#[test]
fn can_sum_highest_joltages() {
    assert_eq!(sum_highest_joltage(&example_banks()), 357)
}

fn sum_highest_joltage(banks: &Vec<BatteryBank>) -> u32 {
    banks.iter().map(find_highest_joltage).sum()
}
```

## Part two

To extend part one, I now need to do the same, but find a 12-digit number. I could write this all out for 12-digits,
but I can further extract it to work with a list of maximum digits. I can reuse both functions, adding a digits
parameter to differentiate the parts.

```rust
#[test]
fn can_find_highest_joltage_in_battery_bank() {
    // ...
    assert_eq!(
        find_highest_joltage(&vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
        987654321111
    );
    assert_eq!(
        find_highest_joltage(&vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
        811111111119
    );
    assert_eq!(
        find_highest_joltage(&vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
        434234234278
    );
    assert_eq!(
        find_highest_joltage(&vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
        888911112111
    );
}

#[test]
fn can_sum_highest_joltages() {
    assert_eq!(sum_highest_joltage(&example_banks(), 2), 357);
    assert_eq!(sum_highest_joltage(&example_banks(), 12), 3121910778619);
}
```

There isn't an easy way to call `tuple_windows` with a variable length, so I fall back to managing it manually. I
use an outer loop tracking where I'm looking for a maximum first digit, and then track through the rest of the maximums
I'm trying to find, updating if a higher digit is found. If we do find one, the remaining digits are set to the current
values, as a shortcut to resetting them to 0, and then immediately setting them to the relevant digit which is
automatically higher.

```rust
fn find_highest_joltage(bank: &BatteryBank, digits: usize) -> u64 {
    let mut max_digits = vec![0; digits];
    
    for start in 0..=(bank.len() - digits) {
        let mut set_digit = false;
        for current in 0..digits {
            if max_digits[current] < bank[start + current] || set_digit {
                set_digit = true;
                max_digits[current] = bank[start + current];
            }
        }
    }
    
    max_digits
        .iter()
        .fold(0, |acc, &digit| acc * 10 + digit as u64)
}
```

The sum stays the same apart from passing the required number of digits through to `find_highest_joltage`.

```rust
fn sum_highest_joltage(banks: &Vec<BatteryBank>, digits: usize) -> u64 {
    banks
        .iter()
        .map(|bank| find_highest_joltage(bank, digits))
        .sum()
}
```
