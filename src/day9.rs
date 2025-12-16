use crate::timed::{print_timespan, timed};
use crate::util::spatial_grid::SpatialGrid;
use crate::util::str_util::transpose;
use crate::util::vec2::Vec2;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::mem::swap;
use std::net::AddrParseError;
use std::rc::Rc;
use std::simd::cmp::SimdOrd;
use std::simd::num::SimdInt;
use std::simd::{Simd, i64x8, u32x8};
use std::time::Instant;

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
    let dx = (n1x - n2x).abs() + ones;
    let dy = (n1y - n2y).abs() + ones;

    dx * dy
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

fn shrink_outline(xs: &Vec<i64>, ys: &Vec<i64>, len: usize) -> (Vec<i64>, Vec<i64>) {
    let mut x_mod = 0;
    let mut y_mod = 0;

    let mut xs = xs.clone();
    let mut ys = ys.clone();

    for i in 0..len {
        let j = (i + 1) % len;
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
    (xs, ys)
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
    area.abs()
}

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

    let mut point_grid = SpatialGrid::<2, 4>::new(&min, &max);

    for i in input {
        point_grid.add_point(i.id, &[i.x, i.y]);
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
    let mut area_buffer = Vec::new();

    for i in (0..input.len() - 1).step_by(8) {
        let n1x = Simd::from_slice(&xs[i..i + 8]);
        let n1y = Simd::from_slice(&ys[i..i + 8]);

        for j in i + 1..input.len() {
            let n2x = Simd::from_slice(&xs[j..j + 8]);
            let n2y = Simd::from_slice(&ys[j..j + 8]);

            let mut areas = area(n1x, n1y, n2x, n2y);

            if j + 8 > num_inputs {
                let area_mask = area_masks[j + 8 - num_inputs];
                areas = areas * area_mask;
            }

            let area = areas.to_array();
            for k in 0..area.len() {
                let a = area[k];
                if a > max_area {
                    area_buffer.push((i + k, j + k, a));
                }
            }
        }
        if area_buffer.len() > 100 || i >= input.len() - 8 {
            area_buffer.sort_by_key(|&(_, _, area)| Reverse(area));
            'area_loop: for &(i, j, area) in &area_buffer {
                if area <= max_area {
                    break;
                }

                let min = Vec2::new(xs[i].min(xs[j]), ys[i].min(ys[j]));
                let max = Vec2::new(xs[i].max(xs[j]), ys[i].max(ys[j]));

                for r in 0..num_inputs {
                    let v = Vec2::new(xs[r], ys[r]);

                    let nr = (r + 1) % num_inputs;
                    let nv = Vec2::new(xs[nr], ys[nr]);

                    if v.in_bounds(min + Vec2::one(), max - Vec2::one()) {
                        continue 'area_loop;
                    }

                    let inside_y =
                        (v.y >= min.y && v.y <= max.y) || (nv.y >= min.y && nv.y <= max.y);
                    let inside_x =
                        (v.x >= min.x && v.x <= max.x) || (nv.x >= min.x && nv.x <= max.x);

                    if (v.x > max.x && nv.x < max.x && inside_y)
                        || (v.x > min.x && nv.x < min.x && inside_y)
                        || (v.y > max.y && nv.y < max.y && inside_x)
                        || (v.y > min.y && nv.y < min.y && inside_x)
                    {
                        continue 'area_loop;
                    }
                }
                max_area = area;
            }
            area_buffer.clear();
        }
    }

    max_area
}

pub fn day9() {
    let input = fs::read_to_string("inputs/day9.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

// 222529760 too low
// 1410470448 too low
// 1410501884
// 1478326656 wrong
// 2859243744 too high
//

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
