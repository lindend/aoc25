use std::fs;
use crate::timed::timed;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| {
            let mult = if l.starts_with("L") { -1 } else { 1 };
            l[1..]
                .parse::<i32>()
                .expect("could not parse input") * mult
        })
        .collect()
}

pub fn part1(inputs: &Vec<i32>, start: i32) -> i32 {
    let mut current = start;
    let mut num_zero = 0;
    
    for &i in inputs {
        current = (current + i) % 100;
        if current == 0 {
            num_zero += 1;
        }
    }
    
    num_zero
}

pub fn part2(inputs: &Vec<i32>, start: i32) -> i32 {
    let mut current = start;
    let mut num_zero = 0;

    for &i in inputs {
        let overrun = current + i;
        let num_laps = overrun.abs() / 100;
        if overrun == 0 || (current != 0 && current.signum() != overrun.signum()) {
            num_zero += 1;
        }
        current = overrun.rem_euclid(100);
        num_zero += num_laps;
    }

    num_zero
}

pub fn day1() {
    let input = fs::read_to_string("inputs/day1.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs, 50)));
    println!("Part 2: {}", timed(|| part2(&inputs, 50)));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let inputs = parse_input(&input);
        assert_eq!(3, part1(&inputs, 50));
    }

    #[test]
    fn test_p2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let inputs = parse_input(&input);
        assert_eq!(6, part2(&inputs, 50));
    }
}