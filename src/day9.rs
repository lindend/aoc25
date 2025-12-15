use crate::timed::{print_timespan, timed};
use crate::util::str_util::transpose;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::mem::swap;
use std::rc::Rc;
use std::simd::cmp::SimdOrd;
use std::simd::num::SimdInt;
use std::simd::{Simd, i64x8, u32x8};
use std::time::Instant;
use crate::util::spatial_grid::SpatialGrid;

#[derive(Copy, Clone)]
struct Node {
    id: usize,
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> Vec<Node> {
    input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let mut parts = l.splitn(2, ",").map(|p| p.parse::<i64>().unwrap());
            Node {
                id,
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
            }
        })
        .collect()
}

fn area(n1x: i64x8, n1y: i64x8, n2x: i64x8, n2y: i64x8) -> i64x8 {
    let ones = Simd::splat(1);
    let dx = n1x - n2x + ones;
    let dy = n1y - n2y + ones;

    let signed_areas = dx * dy;
    signed_areas.abs()
}

pub fn part1(input: &Vec<Node>) -> i64 {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for i in input {
        xs.push(i.x);
        ys.push(i.y);
    }

    for i in 0..input.len() % 8 + 8 {
        xs.push(0);
        ys.push(0);
    }
    let num_inputs = input.len();

    let mut max_area = Simd::splat(0i64);

    let area_masks: Vec<i64x8> = (0..9)
        .map(|m| {
            let area_mask: Vec<_> = (0..8).map(|n| if n >= m { 1 } else { 0 }).rev().collect();
            Simd::from_slice(&area_mask)
        })
        .collect();

    for i in (0..input.len() - 1).step_by(8) {
        let n1x = Simd::from_slice(&xs[i..i + 8]);
        let n1y = Simd::from_slice(&ys[i..i + 8]);

        for j in i + 1..input.len() {
            let n2x = Simd::from_slice(&xs[j..j + 8]);
            let n2y = Simd::from_slice(&ys[j..j + 8]);

            let mut areas = area(n1x, n1y, n2x, n2y);

            if j + 8 >= num_inputs {
                let area_mask = area_masks[j + 8 - num_inputs];
                areas = areas * area_mask;
            }

            max_area = max_area.simd_max(areas);
        }
    }

    max_area.reduce_max()
}

fn clamp_plus_one(v: i64, min: i64, max: i64) -> i64 {
    if v <= min {
        min
    } else if v >= max {
        max + 1
    } else {
        v + 1
    }
}

fn surrounded_area(len: usize, xs: &Vec<i64>, ys: &Vec<i64>, from: usize, to: usize) -> i64 {
    let min_x = xs[from].min(xs[to]);
    let max_x = xs[from].max(xs[to]);
    let min_y = ys[from].min(ys[to]);
    let max_y = ys[from].max(ys[to]);

    let mut area = 0;

    for i in 0..len {
        let j = (i + 1) % len;
        let x = xs[i].clamp(min_x, max_x);
        let y = ys[i].clamp(min_y, max_y);
        let y2 = ys[j].clamp(min_y, max_y);
        let dy = y2 - y;

        area += x * dy;
    }
    area
}

struct NodePair {
    id0: usize,
    id1: usize,
    area: i64,
}

impl Ord for NodePair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.area > other.area {
            Ordering::Less
        } else if (self.area < other.area) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for NodePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodePair {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for NodePair {}

pub fn part2(input: &Vec<Node>) -> i64 {
    let start = Instant::now();

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    
    let mut min = [i64::MAX; 2];
    let mut max = [0; 2];
    
    for i in input {
        min[0] = min[0].min(i.x);
        min[1] = min[1].min(i.y);
        
        max[0] = max[0].max(i.x);
        max[1] = max[1].max(i.y);
    }
    
    let mut point_grid = SpatialGrid::<3, 5>::new(&min, &max);
    
    for i in input {
        xs.push(i.x);
        ys.push(i.y);
    }

    for i in 0..input.len() % 8 + 8 {
        xs.push(0);
        ys.push(0);
    }
    let num_inputs = input.len();

    let area_masks: Vec<i64x8> = (0..9)
        .map(|m| {
            let area_mask: Vec<_> = (0..8).map(|n| if n >= m { 1 } else { 0 }).rev().collect();
            Simd::from_slice(&area_mask)
        })
        .collect();
    let mut max_area = 0;

    for i in (0..input.len() - 1).step_by(8) {
        let n1x = Simd::from_slice(&xs[i..i + 8]);
        let n1y = Simd::from_slice(&ys[i..i + 8]);

        for j in i + 1..input.len() {
            let n2x = Simd::from_slice(&xs[j..j + 8]);
            let n2y = Simd::from_slice(&ys[j..j + 8]);

            let mut areas = area(n1x, n1y, n2x, n2y);

            if j + 8 >= num_inputs {
                let area_mask = area_masks[j + 8 - num_inputs];
                areas = areas * area_mask;
            }
            
            let area = areas.to_array();
            for i in 0..area.len() {
                let area = area[i];
                if area > max_area {
                    let min = 1;
                }
            }
        }
    }

    let mut x_mod = 0;
    let mut y_mod = 0;
    for i in 0..input.len() {
        let j = (i + 1) % input.len();
        let dx = xs[j] - xs[i];
        let dy = ys[j] - ys[i];

        if dx > 0 {
            y_mod = 0;
        } else if dx < 0 {
            y_mod = 1;
        }
        if dy > 0 {
            x_mod = 1;
        } else if dy < 0 {
            x_mod = 0;
        }

        xs[i] += x_mod;
        ys[i] += y_mod;
    }

    let found_pairs = Instant::now();

    let sorted_areas = top_areas.into_sorted_vec();

    let mut max_area = 0;
    let mut num_loops = 0;

    for node_pair in sorted_areas {
        if node_pair.area <= max_area {
            break;
        }
        num_loops += 1;

        let actual_area = surrounded_area(input.len(), &xs, &ys, node_pair.id0, node_pair.id1);
        if actual_area < node_pair.area {
            continue;
        }
        
        max_area = max_area.max(actual_area);
    }

    let refine_areas = Instant::now();
    println!("num loops: {num_loops}");

    print_timespan("Find pairs", found_pairs - start);
    print_timespan("Refine area", refine_areas - found_pairs);

    max_area
}

pub fn day9() {
    let input = fs::read_to_string("inputs/day9.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(50, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(24, part2(&input));
    }
}
