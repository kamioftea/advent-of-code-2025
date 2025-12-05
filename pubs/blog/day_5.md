---
day: 5
tags: [ post ]
header: 'Day 5: Cafeteria'
---

Today I have to work out which (large) ids are in which (large) id ranges. I feel this is something previous
years' advent of code puzzles have primed me to think about differently.

## Parse input

I add a type for the ranges, and a test from the sample input. My plan for solving today relies on the ids and
ranges being sorted. The actual puzzle input isn't pre-sorted in the way most of the sample is, so I change the order a
bit to simulate that. I also arranged the expected data in sorted order.

```rust
type IdRange = (u64, u64);

fn sample_data() -> (Vec<IdRange>, Vec<u64>) {
    (
        vec![(3, 5), (10, 14), (12, 18), (16, 20)],
        vec![1, 5, 8, 11, 17, 32],
    )
}

#[test]
fn can_parse_input() {
    let input = "\
3-5
10-14
16-20
12-18

1
8
17
11
5
32
"
        .to_string();
    
    assert_eq!(parse_input(&input), sample_data());
}
```

## Part one

Once in sort order, I can do a single pass through the ids and ranges in step. If the id is before the current range,
I know it is not included in any of them because all others start later. Once I find an id in the range, I can
consume ids until they exceed the upper bound, counting each as fresh. Once outside, I need to check the same id
against the next range with the same id incase they overlap, or are adjacent. Once I run out of ids or ranges, no
further ids can be in a range, so I can stop counting.

```rust
#[test]
fn can_count_fresh_ids() {
    let (ranges, ids) = sample_data();
    assert_eq!(count_fresh_ids(&ranges, &ids), 3);
}

fn count_fresh_ids(ranges: &Vec<IdRange>, ids: &Vec<u64>) -> u64 {
    let mut fresh_count = 0;
    let mut range_index = 0;
    let mut id_index = 0;
    
    loop {
        if range_index == ranges.len() || id_index == ids.len() {
            break;
        }
        
        let (min, max) = ranges[range_index];
        let id = ids[id_index];
        
        if id <= max {
            id_index += 1;
            if id >= min {
                fresh_count += 1;
            }
        } else {
            range_index += 1;
        }
    }
    
    fresh_count
}
```

## Part two

For part two I have to count all possible ids that are in at least one fresh range. If anything this is simpler than
the first part, as I only need to consider the list of ranges. I can keep track of the maximum id seen in a range, and
only add the subset of new ranges that are higher than that. Due to the range being inclusive there's an awkward
off-by-one error to compensate for.

```rust
#[test]
fn can_count_possible_fresh_ids() {
    let (ranges, _) = sample_data();
    assert_eq!(count_possible_fresh_ids(&ranges), 14);
}

fn count_possible_fresh_ids(ranges: &Vec<IdRange>) -> u64 {
    let mut total_ids = 0;
    let mut id_threshold = 0;
    
    for &(min, max) in ranges {
        let lower_bound = min.max(id_threshold);
        if lower_bound <= max {
            total_ids += max - lower_bound + 1;
        }
        id_threshold = id_threshold.max(max + 1)
    }
    
    total_ids
}
```
