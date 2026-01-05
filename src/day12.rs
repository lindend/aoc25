use std::{collections::VecDeque, fs, str::Lines};

use rustc_hash::FxHashMap;

use crate::{timed::timed, util::grid::Grid};

#[derive(Clone, Copy, PartialEq, Eq)]
enum GridEntry {
    Free,
    Occupied,
}

impl std::fmt::Debug for GridEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Free => write!(f, "."),
            Self::Occupied => write!(f, "#"),
        }
    }
}

struct Shape {
    grid: Grid<GridEntry>,
}

struct Region {
    width: usize,
    height: usize,
    num_shapes: Vec<usize>,
}

type Input<'a> = (Vec<Shape>, Vec<Region>);

fn read_shape(lines: &mut Lines<'_>) -> Shape {
    let mut grid_str = Vec::new();

    loop {
        if let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }
            grid_str.push(l);
        } else {
            break;
        }
    }

    let grid = Grid::from_lines(grid_str.iter().copied(), |s| match s {
        '#' => GridEntry::Occupied,
        '.' => GridEntry::Free,
        _ => panic!("Unrecognized input"),
    });

    Shape { grid }
}

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
        if *i == input.len() {
            return num;
        }
    }
}

fn parse_region(line: &str) -> Region {
    let b = line.as_bytes();

    let mut i = 0;
    let width = read_num(b, &mut i);
    i += 1;
    let height = read_num(b, &mut i);
    i += 2;

    let mut num_shapes = Vec::new();

    while i < b.len() {
        num_shapes.push(read_num(b, &mut i));
        i += 1;
    }

    Region {
        height,
        width,
        num_shapes,
    }
}

fn parse_input(input: &str) -> Input {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.ends_with(":") {
            shapes.push(read_shape(&mut lines));
        } else {
            regions.push(parse_region(line));
            // while let Some(line) = lines.next() {
            //     regions.push(parse_region(line));
            // }
        }
    }

    (shapes, regions)
}

pub fn part1(input: &Input) -> i64 {
    let mut num_big_enough = 0;
    let mut num_possible = 0;
    let shape_areas: Vec<usize> = input
        .0
        .iter()
        .map(|s| {
            s.grid
                .iter()
                .filter(|(_, f)| **f == GridEntry::Occupied)
                .count()
        })
        .collect();

    for (i, region) in input.1.iter().enumerate() {
        let num_shapes: usize = region.num_shapes.iter().copied().sum();
        let shape_area: usize = region
            .num_shapes
            .iter()
            .copied()
            .enumerate()
            .map(|(i, num)| shape_areas[i] * num)
            .sum();

        let shape_rect_area = num_shapes * 9;
        let region_area = region.width * region.height;

        let fit_in_area = shape_area < region_area;
        let fit_in_rect_area = shape_rect_area <= region_area;

        if fit_in_rect_area {
            num_big_enough += 1;
        }
        if fit_in_area {
            num_possible += 1;
        }
    }
    num_big_enough
}

pub fn part2(input: &Input) -> i64 {
    0
}

pub fn day12() {
    let input = fs::read_to_string("inputs/day12.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT_P1);
        assert_eq!(3, part1(&input));
    }

    /*
    ....AAAFFE.E
    .BBBAAFFFEEE
    DDDBAAFFCECE
    DBBB....CCC.
    DDD.....C.C.


    1849


         */
}
