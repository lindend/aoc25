use crate::timed::{print_timespan, timed};
use crate::util::equation_system::gauss_elimination;
use crate::util::str_util::transpose;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem::swap;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;
use std::simd::{Simd, f64x2, i64x8, u32x8};
use std::time::Instant;
use std::{fs, usize};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

fn read_num(input: &[u8], i: &mut usize) -> usize {
    let mut num = 0;
    loop {
        match input[*i] {
            c if c >= b'0' && c <= b'9' => num = num * 10 + (c - b'0') as usize,
            _ => {
                return num;
            }
        }
        *i += 1;
    }
}

fn read_numbers(input: &[u8], i: &mut usize) -> Vec<usize> {
    let mut res = Vec::new();
    while *i < input.len() {
        match input[*i] {
            b']' | b')' | b'}' => {
                *i += 1;
                return res;
            }
            b',' => {
                *i += 1;
            }
            _ => {
                res.push(read_num(input, i));
            }
        };
    }
    panic!("No closing character")
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut indicators = None;
            let mut buttons = Vec::new();
            let mut joltage_req = None;

            let l = l.as_bytes();
            let mut i = 0;
            while i < l.len() {
                match l[i] {
                    b'[' => {
                        let mut v = Vec::new();
                        i += 1;
                        while l[i] != b']' {
                            v.push(match l[i] {
                                b'.' => Indicator::Off,
                                b'#' => Indicator::On,
                                _ => panic!("Weird input {}", l[i] as char),
                            });
                            i += 1;
                        }
                        indicators = Some(v);
                    }
                    b'(' => {
                        i += 1;
                        buttons.push(read_numbers(l, &mut i));
                    }
                    b'{' => {
                        i += 1;
                        joltage_req = Some(read_numbers(l, &mut i));
                    }
                    b' ' => {}
                    _ => panic!("Weird input {}", l[i] as char),
                };
                i += 1;
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

fn hash(lights: &Vec<Indicator>) -> u64 {
    let mut hasher = DefaultHasher::new();
    lights.hash(&mut hasher);
    hasher.finish()
}

fn shortest_light_input(target: &Vec<Indicator>, buttons: &Vec<Vec<usize>>) -> i64 {
    let mut heads = BinaryHeap::new();
    heads.push(State {
        cost: 0,
        lights: vec![Indicator::Off; target.len()],
    });

    let mut visited = HashSet::new();

    loop {
        let head = heads.pop().expect("No path");

        for btn in buttons {
            let mut lights = head.lights.clone();
            for &i in btn {
                lights[i].toggle();
            }

            let light_hash = hash(&lights);
            if !visited.insert(light_hash) {
                continue;
            }

            let num_wrong = lights.iter().zip(target).filter(|&(a, b)| *a != *b).count();
            if num_wrong == 0 {
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

/*
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

a = 0 0 0 1
b = 0 1 0 1
c = 0 0 1 0
d = 0 1 1 1
e = 1 0 1 0
f = 1 1 0 0

0 0 0 0 1 1 3
0 1 0 1 0 1 5
0 0 1 1 1 0 4
1 1 0 1 0 0 7


1 1 0 1 0 0 7
0 1 0 1 0 1 5
0 0 1 1 1 0 4
0 0 0 0 1 1 3

f = t
e = 3 - t
c + d + e = 4
c + d = 1 + t
c = s
d = 1 + t - s
b + d + f = 5
b = 4 - 2t + s
a + b + d = 7
a + d = 3 + 2t - s
a = 2 - t


s = 0
t = 0

a = 2
b = 4
c = 0
d = 1
e = 3
f = 0
*/

pub fn part2(input: &Input) -> i64 {
    let mut num_presses = 0;
    for (_, buttons, joltages) in input {
        let mut matrix = joltages
            .iter()
            .enumerate()
            .map(|(joltage_index, joltage)| {
                buttons
                    .iter()
                    .map(|button| {
                        if button.contains(&joltage_index) {
                            1.0
                        } else {
                            0.0
                        }
                    })
                    .chain(vec![*joltage as f64])
                    .collect()
            })
            .collect();
        let mut res = gauss_elimination(&matrix);
        println!("Res: {res:?}");

        let mut param_max: f64 = 0.0;

        for param in &res[1..] {
            for i in 0..param.len() {
                if param[i] == 0.0 {
                    continue;
                }
                for j in 0..buttons[i].len() {
                    let factor = joltages[buttons[i][j]] as f64 / param[i];
                    param_max = param_max.max(factor);
                }
            }
        }

        let param_max = param_max as i64;

        println!("pmax: {param_max}, num params: {}", res.len() - 1);

        // let mut param_maxes = vec![1.0];

        // // Find the min and max amounts for each param
        // for param in &res[1..] {
        //     let mut max: f64 = f64::MAX;

        //     for p_idx in 0..param.len() {
        //         let p = param[p_idx];
        //         if p.abs() < 0.000001 {
        //             continue;
        //         }

        //         let v = res[0][p_idx];
        //         let factor = v / p;
        //         if v > 0.0 && p < 0.0 {
        //             max = max.min(-factor)
        //         }
        //         if factor < 0.0 {
        //             println!("Factor: {p} {v} {factor}");
        //         }
        //         if max == f64::MAX {
        //             max = 0.0;
        //         }
        //     }
        //     param_maxes.push(max);
        // }

        // println!("maxes: {param_maxes:?}");

        let mut sum = res[0].clone();

        println!("Sum: {sum:?}");
        for i in 0..sum.len() {
            let s = sum[i];
            if s < 0.0 {
                println!("Less than 1 {s}");
                for param in &res[1..] {
                    if param[i] > 0.0 {
                        let factor = -s / param[i];
                        for j in 0..sum.len() {
                            sum[j] += param[j] * factor;
                        }
                        if sum.iter().all(|&s| s >= 0.0) {
                            break;
                        } else {
                            sum = res[0].clone();
                        }
                    }
                }
            }
        }

        if sum.iter().any(|&s| s < 0.0) {
            println!("{sum:?}");
            panic!("Hmm");
        }

        num_presses += sum.iter().sum::<f64>() as i64;
    }
    num_presses
}

// 19159

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
