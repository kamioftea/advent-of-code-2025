---
day: 4
tags: [ post ]
header: 'Day 4: Printing Department'
---

Today is the first grid puzzle of the year. I need to represent the location of all the rolls of paper in the
printing department, and work out which have three or fewer neighbours, allowing them to be accessed by a forklift.

## Parse input

I find it easier to define a struct to encapsulate the grid functionality, so first some definitions.

```rust
#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
struct Roll {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PrintingDepartment {
    rolls: HashSet<Roll>,
}
```

Whilst in the past I have enumerated the grid, it is sufficient to test that the grid has the correct number of
rolls of paper, and that a few sample locations are confirmed to correctly have a roll or be empty. I originally
implement `has_roll_at` in the main code, but it is not used outside of testing, so I move the implementation into
the test module.

```rust
impl PrintingDepartment {
    fn has_roll_at(&self, x: usize, y: usize) -> bool {
        self.rolls.contains(&Roll { x, y })
    }
}

fn sample_input() -> String {
    "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.\n"
        .to_string()
}

#[test]
fn can_parse_input() {
    let dept = PrintingDepartment::from(&sample_input());
    
    assert_eq!(dept.rolls.len(), 71);
    
    assert!(dept.has_roll_at(2, 0));
    assert!(dept.has_roll_at(8, 9));
    assert!(dept.has_roll_at(9, 7));
    
    assert!(!dept.has_roll_at(0, 0));
    assert!(!dept.has_roll_at(9, 9));
    assert!(!dept.has_roll_at(10, 10));
}
```

I implement `From<&String>` to perform the actual parsing in a standard way.

```rust
impl From<&String> for PrintingDepartment {
    fn from(value: &String) -> Self {
        let rolls = value
            .lines()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, cell)| cell == '@')
                    .map(move |(x, _)| Roll { x, y })
            })
            .collect();
        
        PrintingDepartment { rolls }
    }
}
```

## Part one

I now need to count how many rolls are accessible to the forklift. Since this is based on neighbour count, I first need
to be able to do that.

```rust
fn sample_dept() -> PrintingDepartment {
    PrintingDepartment::from(&sample_input())
}

#[test]
fn can_count_filled_neighbours() {
    let dept = sample_dept();
    
    assert_eq!(dept.neighbour_count(0, 0), 2);
    assert_eq!(dept.neighbour_count(4, 4), 8);
}
```

The actual implementation is pretty verbose to account for those locations on the edges of the department floor, and
that the location being checked shouldn't be included as one of its neighbours.

```rust
impl PrintingDepartment {
    fn neighbour_count(&self, x: usize, y: usize) -> usize {
        (y.checked_sub(1).unwrap_or(0)..=(y + 1))
            .flat_map(|y1| {
                (x.checked_sub(1).unwrap_or(0)..=(x + 1))
                    .filter(move |&x1| x != x1 || y != y1)
                    .map(move |x1| Roll { x: x1, y: y1 })
                    .filter(|coord| self.rolls.contains(coord))
            })
            .count()
    }
}
```

That done I then need to call that for each roll's location and count those that return three or fewer neighbours.

```rust
#[test]
fn can_count_accessible_rolls() {
    assert_eq!(sample_dept().count_accessible_rolls(), 13);
}

impl PrintingDepartment {
    // ...
    fn count_accessible_rolls(&self) -> usize {
        self.rolls
            .iter()
            .filter(|roll| self.neighbour_count(roll.x, roll.y) < 4)
            .count()
    }
}
```

## Part two

Once the accessible rolls have been removed, some of the remaining rolls also become removable. I need to keep removing
rolls until the remaining rolls all have four or more neighbours (or the floor is empty), and the system becomes stable.

First I implement removing the accessible rolls. Updating the set whilst iterating over it isn't possible with
Rust's borrowing rules, so I create a copy without those rolls instead.

```rust
#[test]
fn can_remove_accessible_rolls() {
    let dept = sample_dept();
    
    let next_dept = dept.remove_accessible_rolls();
    
    assert_eq!(next_dept.rolls.len(), 58);
    assert_eq!(next_dept.count_accessible_rolls(), 12);
}

impl PrintingDepartment {
    // ...
    fn remove_accessible_rolls(&self) -> PrintingDepartment {
        let rolls = self
            .rolls
            .iter()
            .filter(|roll| self.neighbour_count(roll.x, roll.y) >= 4)
            .cloned()
            .collect();
        
        PrintingDepartment { rolls }
    }
}
```

Then repeatedly call this until stable. I first implement this with a loop...

```rust
#[test]
fn can_count_possible_removals() {
    assert_eq!(sample_dept().count_removable_rolls(), 43);
}

impl PrintingDepartment {
    // ...
    fn count_removable_rolls(&self) -> usize {
        let mut current_grid: Grid = self.clone();
        
        loop {
            let next_grid = current_grid.remove_accessible_rolls();
            if next_grid.rolls.len() == current_grid.rolls.len() {
                break;
            }
            
            current_grid = next_grid;
        }
        
        self.rolls.len() - current_grid.rolls.len()
    }
}
```

...but it's cleaner using recursion

```rust
impl PrintingDepartment {
    // ...
    fn count_removable_rolls(&self) -> usize {
        let next = self.remove_accessible_rolls();
        let removed = self.rolls.len() - next.rolls.len();
        
        if removed == 0 {
            0
        } else {
            removed + next.count_removable_rolls()
        }
    }
}
```
