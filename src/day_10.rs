//! This is my solution for [Advent of Code - Day 10: _Factory_](https://adventofcode.com/2025/day/10)
//!
//!

use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::fs;
use z3::Optimize;
use z3::SatResult::Sat;
use z3::ast::Int;

/// The entry point for running the solutions with the 'real' puzzle input.
///
/// - The puzzle input is expected to be at `<project_root>/res/day-10-input`
/// - It is expected this will be called by [`super::main()`] when the user elects to run day 10.
pub fn run() {
    let contents = fs::read_to_string("res/day-10-input.txt").expect("Failed to read file");

    let machines = parse_input(&contents);

    println!(
        "It takes {} button presses to start all the machines",
        total_initialisation_steps(&machines)
    );

    println!(
        "It takes {} button presses to configure the machines' joltages",
        total_joltage_confguration_steps(&machines)
    );
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Button {
    bitmask: u32,
}

impl Button {
    fn parse(spec: &str, lights_length: &usize) -> Self {
        let bitmask = spec
            .trim_matches(&['(', ')'])
            .split(',')
            .map(|idx| idx.parse::<usize>().unwrap())
            .fold(0, |acc, idx| acc + (1 << (lights_length - idx - 1)));

        Button { bitmask }
    }
}

impl From<u32> for Button {
    fn from(value: u32) -> Self {
        Button { bitmask: value }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct FactoryMachine {
    target_indicators: u32,
    buttons: Vec<Button>,
    jolts: Vec<u32>,
}

impl From<&str> for FactoryMachine {
    fn from(value: &str) -> Self {
        let mut parts: Vec<&str> = value.split(" ").collect();
        let target_spec = parts.remove(0);
        let jolts_spec = parts.pop().unwrap();

        fn parse_indicators(spec: &str) -> u32 {
            spec.trim_matches(&['[', ']'])
                .chars()
                .fold(0, |acc, chr| (acc << 1) + (chr == '#') as u32)
        }

        fn parse_jolts(spec: &str) -> Vec<u32> {
            spec.trim_matches(&['{', '}'])
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        }

        let light_length = target_spec.len() - 2;

        FactoryMachine {
            target_indicators: parse_indicators(target_spec),
            buttons: parts
                .into_iter()
                .map(|spec| Button::parse(spec, &light_length))
                .collect(),
            jolts: parse_jolts(jolts_spec),
        }
    }
}

fn parse_input(input: &String) -> Vec<FactoryMachine> {
    input.lines().map_into().collect()
}

fn find_shortest_startup_sequence(machine: &FactoryMachine) -> u32 {
    #[derive(Debug, Eq, PartialEq)]
    struct Step {
        indicators: u32,
        target: u32,
        proximity: u32,
        presses: u32,
    }

    impl Ord for Step {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .presses
                .cmp(&self.presses)
                .then_with(|| other.proximity.cmp(&self.proximity))
        }
    }

    impl PartialOrd for Step {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl From<&FactoryMachine> for Step {
        fn from(value: &FactoryMachine) -> Self {
            Step {
                indicators: 0,
                target: value.target_indicators,
                presses: 0,
                proximity: value.target_indicators,
            }
        }
    }

    impl Step {
        fn apply(&self, &Button { bitmask }: &Button) -> Self {
            let indicators = self.indicators ^ bitmask;
            let proximity = self.target ^ indicators;
            Step {
                indicators,
                presses: self.presses + 1,
                proximity,
                target: self.target,
            }
        }
    }

    let mut heap: BinaryHeap<Step> = BinaryHeap::new();
    heap.push(machine.into());

    while let Some(curr) = heap.pop() {
        if curr.indicators == machine.target_indicators {
            return curr.presses;
        }

        machine
            .buttons
            .iter()
            .map(|button| curr.apply(button))
            .for_each(|step| heap.push(step))
    }

    unreachable!("Heap loop won't terminate unless it finds a result")
}

fn find_shortest_joltage_configuration(machine: &FactoryMachine) -> usize {
    let optimiser = Optimize::new();
    let total_presses = Int::fresh_const("total_presses");

    let press_counts: Vec<Int> = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(idx, button)| {
            let count = Int::fresh_const(&format!("button {idx} ({})", button.bitmask));
            optimiser.assert(&count.ge(0));

            count
        })
        .collect();

    optimiser.assert(&total_presses.eq(Int::add(&press_counts)));

    for (jolt_idx, &target) in machine.jolts.iter().rev().enumerate() {
        let buttons: Vec<&Int> = machine
            .buttons
            .iter()
            .enumerate()
            .filter(|&(_, button)| (button.bitmask & 1 << jolt_idx) > 0)
            .map(|(button_idx, _)| &press_counts[button_idx])
            .collect();

        optimiser.assert(&Int::add(&buttons).eq(Int::from_u64(target as u64)));
    }

    optimiser.minimize(&total_presses);

    if optimiser.check(&[]) != Sat {
        unreachable!("No solution found")
    }

    optimiser
        .get_model()
        .unwrap()
        .eval(&total_presses, true)
        .unwrap()
        .as_i64()
        .unwrap() as usize
}

fn total_initialisation_steps(machines: &Vec<FactoryMachine>) -> u32 {
    machines.iter().map(find_shortest_startup_sequence).sum()
}

fn total_joltage_confguration_steps(machines: &Vec<FactoryMachine>) -> usize {
    machines
        .par_iter()
        .map(find_shortest_joltage_configuration)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_10::*;
    use itertools::Itertools;

    fn sample_machines() -> Vec<FactoryMachine> {
        vec![
            FactoryMachine {
                target_indicators: 6,
                buttons: vec![1, 5, 2, 3, 10, 12].into_iter().map_into().collect(),
                jolts: vec![3, 5, 4, 7],
            },
            FactoryMachine {
                target_indicators: 2,
                buttons: vec![23, 6, 17, 28, 15].into_iter().map_into().collect(),
                jolts: vec![7, 5, 12, 7, 2],
            },
            FactoryMachine {
                target_indicators: 29,
                buttons: vec![62, 38, 59, 24].into_iter().map_into().collect(),
                jolts: vec![10, 11, 11, 5, 10, 5],
            },
        ]
    }

    #[test]
    fn can_parse_machines() {
        let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        .to_string();

        assert_eq!(parse_input(&input), sample_machines());
    }

    #[test]
    fn can_initialise_machines() {
        let machines = sample_machines();

        assert_eq!(find_shortest_startup_sequence(&machines[0]), 2);
        assert_eq!(find_shortest_startup_sequence(&machines[1]), 3);
        assert_eq!(find_shortest_startup_sequence(&machines[2]), 2);

        assert_eq!(total_initialisation_steps(&machines), 7);
    }

    #[test]
    fn can_configure_joltage() {
        let machines = sample_machines();

        assert_eq!(find_shortest_joltage_configuration(&machines[0]), 10);
        assert_eq!(find_shortest_joltage_configuration(&machines[1]), 12);
        assert_eq!(find_shortest_joltage_configuration(&machines[2]), 11);

        assert_eq!(total_joltage_confguration_steps(&machines), 33);
    }
}
