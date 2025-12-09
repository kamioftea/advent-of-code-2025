---
day: 8
tags: [ post ]
header: 'Day 8: Playground'
---

Today feels like a step up from previous days, in terms of the steps to work through, and the puzzle writing / examples
provided felt less helpful. The task is to cluster junction boxes in 3D space, sorted by smallest connection first.

## Parse input

First some types and tests.

```rust
type JunctionBox = (i64, i64, i64);

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
```

The parsing here is trivial, split by lines and then `,`s. There isn't a handy `split_twice` function, so I need to be a
bit more verbose in extracting the three numbers on each line.

```rust
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
```

## Part one

First I need all the possible connections, sorted by shortest first. The walk through provides the first four of
that list, which can be the test.

```rust
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
```

I can use `Itertools::tuple_combinations` to produce all the possible connections. I only need to keep track of which
boxes are bing connected, I can use their indices to reference the boxes. I will also need to calculate the distance.
This comes across a quirk of Rust: floating point numbers don't implement `Ord` because they include `NaN` which
prevents a total ordering. For now, I use `isqrt` and will revisit this if it causes an actual bug - I suspect the input
does not have a key connection tied for length once rounded with another.

```rust
fn connection_distance(
    (x_a, y_a, z_a): &JunctionBox,
    (x_b, y_b, z_b): &JunctionBox
) -> i64 {
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
```

With the connections sorted I now need to start combining them. I do this by creating a lookup for which circuit a node
currently belongs to, initialised with their own index. When a connection is combined all boxes in the circuit of the
second box are given the circuit id of the first. Once combined. the counts of each id will be the circuit sizes.

I split combining the three largest into a wrapper function, but the hard work is done by that point.

```rust
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

fn circuits_after_n_connections(
    junction_boxes: &Vec<JunctionBox>,
    target_connections: u32,
) -> Vec<usize> {
    let mut circuits: Vec<usize> = junction_boxes
        .iter()
        .enumerate()
        .map(|(idx, _)| idx)
        .collect();
    
    for &(a, b) in
        order_possible_connections(junction_boxes).iter().take(target_connections)
    {
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
```

## Part two

This is very similar, but the task is to keep going until the boxes are fully combined into a single circuit. I drop
the `take(n)` connections, and instead add a check to see if there is only one circuit id present. Converting to the
puzzle solution is also different.

```rust
#[test]
fn can_find_connection_that_merges_circuit() {
    assert_eq!(find_final_connection(&sample_junction_boxes()), (10, 12));
    assert_eq!(
        find_x_product_of_final_connection(&sample_junction_boxes()),
        25272
    );
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
```

## Optimisation

I feel I'm missing a quicker algorithm, but I do see one optimisation, which is that the shortest connections are
being built and sorted twice, and that is the bottleneck. I first have the list of connections passed around as an
extra parameter. This works and roughly halves the runtime, but is clunky, and wouldn't be intuitive as an API. So
I combine the boxes and connections into a struct, doing the sorting as part of the parsing.

```rust
type Connection = (usize, usize);

struct DecorationProject {
    boxes: Vec<JunctionBox>,
    connections: Vec<Connection>,
}
```

The functions are then refactored to pass that around in place of the list of junction boxes.
