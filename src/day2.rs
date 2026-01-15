use crate::{
    timed::timed,
    util::{
        search::{binary_search_leftmost, binary_search_rightmost},
        simd_util::simd_num_digits,
    },
};
use std::{
    fs,
    ops::{BitAnd, Shr},
    simd::{
        Simd, StdFloat,
        cmp::{SimdOrd, SimdPartialOrd},
        f64x8, i64x8,
        num::{SimdFloat, SimdInt, SimdUint},
    },
};

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

fn split_number(num: i64) -> (i64, i64) {
    // +2 because we want to split so that uneven length
    // numbers have a longer right part.
    // E.g. 12345 -> (12, 345)
    let len = num.ilog10() + 2;

    let midpoint = len / 2;

    let midpoint_exp = 10i64.pow(midpoint);
    let left = num / midpoint_exp;
    let right = num - left * midpoint_exp;

    (left, right)
}

fn combine_numbers(left: i64, right: i64) -> i64 {
    let right_len = right.ilog10() + 1;
    left * 10i64.pow(right_len) + right
}

pub fn part1(ranges: &Vec<Range>) -> i64 {
    let mut sum = 0i64;

    for &(min, max) in ranges {
        let mut current = min.max(11);
        while current <= max {
            let (left, right) = split_number(current);
            let next = if left == right {
                if current >= min {
                    debug_assert!(current >= min);
                    debug_assert!(current <= max);

                    sum += current;
                }
                left + 1
            } else {
                left
            };
            current = combine_numbers(next, next);
        }
    }

    sum
}

static ten_exp_2: [i64; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

#[inline(always)]
fn combine_number(num: i64x8, n: usize) -> i64x8 {
    let mut res = Simd::splat(0);
    let mut exp = Simd::splat(1);
    for i in (0..n).rev() {
        res += num * exp;
        let num_digits = simd_num_digits(num);
        exp *= Simd::gather_or(&ten_exp_2, num_digits.cast::<usize>(), Simd::splat(0));
    }
    res
}

pub fn part2(ranges: &Vec<Range>) -> i64 {
    let mut min_min = *ranges.iter().map(|(min, _)| min).min().unwrap();
    let mut max_max = *ranges.iter().map(|(_, max)| max).max().unwrap();
    let mut max_num_repeats = max_max.ilog10() + 1;
    let mut duplicates = Vec::new();

    for num_repeats in 2..=max_num_repeats {
        let mut current = Simd::from_array([1, 2, 3, 4, 5, 6, 7, 8]);

        let mut in_range = true;
        while in_range {
            let combined = combine_number(current, num_repeats as usize);

            if combined.reduce_max() > max_max {
                in_range = false;
            }
            for n in combined.to_array() {
                duplicates.push(n);
            }
            current += Simd::splat(8);
        }
    }

    duplicates.sort();
    duplicates.dedup();

    let mut total = 0;
    for &(min, max) in ranges {
        let start = binary_search_leftmost(&duplicates, min);
        let end = binary_search_rightmost(&duplicates, max);
        if start <= end {
            total += duplicates[start..=end].iter().sum::<i64>();
        }
    }
    total
}

pub fn day2() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::timed::print_timespan;

    use super::*;
    use test::Bencher;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_split() {
        assert_eq!(split_number(123456), (123, 456));
        assert_eq!(split_number(1234567), (123, 4567));
    }

    #[test]
    fn test_combine() {
        assert_eq!(combine_numbers(123, 456), (123456));
        assert_eq!(combine_numbers(1234, 567), (1234567));
    }

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

        let inputs = parse_input(&input);

        b.iter(|| part1(&inputs));
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

        let inputs = parse_input(&input);

        assert_eq!(part2(&inputs), 54446379122);

        b.iter(|| part2(&inputs));
    }

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
    fn combine_n() {
        let ranges = parse_input(&TEST_INPUT);
        assert_eq!(combine_number(Simd::splat(12), 3), Simd::splat(121212));
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

    #[test]
    fn test_p2_large_range() {
        assert_eq!(2121212121, part2(&parse_input(&"2121212118-2121212124")));
        assert_eq!(824824824, part2(&parse_input(&"824824821-824824827")));
        assert_eq!(38593859, part2(&parse_input(&"38593856-38593862")));
        assert_eq!(446446, part2(&parse_input(&"446443-446449")));
        assert_eq!(0, part2(&parse_input(&"1698522-1698528")));
        assert_eq!(222222, part2(&parse_input(&"222220-222224")));
        assert_eq!(1188511885, part2(&parse_input(&"1188511880-1188511890")));
        assert_eq!(999 + 1010, part2(&parse_input(&"998-1012")));
    }

    #[test]
    fn test_real() {
        let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

        let inputs = parse_input(&input);

        assert_eq!(part1(&inputs), 32976912643);
        assert_eq!(part2(&inputs), 54446379122);
    }
}
