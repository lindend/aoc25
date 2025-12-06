use crate::timed::timed;
use crate::util::str_util::transpose;
use std::fs;

#[derive(PartialOrd, PartialEq, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
struct MathProblems<'a> {
    rows: Vec<Vec<&'a str>>,
    operators: Vec<Operator>,
}

fn all_space(lines: &Vec<&str>, idx: usize) -> bool {
    lines.iter().all(|l| l.as_bytes()[idx] == ' ' as u8)
}

fn parse_input(input: &str) -> MathProblems {
    let lines: Vec<_> = input.split("\n").collect();

    let mut prev_end = 0;

    let mut res = MathProblems {
        rows: Vec::new(),
        operators: Vec::new(),
    };

    let num_lines = lines.len();
    let line_len = lines.iter().map(|l| l.len()).max().unwrap();

    for mut i in 1..line_len {
        if i == line_len - 1 {
            i = line_len
        }

        if i == line_len || all_space(&lines, i) {
            res.rows.push(
                lines[0..num_lines - 1]
                    .iter()
                    .map(|&l| &l[prev_end..i])
                    .collect(),
            );

            let op_line = lines[num_lines - 1];

            let operator_str = op_line[prev_end..i.min(op_line.len())].trim();
            let operator = match operator_str {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                _ => panic!("Unsupported operator {operator_str}"),
            };
            res.operators.push(operator);
            prev_end = i + 1;
        }
    }

    res
}

fn do_calculation(nums: &[&str], operator: &Operator) -> i64 {
    let int_numbers = nums.iter().map(|n| n.trim().parse::<i64>().unwrap());
    match operator {
        Operator::Add => int_numbers.sum(),
        Operator::Multiply => int_numbers.reduce(|f, v| f * v).unwrap(),
    }
}

pub fn part1(input: &MathProblems) -> i64 {
    let mut sum = 0;

    let rows = &input.rows;
    let operators = &input.operators;
    for (nums, operator) in rows.iter().zip(operators) {
        sum += do_calculation(nums, operator);
    }

    sum
}

pub fn part2(input: &MathProblems) -> i64 {
    let mut sum = 0;

    let rows = &input.rows;
    let operators = &input.operators;
    for (nums, operator) in rows.iter().zip(operators) {
        let transposed = transpose(nums);
        let real_nums: Vec<_> = transposed
            .iter()
            .map(|n| n.as_str())
            .collect();
        sum += do_calculation(&real_nums, operator);
    }

    sum
}

pub fn day6() {
    let input = fs::read_to_string("inputs/day6.txt").expect("Could not read input");

    let inputs = timed(|| parse_input(&input));

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(4277556, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(3263827, part2(&input));
    }
}
