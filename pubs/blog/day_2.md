---
day: 2
tags: [ post ]
header: 'Day 2: Gift Shop'
---

Today I'm hunting down invalid ids in ranges of numbers. An invalid number is composed of a smaller number repeated
exactly twice.

## Parse input

I debated whether it was worth keeping the numbers as strings for ease of splitting, but I will still need them as
numbers when working with the range they define, and I can use powers, divisions, and log base 10 to manage the
splitting numerically.

I need a type for the range, and the samples from the puzzles can be tests.

```rust
type IdRange = (u64, u64);

fn sample_input() -> String {
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
     1698522-1698528,446443-446449,38593856-38593862,\
     565653-565659,824824821-824824827,2121212118-2121212124\n"
        .to_string()
}

fn sample_ranges() -> Vec<IdRange> {
    vec![
        (11, 22),
        (95, 115),
        (998, 1012),
        (1188511880, 1188511890),
        (222220, 222224),
        (1698522, 1698528),
        (446443, 446449),
        (38593856, 38593862),
        (565653, 565659),
        (824824821, 824824827),
        (2121212118, 2121212124),
    ]
}

#[test]
fn can_parse_input() {
    assert_eq!(parse_input(&sample_input()), sample_ranges());
}
```

The parsing is two stage:

- Split the line into ranges on the `,`s
- Split each range to min and max on the `-`

```rust

fn parse_input(input: &String) -> Vec<IdRange> {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect()
}
```

## Part one

The plan for getting ids from a range is:

- start with the first half of the number
- create an incrementing iterator
- map those values into the repeated number, and
- take numbers from the iterator until they are out of the range

First a test

```rust
#[test]
fn can_find_invalid_ids() {
    assert_eq!(find_invalid_ids(&(11, 22)), vec![11, 22]);
    assert_eq!(find_invalid_ids(&(95, 115)), vec![99]);
    assert_eq!(find_invalid_ids(&(998, 1012)), vec![1010]);
    assert_eq!(
        find_invalid_ids(&(1188511880, 1188511890)),
        vec![1188511885]
    );
    assert_eq!(find_invalid_ids(&(222220, 222224)), vec![222222]);
    assert_eq!(find_invalid_ids(&(1698522, 1698528)), Vec::<u64>::new());
    assert_eq!(find_invalid_ids(&(446443, 446449)), vec![446446]);
    assert_eq!(find_invalid_ids(&(38593856, 38593862)), vec![38593859]);
    assert_eq!(find_invalid_ids(&(565653, 565659)), Vec::<u64>::new());
    assert_eq!(find_invalid_ids(&(824824821, 824824827)), Vec::<u64>::new());
    assert_eq!(
        find_invalid_ids(&(2121212118, 2121212124)),
        Vec::<u64>::new()
    );
}
```

I was thinking I'd need to split the range into sub ranges with uniform magnitude (i.e. `17-1234` -> `17-99`,
`100-999` and `1000-1234`), then skip those where length was not even, but as I was writing it out, I realised
those cases are implicitly excluded because the generated numbers will always be an even length and e.g. for the range
`123-125`, it would start with `1212` and immediately return an empty list because the first item is already outside
the range.

```rust
fn find_invalid_ids(&(min, max): &IdRange) -> Vec<u64> {
    let starting_magnitude = (min.ilog10()) / 2;
    let first_half_of_number = min / 10u64.pow(starting_magnitude + 1);
    let first_power_of_ten = 10u64.pow(starting_magnitude);
    let start = (first_half_of_number).max(first_power_of_ten);
    
    (start..)
        .map(|base| format!("{base}{base}").parse::<u64>().unwrap_or(u64::MAX))
        .skip_while(|&invalid_id| invalid_id < min)
        .take_while(|&invalid_id| invalid_id <= max)
        .collect()
}
```

The first attempt didn't have the `skip_while`, but the test range `(565653, 565659)` highlights that it needs to be
included. `565653` starts with `565`, doubled is `565565`, which is smaller than the start of the range.

All that was left was to do that for each range and sum the results.

```rust
#[test]
fn can_sum_invalid_ids_for_range_list() {
    assert_eq!(sum_invalid_ids(&sample_ranges()), 1227775554);
}

fn sum_invalid_ids(ranges: &Vec<IdRange>) -> u64 {
    ranges.iter().flat_map(find_invalid_ids).sum()
}
```

## Part two

The puzzle is expanded to include numbers that are repeated more than twice. There are lower and upper bounds to the
number of repeats (2 and the length of the range maximum). I decide to make `find_invalid_ids` work for any given repeat
count, then work through the range of possible repeats, and return the unique ids across all of them.

First refactoring `find_invalid_ids` -> `find_invalid_ids_for_repeats`. I add a basic test for three repeats, but
most of the edge cases will be covered in the tests for the full repeat range.

