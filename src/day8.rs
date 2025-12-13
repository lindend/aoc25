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

#[derive(Copy, Clone)]
struct Node {
    id: usize,
    x: i64,
    y: i64,
    z: i64,
}

fn parse_input(input: &str) -> Vec<Node> {
    input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let mut parts = l.splitn(3, ",").map(|p| p.parse::<i64>().unwrap());
            Node {
                id,
                x: parts.next().unwrap(),
                y: parts.next().unwrap(),
                z: parts.next().unwrap(),
            }
        })
        .collect()
}

struct NodePair {
    id0: usize,
    id1: usize,
    distance: i64,
}

impl Ord for NodePair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
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

fn distance(n1x: i64x8, n1y: i64x8, n1z: i64x8, n2x: i64x8, n2y: i64x8, n2z: i64x8) -> i64x8 {
    let dx = n1x - n2x;
    let dy = n1y - n2y;
    let dz = n1z - n2z;

    dx * dx + dy * dy + dz * dz
}

fn find_pairs(input: &Vec<Node>, num_pairs: usize) -> Vec<NodePair> {
    let mut ids = Vec::new();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    for i in input {
        ids.push(i.id);
        xs.push(i.x);
        ys.push(i.y);
        zs.push(i.z);
    }
    let num_inputs = input.len();

    for i in 0..input.len() % 8 + 8 {
        xs.push(0);
        ys.push(0);
        zs.push(0);
    }

    let mut max = i64::MAX;

    let mut closest_pairs = BinaryHeap::with_capacity(num_pairs);

    for i in (0..input.len() - 1).step_by(8) {
        let n1x = Simd::from_slice(&xs[i..i + 8]);
        let n1y = Simd::from_slice(&ys[i..i + 8]);
        let n1z = Simd::from_slice(&zs[i..i + 8]);

        for j in i + 1..input.len() {
            let n2x = Simd::from_slice(&xs[j..j + 8]);
            let n2y = Simd::from_slice(&ys[j..j + 8]);
            let n2z = Simd::from_slice(&zs[j..j + 8]);

            let dists = distance(n1x, n1y, n1z, n2x, n2y, n2z);
            let dist = dists.to_array();
            closest_pairs.extend(
                dist.iter()
                    .enumerate()
                    .filter(|(r, d)| **d < max && j + r < num_inputs)
                    .map(|(r, d)| NodePair {
                        id0: ids[i + r],
                        id1: ids[j + r],
                        distance: *d,
                    }),
            );
            while closest_pairs.len() > num_pairs {
                closest_pairs.pop();
            }
            if closest_pairs.len() == num_pairs {
                max = closest_pairs.peek().unwrap().distance;
            }
        }
    }

    let mut pair_vec = closest_pairs.into_vec();
    pair_vec.sort_by_key(|v| v.distance);
    pair_vec
}

pub fn part1(input: &Vec<Node>, num_pairs: usize) -> i64 {
    let start = Instant::now();

    let mut closest_pairs = find_pairs(input, num_pairs);

    let found_pairs = Instant::now();

    let mut circuit_ids: Vec<_> = (0..input.len()).map(|c| None).collect();
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for pair in closest_pairs.iter().take(num_pairs) {
        let cid0 = circuit_ids[pair.id0];
        let cid1 = circuit_ids[pair.id1];
        if cid0 != None && cid0 == cid1 {
            continue;
        }

        // Add all to new circuit
        let (new_circuit, old_circuit) = match (cid0, cid1) {
            (None, None) => (circuits.len(), None),
            (Some(cid), None) => (cid, None),
            (None, Some(cid)) => (cid, None),
            (Some(cid), Some(old_cid)) => (cid, Some(old_cid)),
        };
        if new_circuit == circuits.len() {
            circuits.push(HashSet::new());
        }
        circuits[new_circuit].insert(pair.id0);
        circuits[new_circuit].insert(pair.id1);

        for &circ in &circuits[new_circuit] {
            circuit_ids[circ] = Some(new_circuit);
        }
        if let Some(old_circuit) = old_circuit {
            for &circ in &circuits[old_circuit] {
                circuit_ids[circ] = Some(new_circuit);
            }
            let old_entries = circuits[old_circuit].clone();
            circuits[new_circuit].extend(old_entries);
            circuits[old_circuit].clear();
        }
    }

    let form_circuits = Instant::now();

    circuits.sort_by_key(|circuit| Reverse(circuit.len()));

    let sorted_circuits = Instant::now();

    print_timespan("Find pairs", found_pairs - start);
    print_timespan("Form circuits", form_circuits - found_pairs);
    print_timespan("Sort circuits", sorted_circuits - form_circuits);

    (circuits[0].len() * circuits[1].len() * circuits[2].len()) as i64
}

pub fn part2(input: &Vec<Node>) -> i64 {
    let start = Instant::now();

    let mut closest_pairs = find_pairs(input, input.len() * input.len());

    let found_pairs = Instant::now();

    let mut circuit_ids: Vec<_> = (0..input.len()).map(|c| None).collect();
    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    for pair in closest_pairs.iter() {
        let cid0 = circuit_ids[pair.id0];
        let cid1 = circuit_ids[pair.id1];
        if cid0 != None && cid0 == cid1 {
            continue;
        }

        // Add all to new circuit
        let (new_circuit, old_circuit) = match (cid0, cid1) {
            (None, None) => (circuits.len(), None),
            (Some(cid), None) => (cid, None),
            (None, Some(cid)) => (cid, None),
            (Some(cid), Some(old_cid)) => (cid, Some(old_cid)),
        };
        if new_circuit == circuits.len() {
            circuits.push(HashSet::new());
        }
        circuits[new_circuit].insert(pair.id0);
        circuits[new_circuit].insert(pair.id1);

        for &circ in &circuits[new_circuit] {
            circuit_ids[circ] = Some(new_circuit);
        }
        if let Some(old_circuit) = old_circuit {
            for &circ in &circuits[old_circuit] {
                circuit_ids[circ] = Some(new_circuit);
            }
            let old_entries = circuits[old_circuit].clone();
            circuits[new_circuit].extend(old_entries);
            circuits[old_circuit].clear();
        }

        if circuits[new_circuit].len() == input.len() {
            let form_circuits = Instant::now();
            print_timespan("Find pairs", found_pairs - start);
            print_timespan("Form circuits", form_circuits - found_pairs);
            return input[pair.id0].x * input[pair.id1].x;
        }
    }

    panic!("wtf");
}

pub fn day8() {
    let input = fs::read_to_string("inputs/day8.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs, 1000)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
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
425,690,689";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(40, part1(&input, 10));
    }

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(25272, part2(&input));
    }
}
