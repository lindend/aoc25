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
use std::{fs, i64, usize};

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

type Buttons = Vec<Vec<usize>>;
type Joltages = Vec<usize>;

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

fn is_p2_solution(buttons: &Buttons, joltages: &Joltages, presses: &Vec<f64>) -> bool {
    let mut result_joltages: Joltages = vec![0; joltages.len()];

    for (button_idx, button) in buttons.iter().enumerate() {
        for &joltage_idx in button {
            result_joltages[joltage_idx] += presses[button_idx].round() as usize;
        }
    }

    joltages
        .iter()
        .copied()
        .zip(result_joltages)
        .all(|(j0, j1)| j0 == j1)
}

fn part2_line(buttons: &Buttons, joltages: &Joltages) -> i64 {
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

    let mut param_max = vec![i64::MAX; res.len() - 1];
    for (param_idx, param) in res[1..].iter().cloned().enumerate() {
        for i in 0..param.len() {
            if param[i] == 0.0 {
                continue;
            }
            let param_factor = (res[0][i] / param[i]).abs();
            let mut max_value = 0;
            for &joltage_idx in &buttons[i] {
                let joltage_factor = (joltages[joltage_idx] as f64 / param[i]).abs();
                max_value = max_value.max((param_factor + joltage_factor) as i64);
            }
            param_max[param_idx] = max_value;
        }
    }

    for i in 0..res[0].len() {
        let all_zero_or_below = (0..(res.len() - 1)).all(|param_idx| res[param_idx + 1][i] <= 0.0);

        if all_zero_or_below {
            for param_idx in 0..res.len() - 1 {
                let param_value = res[param_idx + 1][i];
                if param_value == 0.0 {
                    continue;
                }
                param_max[param_idx] =
                    param_max[param_idx].min((res[0][i] / -param_value).abs() as i64);
            }
        }
    }

    println!("pmax: {param_max:?}, num params: {}", res.len() - 1);

    if res.len() == 1 {
        let num_presses = res[0].iter().map(|v| v.round()).sum::<f64>().round() as i64;
        assert_ne!(num_presses, 0);
        assert!(is_p2_solution(buttons, joltages, &res[0]));
        println!("min num presses: {num_presses}");
        println!("min v: {:?}", res[0]);
        return num_presses;
    }

    let mut param_values = vec![0i64; res.len() - 1];
    let mut min_num_presses = i64::MAX;
    let mut min_v = Vec::new();
    'main_loop: loop {
        let mut v = res[0].clone();
        'param_loop: for param_index in 0..param_values.len() {
            for i in 0..v.len() {
                let factor = param_values[param_index] as f64;
                v[i] += factor * res[1 + param_index][i];
            }
        }

        if v.iter()
            .all(|&v| v >= -1e-5 && (v - v.round()).abs() < 1e-5)
        {
            let num_presses = v.iter().map(|v| v.round()).sum::<f64>().round() as i64;
            if num_presses < min_num_presses {
                min_v = v;
            }
            min_num_presses = min_num_presses.min(num_presses);
        }

        // Step to the next param value
        for i in 0..param_values.len() {
            param_values[i] += 1;
            if param_values[i] > param_max[i] {
                if i == param_values.len() - 1 {
                    println!("min num presses: {min_num_presses}");
                    println!("min v: {min_v:?}");
                    assert!(is_p2_solution(buttons, joltages, &min_v));
                    assert!(min_num_presses > 0);
                    assert!(min_num_presses < 1000000);
                    return min_num_presses;
                }

                param_values[i] = 0;
            } else {
                break;
            }
        }
        if param_values.len() == 0 {
            panic!("Should not be here");
        }
    }
}

pub fn part2(input: &Input) -> i64 {
    let num_presses: Vec<_> = input
        .iter()
        .map(|(_, buttons, joltages)| part2_line(buttons, joltages))
        .collect();
    println!("num_presses: {num_presses:?}");
    num_presses.iter().sum()
}

// Unlikely:
// 19159

// Attempted
// 19292 - too low

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

    #[test]
    fn test_p2_2() {
        let input = parse_input(&"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(12, part2(&input));
    }

    #[test]
    fn test_p2_3() {
        let input = parse_input(
            &"[..##..] (0,5) (1,2,3,4,5) (1,3,4,5) (3,4) (2,3,5) (0,1,2,5) {29,40,23,42,39,52}",
        );
        assert_eq!(part2(&input), 71);
    }

    #[test]
    fn test_p2_4() {
        let input = parse_input(
            &"[..#...#.#] (2,7) (1,4,7) (0,1,3,4,5,6,8) (2,3,4,5,6,7,8) (1,4,6,7) (0,2,4,5,6,7,8) (0,5,7) (0,1,3,5,6,7,8) (0,4,6) (0,1,2,5,6,7,8) (0,1,2,3,5) {237,230,49,207,213,228,221,72,200}",
        );
        assert_eq!(part2(&input), 267);
    }

    #[test]
    fn test_p2_5() {
        let input = parse_input(&"[####] (1,3) (2,3) (3) (0,1,2) (0,2) (0,1) {21,23,21,27}");
        assert_eq!(part2(&input), 48);
    }
}

// Weird:
// [[1.0, 1.0, 1.0, 0.0, 220.0], [0.0, 1.0, 1.0, 0.0, 207.0], [1.0, 0.0, 0.0, 0.0, 13.0], [1.0, 0.0, 1.0, 1.0, 225.0], [0.0, 1.0, 1.0, 0.0, 207.0], [1.0, 1.0, 0.0, 1.0, 48.0]]
