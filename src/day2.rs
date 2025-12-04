use std::fs;
use crate::timed::timed;

type Range = (i64, i64);
fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(",")
        .map(|l| l.split_once("-").expect("No separator found"))
        .map(|(min, max)| {
            (
                min.parse()
                    .expect(&*format!("Could not parse min, {}", min)),
                max.parse().expect("Could not parse max"),
            )
        })
        .collect()
}

pub fn part1(ranges: &Vec<Range>) -> i64 {
    let mut sum = 0i64;

    for (min, max) in ranges {
        let mut current = (*min).max(11);
        while current <= *max {
            let current_str = current.to_string();
            let half_point = current_str.len() / 2;
            let next = if current_str[..half_point] == current_str[half_point..] {
                if current >= *min {
                    assert!(current >= *min);
                    assert!(current <= *max);

                    sum += current;
                }
                (current_str[..half_point].parse::<i64>().unwrap()) + 1
            } else {
                current_str[..half_point].parse::<i64>().unwrap()
            };
            current = format!("{next}{next}").parse().unwrap();
        }
    }

    sum
}

// Execution took 231ms
fn is_repeated_str(current: i64) -> bool {
    let current_str = current.to_string();

    for num_splits in 2..current_str.len()+1 {
        if current_str.len() % num_splits != 0 {
            continue;
        }

        let repeater = &current_str[..current_str.len() / num_splits];
        if current_str == repeater.repeat(num_splits) {
            return true;
        }
    }

    false
}

// Execution took 202ms
fn is_repeated(current: i64) -> bool {
    let current_len = current.ilog10() + 1;
    for test_len in 0..current_len{
        let test = current % 10i64.pow(test_len);
        if test == 0 || current % test != 0 {
            continue
        }
        let divisor = current / test;
        let mut expected = 0;
        for j in 0..current_len/test_len {
            expected += 10i64.pow(test_len * j);
        }

        if divisor == expected {
            return true
        }
    }

    false
}

pub fn part2(ranges: &Vec<Range>) -> i64 {
    let mut sum = 0i64;

    for (min, max) in ranges {
        let mut current = (*min).max(11);
        while current <= *max {
            if is_repeated(current) {
                sum += current;
            }
            current += 1;
        }
    }

    sum
}

pub fn day2() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

    let inputs = parse_input(&input);

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1() {
        let ranges = parse_input(&TEST_INPUT);
        assert_eq!(1227775554, part1(&ranges));
    }

    #[test]
    fn test_p1_small_range() {
        let ranges = parse_input(&"11-22,95-115");
        assert_eq!(132, part1(&ranges));
    }

    #[test]
    fn test_p1_1010() {
        let ranges = parse_input(&"998-1012");
        assert_eq!(1010, part1(&ranges));
    }

    #[test]
    fn test_p1_other_range() {
        let ranges = parse_input(&"1188511880-1188511890");
        assert_eq!(1188511885, part1(&ranges));
    }

    #[test]
    fn test_p1_no_ranges() {
        let ranges = parse_input(&"2121212118-2121212124");
        assert_eq!(0, part1(&ranges));
    }

    #[test]
    fn test_p2() {
        let ranges = parse_input(&TEST_INPUT);
        assert_eq!(4174379265, part2(&ranges));
    }

    #[test]
    fn test_p2_small_range() {
        let ranges = parse_input(&"11-22,95-115");
        assert_eq!(243, part2(&ranges));
    }
}
