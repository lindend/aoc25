use std::{collections::VecDeque, fs};

use rustc_hash::FxHashMap;

use crate::timed::timed;

type Input<'a> = (NodeList<'a>, FxHashMap<usize, Vec<usize>>);

struct NodeList<'a> {
    nodes: Vec<&'a str>,
    indices: FxHashMap<&'a str, usize>,
}

impl<'a> NodeList<'a> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            indices: FxHashMap::default(),
        }
    }

    pub fn get_or_add_index(&mut self, node: &'a str) -> usize {
        match self.indices.get(node) {
            Some(&idx) => idx,
            None => {
                let idx = self.nodes.len();
                self.nodes.push(node);
                self.indices.insert(node, idx);
                return idx;
            }
        }
    }
}

fn parse_input(input: &str) -> Input {
    let mut nodes = NodeList::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let mut src_targets = line.splitn(2, ":");
        let src = src_targets.next().unwrap();
        let src_index = nodes.get_or_add_index(src);
        let targets = src_targets.next().unwrap();
        for target in targets.split_whitespace() {
            let target_index = nodes.get_or_add_index(target);
            edges.push((src_index, target_index));
        }
    }

    let mut edges_by_node: FxHashMap<usize, Vec<usize>> = FxHashMap::default();

    for (from, to) in edges {
        edges_by_node
            .entry(from)
            .and_modify(|v| v.push(to))
            .or_insert(vec![to]);
    }

    (nodes, edges_by_node)
}

fn num_paths_between((nodes, edges_by_node): &Input, start: &str, end: &str) -> i64 {
    let mut counts = vec![0; nodes.nodes.len()];

    let mut queue = VecDeque::new();

    let you_index = *nodes.indices.get(start).expect("No start-node");
    counts[you_index] = 1;
    queue.push_back(you_index);

    let end_index = *nodes.indices.get(end).expect("No end-node");

    while let Some(node) = queue.pop_front() {
        let node_count = counts[node];
        if node_count == 0 {
            continue;
        }

        if node != end_index {
            counts[node] = 0;
        }

        if let Some(outputs) = edges_by_node.get(&node) {
            for &output in outputs {
                counts[output] += node_count;
                queue.push_back(output);
            }
        }
    }

    counts[end_index]
}

pub fn part1(input: &Input) -> i64 {
    num_paths_between(input, "you", "out")
}

pub fn part2(input: &Input) -> i64 {
    let fft_dac = num_paths_between(input, "fft", "dac");
    if fft_dac > 0 {
        let svr_fft = num_paths_between(input, "svr", "fft");
        let dac_out = num_paths_between(input, "dac", "out");
        (svr_fft * fft_dac * dac_out)
    } else {
        let dac_fft = num_paths_between(input, "dac", "fft");
        let svr_dac = num_paths_between(input, "svr", "dac");
        let fft_out = num_paths_between(input, "fft", "out");
        (svr_dac * dac_fft * fft_out)
    }
}

pub fn day11() {
    let input = fs::read_to_string("inputs/day11.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT_P1);
        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_p1_2() {
        let input = parse_input(
            &"you: a c
a: b
b: c
c: out",
        );
        assert_eq!(2, part1(&input));
    }

    const TEST_INPUT_P2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT_P2);
        assert_eq!(2, part2(&input));
    }
}
