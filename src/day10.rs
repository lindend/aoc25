use crate::timed::{print_timespan, timed};
use crate::util::str_util::transpose;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::mem::swap;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;
use std::simd::{Simd, i64x8, u32x8};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Indicator {
    On,
    Off,
}

impl Indicator {
    pub fn toggle(&mut self) {
        *self = match self {
            Indicator::Off => Indicator::On,
            Indicator::On => Indicator::Off,
        };
    }
}

type Input = Vec<(Vec<Indicator>, Vec<Vec<usize>>, Vec<usize>)>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut indicators = None;
            let mut buttons = Vec::new();
            let mut joltage_req = None;

            let mut parts = l.split(" ");

            for p in parts {
                if p.starts_with("[") {
                    // Indicators
                    indicators = Some(
                        p.chars()
                            .filter_map(|c| match c {
                                '.' => Some(Indicator::Off),
                                '#' => Some(Indicator::On),
                                _ => None,
                            })
                            .collect(),
                    );
                } else if p.starts_with("(") {
                    // Buttons
                    buttons.push(
                        p[1..p.len() - 1]
                            .split(",")
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    );
                } else if p.starts_with("{") {
                    // Joltage requirements
                    joltage_req = Some(
                        p[1..p.len() - 1]
                            .split(",")
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    );
                }
            }

            (
                indicators.expect("No indicators"),
                buttons,
                joltage_req.expect("No joltage requirement"),
            )
        })
        .collect()
}

#[derive(PartialEq, Eq)]
struct State {
    lights: Vec<Indicator>,
    cost: i64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_light_input(target: &Vec<Indicator>, buttons: &Vec<Vec<usize>>) -> i64 {
    let mut heads = BinaryHeap::new();
    heads.push(State {
        cost: 0,
        lights: vec![Indicator::Off; target.len()],
    });

    loop {
        let head = heads.pop().expect("No path");

        for btn in buttons {
            let mut lights = head.lights.clone();
            for &i in btn {
                lights[i].toggle();
            }

            if lights.iter().zip(target).all(|(a, b)| a == b) {
                return head.cost + 1;
            }

            heads.push(State {
                cost: head.cost + 1,
                lights,
            })
        }
    }
}

pub fn part1(input: &Input) -> i64 {
    input
        .iter()
        .map(|(target, buttons, _)| shortest_light_input(target, buttons))
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    0
}

pub fn day10() {
    let input = fs::read_to_string("inputs/day10.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(7, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(33, part2(&input));
    }
}
