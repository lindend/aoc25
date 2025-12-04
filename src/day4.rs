use std::cmp::PartialEq;
use std::fs;
use crate::timed::timed;
use crate::util::grid::Grid;
use crate::util::vec2::Vec2;

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    PaperRoll
}

fn parse_input(input: &str) -> Grid<Cell> {
    Grid::from_str(input, |c| match c {
        '@' => Cell::PaperRoll,
        _ => Cell::Empty
    })
}


pub fn get_accessible(grid: &Grid<Cell>) -> Vec<Vec2<i64>> {
    let neighbours = vec![
        Vec2::new(-1, 0),
        Vec2::new(1, 0),
        Vec2::new(-1, 1),
        Vec2::new(0, 1),
        Vec2::new(1, 1),
        Vec2::new(-1, -1),
        Vec2::new(0, -1),
        Vec2::new(1, -1)
    ];

    grid.iter()
        .filter(|(pos, value)|
            **value == Cell::PaperRoll &&
            neighbours.iter()
                .filter(|n| grid.at(n.x + pos.x, n.y + pos.y).unwrap_or(Cell::Empty) == Cell::PaperRoll)
                .count() < 4
        )
        .map(|(pos, _)| pos)
        .collect()
}

pub fn part1(grid: &Grid<Cell>) -> i64 {
    get_accessible(&grid).len() as i64
}

// 9ms 616us 94ns
pub fn part2(grid: &Grid<Cell>) -> i64 {
    let grid = &mut grid.clone();
    let mut num_removed = 0;
    loop {
        let accessible = get_accessible(&grid);
        
        if accessible.is_empty() {
            break
        }
        
        for a in accessible {
            grid.update(a.x, a.y, Cell::Empty);
            num_removed += 1;
        }
    }
    
    num_removed
}


pub fn day4() {
    let input = fs::read_to_string("inputs/day4.txt").expect("Could not read input");

    let inputs = parse_input(&input);

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
}
