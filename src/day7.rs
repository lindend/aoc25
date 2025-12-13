use crate::timed::timed;
use crate::util::str_util::transpose;
use std::fs;
use std::mem::swap;

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
enum Cell {
    Empty,
    Start,
    Beam,
    Splitter,
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    'S' => Cell::Start,
                    '^' => Cell::Splitter,
                    o => panic!("Unknown cell {o}"),
                })
                .collect()
        })
        .collect()
}

fn print_line(line: &Vec<Cell>) {
    println!(
        "{}",
        line.iter()
            .map(|c| match c {
                Cell::Empty => ".",
                Cell::Beam => "|",
                Cell::Splitter => "^",
                Cell::Start => "S",
            })
            .collect::<Vec<_>>()
            .join("")
    );
}

pub fn part1(input: &Vec<Vec<Cell>>) -> i64 {
    let mut current_line: Vec<_> = input[0]
        .iter()
        .map(|c| match c {
            Cell::Start => Cell::Beam,
            _ => Cell::Empty,
        })
        .collect();
    let mut next_line = vec![Cell::Empty; current_line.len()];

    let mut num_splits = 0;
    for line in input.iter().skip(1) {
        for j in 0..line.len() {
            if current_line[j] == Cell::Beam && line[j] == Cell::Splitter {
                next_line[j - 1] = Cell::Beam;
                next_line[j] = Cell::Empty;
                next_line[j + 1] = Cell::Beam;
                num_splits += 1;
            } else if next_line[j] != Cell::Beam {
                next_line[j] = current_line[j];
            }
        }

        swap(&mut current_line, &mut next_line);

        for i in 0..next_line.len() {
            next_line[i] = Cell::Empty;
        }
    }

    num_splits
}

pub fn part2(input: &Vec<Vec<Cell>>) -> i64 {
    let mut prev_num_paths = vec![1; input[0].len()];
    let mut current_num_paths = vec![1; input[0].len()];
    

    for line in input.iter().rev().skip(1) {
        for i in 0..line.len() {
            let cell = line[i];
            if cell == Cell::Start {
                return prev_num_paths[i];
            }
            
            current_num_paths[i] = match cell {
                Cell::Splitter => prev_num_paths[i - 1] + prev_num_paths[i + 1],
                _ => prev_num_paths[i],
            };
            swap(&mut current_num_paths, &mut prev_num_paths);
        }
    }
    panic!("No start?")
}

pub fn day7() {
    let input = fs::read_to_string("inputs/day7.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(21, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(40, part2(&input));
    }
}
