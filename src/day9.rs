use crate::timed::{print_timespan, timed};
use crate::util::spatial_grid::SpatialGrid;
use crate::util::str_util::transpose;
use crate::util::vec2::Vec2;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::mem::swap;
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

    let (ax, ay) = shrink_outline(&xs, &ys, input.len());

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
                let area = area[k];
                if area > max_area {
                    let i = i + k;
                    let j = j + k;

                    area_buffer.push((i, j, area));
                }
            }
        }
        if area_buffer.len() > 1000 || i >= input.len() - 8 {
            area_buffer.sort_by_key(|&(_, _, area)| Reverse(area));
            'area_loop: for &(i, j, area) in &area_buffer {
                if area <= max_area {
                    break;
                }

                let min = [xs[i].min(xs[j]) + 1, ys[i].min(ys[j]) + 1];
                let max = [xs[i].max(xs[j]) - 1, ys[i].max(ys[j]) - 1];

                // let sa = surrounded_area(input.len(), &ax, &ay, i, j);
                // if area == sa {
                //     max_area = area;
                // }

                let end = Vec2::new(xs[j], ys[j]);

                for r in i..j {
                    let xv = xs[r];
                    let yv = ys[r];
                    let dx = xs[(r + 1) % num_inputs] - xv;
                    let dy = ys[(r + 1) % num_inputs] - yv;

                    // Rotate delta vector 90 deg to left to form the normal
                    let normal = Vec2::new(-dy, dx);
                    let to_end = end - Vec2::new(xv, yv);

                    if normal.dot(&to_end) < 0 {
                        continue 'area_loop;
                    }
                }

                // if !point_grid.bbox_contains_point(&min, &max) {
                max_area = area;
                // }
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

// 1410470448 too low
// 222529760 too low
// 2859243744 too high

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