```rust
#[test]
fn can_find_invalid_pair_ids() {
    // ...
    assert_eq!(find_invalid_ids_for_repeats(&(95, 115), 3), vec![111]);
}

fn find_invalid_ids_for_repeats(&(min, max): &IdRange, repeats: u32) -> Vec<u64> {
    let starting_magnitude = (min.ilog10()) / repeats;
    let start = 10u64.pow(starting_magnitude);
    
    (start..)
        .map(|base| {
            format!("{base}")
                .repeat(repeats as usize)
                .parse::<u64>()
                .unwrap_or(u64::MAX)
        })
        .skip_while(|&invalid_id| invalid_id < min)
        .take_while(|&invalid_id| invalid_id <= max)
        .collect()
}
```

This uncovered a bug in part 1 that didn't affect the puzzle input because the ranges don't hit it. But e.g. the range
`999-1111` would fail to find `1111`. It would try `99`, then `100100` which is outside the range. This manifests in
the test for 3 repeats because `95-115` -> `9` -> `999` and fails to find `111`. The quick fix is to start counting from
the largest power of 10 below the start of the range. This adds in a much larger list of numbers, but it runs quick
enough. The tests are still passing, so all is good.

Onto the full set of possible repeats. I turn the examples into tests.

```rust
#[test]
fn can_find_all_invalid_ids() {
    vec![
        ((11, 22), vec![11, 22]),
        ((95, 115), vec![99, 111]),
        ((998, 1012), vec![999, 1010]),
        ((1188511880, 1188511890), vec![1188511885]),
        ((222220, 222224), vec![222222]),
        ((1698522, 1698528), Vec::<u64>::new()),
        ((38593856, 38593862), vec![38593859]),
        ((565653, 565659), vec![565656]),
        ((824824821, 824824827), vec![824824824]),
        ((2121212118, 2121212124), vec![2121212121]),
    ]
        .into_iter()
        .for_each(|(range, invalid_ids)| {
            assert_contains_in_any_order(find_invalid_ids_for_range(&range), invalid_ids);
        });
}
```

I can use `flat_map` and `unique` to get the required ids.

```rust
fn find_invalid_ids_for_range(range: &IdRange) -> Vec<u64> {
    (2..=range.1.ilog10() + 1)
        .flat_map(|repeats| find_invalid_ids_for_repeats(range, repeats))
        .unique()
        .collect()
}
```

Then the puzzle solution is very similar to part one, but using the `find_invalid_ids_for_range`.

```rust
#[test]
fn can_sum_invalid_ids_for_range_list() {
    assert_eq!(sum_invalid_ids(&sample_ranges()), 4174379265);
}

fn sum_invalid_ids(ranges: &Vec<IdRange>) -> u64 {
    ranges
        .iter()
        .flat_map(|range| find_invalid_ids_for_range(range))
        .sum()
}
```

## Optimisation

The puzzle input runs in 30ms, but the quick fix to start much lower than needed is bugging me. I revisit an idea to
split the range into subranges of a uniform magnitude. This would mean for `95-115` and repeats = `3` it would be
split to `95-99`. `100-115`. `95-99` could be skipped entirely as it's length is not a multiple of `3`. Then
`100-115` starts at `1` and tries `111`, then aborts when `222 > 115`.

I want to keep the same api for `find_invalid_ids_for_repeats` so I move the current logic into an inner function, and
restore the previous logic for starting from the first part of the number. Then split the range and call the inner
function for those that are factors of `repeats`.

```rust
fn find_invalid_ids_for_repeats(&(min, max): &IdRange, repeats: u32) -> Vec<u64> {
    fn find_invalid_ids_for_subrange(&(min, max): &IdRange, repeats: u32) -> Vec<u64> {
        let starting_exponent = (min.ilog10()) / repeats;
        let first_part_of_number = min / 10u64.pow(min.ilog10() - starting_exponent);
        
        (first_part_of_number..)
            .map(|base| {
                format!("{base}")
                    .repeat(repeats as usize)
                    .parse::<u64>()
                    .ok()
            })
            .while_some()
            .skip_while(|&invalid_id| invalid_id < min)
            .take_while(|&invalid_id| invalid_id <= max)
            .collect()
    }
    
    (min.ilog10()..=max.ilog10())
        .filter(|exponent| (exponent + 1) % repeats == 0)
        .flat_map(|exponent| {
            let subrange = (
                min.max(10u64.pow(exponent)),
                max.min(10u64.pow(exponent + 1) - 1),
            );
            find_invalid_ids_for_subrange(&subrange, repeats)
        })
        .collect()
}
```

Tests still pass, and it now executes the test data in <1ms.

