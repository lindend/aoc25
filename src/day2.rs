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

// Execution took 231ms
fn is_repeated_str(current: i64) -> bool {
    let current_str = current.to_string();

    for num_splits in 2..current_str.len() + 1 {
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
// test day2::tests::bench_p2 ... bench: 210,053,762.60 ns/iter (+/- 4,452,027.77)
fn is_repeated(current: i64) -> bool {
    let current_len = current.ilog10() + 1;
    for test_len in 0..current_len {
        let test = current % 10i64.pow(test_len);
        if test == 0 || current % test != 0 {
            continue;
        }
        let divisor = current / test;
        let mut expected = 0;
        for j in 0..current_len / test_len {
            expected += 10i64.pow(test_len * j);
        }

        if divisor == expected {
            return true;
        }
    }

    false
}

// test day2::tests::bench_p2 ... bench: 145,399,576.70 ns/iter (+/- 3,543,887.52)
fn is_repeated_2(current: i64) -> bool {
    let current_len = current.ilog10() + 1;
    for test_len in 1..current_len {
        if current_len % test_len != 0 {
            continue;
        }

        let test = current % 10i64.pow(test_len);
        if test == 0 || current % test != 0 {
            continue;
        }
        let divisor = current / test;
        let mut expected = 0;
        for j in 0..current_len / test_len {
            expected += 10i64.pow(test_len * j);
        }

        if divisor == expected {
            return true;
        }
    }

    false
}

// test day2::tests::bench_p2 ... bench: 145,399,576.70 ns/iter (+/- 3,543,887.52)
pub fn part2_old(ranges: &Vec<Range>) -> i64 {
    let mut sum = 0i64;

    for (min, max) in ranges {
        let mut current = (*min).max(11);
        while current <= *max {
            if is_repeated_2(current) {
                sum += current;
            }
            current += 1;
        }
    }

    sum
}

fn split_number_n(num: i64, n: usize, target: &mut [i64]) -> bool {
    let len = (num.ilog10() + 1) as usize;
    if len % n != 0 {
        return false;
    }

    let mut num = num;

    let mut num_exp = 10i64.pow(((len * n - 1) / n) as u32);
    for i in 0..n {
        target[i] = num / num_exp;
        num = num - target[i] * num_exp;
    }

    true
}

static ten_exp: [i64; 9] = [
    10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

static ten_exp_2: [i64; 10] = [
    1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

#[inline(always)]
fn combine_number(num: i64x8, n: usize) -> i64x8 {
    let mut res = Simd::splat(0);
    let mut exp = Simd::splat(1);
    for i in (0..n).rev() {
        res += num * exp;
        // let logs = num.cast::<f64>().log10();
        // let ten_idx = logs.cast::<usize>();
        // exp *= Simd::gather_or(&ten_exp, ten_idx, Simd::splat(0));
        // let log2 = Simd::splat(64) - num.leading_zeros();
        // let log10 = (Simd::splat(9) * log2).shr(Simd::splat(5)).cast::<usize>();
        // exp *= Simd::gather_or(&ten_exp, log10, Simd::splat(0));
        let num_digits = simd_num_digits(num);
        exp *= Simd::gather_or(&ten_exp_2, num_digits.cast::<usize>(), Simd::splat(0));
    }
    res
}

// test day2::tests::bench_p2_2 ... bench:   6,847,824.40 ns/iter (+/- 219,436.95)
// test day2::tests::bench_p2_2 ... bench:   4,972,764.45 ns/iter (+/- 185,672.51)
pub fn part2(ranges: &Vec<Range>) -> i64 {
    let mut sum = 0i64;

    let mut repeat_buffer = vec![0i64; 100];
    let simd_offsets = Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7]);

    for &(min, max) in ranges {
        let max_num_repeats = (max.ilog10() + 1) as usize;
        let simd_max = Simd::splat(max);
        let simd_min = Simd::splat(min);

        for num_repeats in 2..=max_num_repeats {
            let min_len = (min.ilog10() + 1) as usize;
            let begin_split: i64 = if min_len % num_repeats == 0 {
                split_number_n(min, num_repeats, &mut repeat_buffer);
                *repeat_buffer[0..num_repeats].iter().min().unwrap()
            } else {
                10i64.pow((min_len.div_ceil(num_repeats) - 1) as u32)
            };

            let mut current = Simd::splat(begin_split) + simd_offsets;
            loop {
                let combined = combine_number(current, num_repeats);
                let max_mask = combined.simd_le(simd_max);
                let min_mask = combined.simd_ge(simd_min);

                let masked = max_mask.bitand(min_mask).select(combined, Simd::splat(0));
                sum += masked.reduce_sum();

                if !max_mask.all() {
                    break;
                }

                current += Simd::splat(8);
            }
        }
    }

    sum
}

// test day2::tests::bench_p2_3 ... bench:   1,931,685.50 ns/iter (+/- 7,585.68)
// test day2::tests::bench_p2_3 ... bench:   1,729,865.25 ns/iter (+/- 11,050.66)
pub fn part2_3(ranges: &Vec<Range>) -> i64 {
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
    println!("Part 2: {}", timed(|| part2_3(&inputs)));
}

pub fn day2_p2_bench() {
    let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");
    let inputs = timed(|| parse_input(&input));
    loop {
        part2_3(&inputs);
    }
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
    #[bench]
    fn bench_p2_3(b: &mut Bencher) {
        let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

        let inputs = parse_input(&input);
        println!("HEJ");
        b.iter(|| part2_3(&inputs));
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
    fn split_n() {
        let mut buffer = vec![0i64; 10];
        let ranges = parse_input(&TEST_INPUT);
        assert!(split_number_n(123456, 3, &mut buffer));
        assert_eq!(buffer, [12, 34, 56]);
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
        assert_eq!(2121212121, part2_3(&parse_input(&"2121212118-2121212124")));
        assert_eq!(824824824, part2_3(&parse_input(&"824824821-824824827")));
        assert_eq!(38593859, part2_3(&parse_input(&"38593856-38593862")));
        assert_eq!(446446, part2_3(&parse_input(&"446443-446449")));
        assert_eq!(0, part2_3(&parse_input(&"1698522-1698528")));
        assert_eq!(222222, part2_3(&parse_input(&"222220-222224")));
        assert_eq!(1188511885, part2_3(&parse_input(&"1188511880-1188511890")));
        assert_eq!(999 + 1010, part2_3(&parse_input(&"998-1012")));
    }

    #[test]
    fn test_real() {
        let input = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

        let inputs = parse_input(&input);

        assert_eq!(part1(&inputs), 32976912643);
        assert_eq!(part2_3(&inputs), 54446379122);
    }

    #[test]
    fn test_real_p3_line_by_line() {
        let inputs = fs::read_to_string("inputs/day2.txt").expect("Could not read input");

        let inputs = inputs.split(",");
        let input_lines = inputs.map(|i| parse_input(i));

        for l in input_lines {
            assert_eq!(part2_old(&l), part2_3(&l));
        }
    }
}
