use crate::timed::timed;
use crate::util::grid::Grid;
use crate::util::vec2::Vec2;
use std::cmp::PartialEq;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    PaperRoll,
}

fn parse_input(input: &str) -> Grid<Cell> {
    Grid::from_str(input, |c| match c {
        '@' => Cell::PaperRoll,
        _ => Cell::Empty,
    })
}
fn is_accessible(grid: &Grid<Cell>, pos: Vec2<i64>, value: Cell) -> bool {
    value == Cell::PaperRoll
        && grid
            .neighbours(pos.x, pos.y)
            .filter(|(_, v)| *v == Cell::PaperRoll)
            .count()
            < 4
}
pub fn get_accessible(grid: &Grid<Cell>) -> Vec<Vec2<i64>> {
    grid.iter()
        .filter(|(pos, value)| is_accessible(&grid, *pos, *value.clone()))
        .map(|(pos, _)| pos)
        .collect()
}

pub fn part1(grid: &Grid<Cell>) -> i64 {
    get_accessible(&grid).len() as i64
}

pub fn part2(grid: &Grid<Cell>) -> i64 {
    let grid = &mut grid.clone();
    let mut num_removed = 0;
    let mut accessible = get_accessible(&grid);
    while !accessible.is_empty() {
        let mut next_accessible = Vec::new();
        for a in accessible {
            if grid.at(a.x, a.y).unwrap_or(Cell::Empty) != Cell::PaperRoll
                || !is_accessible(&grid, a, Cell::PaperRoll)
            {
                continue;
            }

            grid.update(a.x, a.y, Cell::Empty);
            next_accessible.extend(grid.neighbours(a.x, a.y).filter_map(
                |(pos, value)| match value {
                    Cell::PaperRoll => Some(pos + a),
                    _ => None,
                },
            ));
            num_removed += 1;
        }
        accessible = next_accessible;
    }

    num_removed
}

pub fn day4() {
    let input = fs::read_to_string("inputs/day4.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_p1() {
        let grid = parse_input(&TEST_INPUT);
        assert_eq!(13, part1(&grid));
    }
    #[test]
    fn test_p2() {
        let grid = parse_input(&TEST_INPUT);
        assert_eq!(43, part2(&grid));
    }
    #[test]
    fn test_p2_2() {
        let grid = parse_input(&TEST_INPUT);
        assert_eq!(43, part2(&grid));
    }

    #[test]
    fn test_p2_real() {
        let input = fs::read_to_string("inputs/day4.txt").expect("Could not read input");
        assert_eq!(8948, part2(&parse_input(&input)));
    }
}
