---
day: 7
tags: [ post ]
header: 'Day 7: Laboratories'
---

Today I have to parse an almost Christmas tree shaped "Tachyon Manifold", to count how many times a branch of tachyon
beam hits a splitter and is further split in two.

## Parse input

First I need to parse another 2D grid. I need to keep track of its height, possibly width, and the beam source as well
as the location of all the splitters. So I'll bundle all of that together in a struct, and add some tests.

```rust
type Coordinate = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct TachyonManifold {
    source: Coordinate,
    splitters: HashSet<Coordinate>,
    width: usize,
    height: usize,
}

fn sample_input() -> String {
    "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        .to_string()
}

#[test]
fn can_parse_input() {
    let manifold = TachyonManifold::from(&sample_input());
    
    assert_eq!(manifold.source, (7, 0));
    
    assert_eq!(manifold.splitters.len(), 22);
    assert!(manifold.splitters.contains(&(7, 2)));
    assert!(manifold.splitters.contains(&(6, 4)));
    assert!(manifold.splitters.contains(&(8, 4)));
    
    assert_eq!(manifold.width, 15);
    assert_eq!(manifold.height, 16);
}
```

Parsing is mostly standard with nested loops for reading the rows then characters of the grid, enumerating them to
build the co-ordinates.

```rust
impl From<&String> for TachyonManifold {
    fn from(input: &String) -> TachyonManifold {
        let mut start = None;
        let mut splitters = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => start = Some((x, y)),
                    '^' => {
                        splitters.insert((x, y));
                    }
                    _ => {}
                }
            }
            width = line.len();
            height += 1;
        }
        
        TachyonManifold {
            source: start.expect("Thin input should include a start position"),
            splitters,
            width,
            height,
        }
    }
}
```

## Part one

I then need to count the splits

```rust
#[test]
fn can_split_beams() {
    let manifold = TachyonManifold::from(&sample_input());
    
    assert_eq!(manifold.count_splits(), 21);
}
```

I can keep track of the beams with a set of the columns containing a beam. A splitter turns of the current position, but
enables those each side. The set will implicitly ignore any duplicates, which handles the merging of the beams for me.

```rust
impl TachyonManifold {
    fn count_splits(&self) -> usize {
        let mut splits = 0;
        let (initial_beam, start_row) = self.source;
        let mut beams: HashSet<usize> = vec![initial_beam].into_iter().collect();
        
        for y in start_row..self.height {
            for x in beams.clone() {
                if self.splitters.contains(&(x, y)) {
                    beams.remove(&x);
                    beams.insert(x - 1);
                    beams.insert(x + 1);
                    splits += 1
                }
            }
        }
        
        splits
    }
}
```

## Part two

Part two is a long-winded way of saying find all the routes a beam can take. I can track this in a similar way, but
I also need to track the number of routes a beam could take to get to a specific splitter. This means that when two
beams merge I need to track that is now two beams. I can keep a vector with a number of beams in each column. When a
beam splits, I set that column back to 0 add it's current count to both sides. Where this merges two beams the count
from both sides will then be added. Once done I can sum all beams to get the count of possible routes.

```rust
#[test]
fn can_count_possible_paths() {
    let manifold = TachyonManifold::from(&sample_input());
    
    assert_eq!(manifold.count_paths(), 40);
}

impl TachyonManifold {
    /// ...
    fn count_paths(&self) -> usize {
        let (initial_beam, start_row) = self.source;
        let mut beams: Vec<usize> = vec![0; self.width + 1].into_iter().collect();
        beams[initial_beam] = 1;
        
        for y in start_row..self.height {
            for (x, paths) in beams.clone().into_iter().enumerate() {
                if self.splitters.contains(&(x, y)) {
                    beams[x] = 0;
                    beams[x - 1] += paths;
                    beams[x + 1] += paths;
                }
            }
        }
        
        beams.iter().sum()
    }
}
```
