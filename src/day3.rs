use std::fs;
use crate::timed::timed;

fn parse_input(input: &str) -> Vec<&str> {
    input
        .lines()
        .collect()
}

fn max_joltage(bank: &str, num_cells: usize) -> i64 {
    let bank_bytes = bank.as_bytes();
    let mut joltage = 0i64;
    let mut start = 0;
    for i in 0..num_cells {
        let mut max_cell = 0;
        let mut cell_idx = 0;
        for cell in start..(bank_bytes.len() - num_cells + i + 1) {
            let cell_value = bank_bytes[cell] - '0' as u8;
            if cell_value > max_cell {
                max_cell = cell_value;
                cell_idx = cell;
            }
        }
        start = cell_idx + 1;
        joltage *= 10;
        joltage += max_cell as i64;
    }

    joltage
}

pub fn part1(banks: &Vec<&str>) -> i64 {
    banks.iter()
        .map(|&b| max_joltage(b, 2))
        .sum()
}

pub fn part2(banks: &Vec<&str>) -> i64 {
    banks.iter()
        .map(|&b| max_joltage(b, 12))
        .sum()
}


pub fn day3() {
    let input = fs::read_to_string("inputs/day3.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_p1() {
        let ranges = parse_input(&TEST_INPUT);
        assert_eq!(357, part1(&ranges));
    }

    #[test]
    fn test_p2() {
        let ranges = parse_input(&TEST_INPUT);
        assert_eq!(3121910778619, part2(&ranges));
    }
}
