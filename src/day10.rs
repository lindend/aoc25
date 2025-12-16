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
    pub fn toggle(&self) -> Self {
        match self {
            Indicator::Off => Indicator::On,
            Indicator::On => Indicator::Off,
        }
    }
}

type Input = Vec<(Vec<Indicator>, Vec<Vec<usize>>, Vec<usize>)>;

fn parse_input(input: &str) -> Input {
    let mut indicators = Vec::new();
    let mut buttons = Vec::new();
    let mut joltage_req = Vec::new();

    input.lines().map(|l| {
        let mut parts = l.split(" ");

        for p in parts {}
    });

    vec![(indicators, buttons, joltage_req)]
}

pub fn part1(input: &Input) -> i64 {
    0
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
        assert_eq!(25272, part2(&input));
    }
}
