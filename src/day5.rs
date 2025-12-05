use crate::timed::timed;
use crate::util::search::{binary_search_leftmost, binary_search_rightmost};
use std::fs;

type Range = (i64, i64);
struct Inventory {
    fresh: Vec<Range>,
    ingredients: Vec<i64>,
}

fn parse_input(input: &str) -> Inventory {
    let (range_input, ingredient_input) = input.split_once("\n\n").expect("Could not parse input");

    let fresh = range_input
        .lines()
        .filter_map(|l| l.split_once("-"))
        .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
        .collect();

    let ingredients = ingredient_input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    Inventory { fresh, ingredients }
}

pub fn part1(inventory: &Inventory) -> i64 {
    let mut ingredients = inventory.ingredients.clone();
    ingredients.sort();

    let mut num_range_cover = vec![0; ingredients.len()];

    for (min, max) in &inventory.fresh {
        let start_idx = binary_search_leftmost(&ingredients, *min);
        let end_idx = binary_search_rightmost(&ingredients, *max);

        for i in start_idx..=end_idx {
            num_range_cover[i] += 1;
        }
    }

    num_range_cover.iter().filter(|r| **r > 0).count() as i64
}

#[derive(PartialEq)]
enum FreshRangeMarker {
    Begin,
    End,
}

pub fn part2(inventory: &Inventory) -> i64 {
    let mut fresh_list: Vec<_> = inventory
        .fresh
        .iter()
        .flat_map(|(min, max)| [(FreshRangeMarker::Begin, min), (FreshRangeMarker::End, max)])
        .collect();

    fresh_list.sort_by_key(|(_, v)| **v);

    let mut stack_counter = 0;
    let mut num_fresh_ingredients = 0;
    let mut start_idx = 0;

    for (marker, &idx) in fresh_list {
        if marker == FreshRangeMarker::Begin {
            if stack_counter == 0 {
                start_idx = idx.max(start_idx);
            }
            stack_counter += 1;
        } else {
            stack_counter -= 1;

            if stack_counter == 0 {
                num_fresh_ingredients += idx - start_idx + 1;
                start_idx = idx + 1;
            }
        }
    }

    num_fresh_ingredients
}

pub fn day5() {
    let input = fs::read_to_string("inputs/day5.txt").expect("Could not read input");

    let inputs = parse_input(&input);

    println!("Part 1: {}", timed(|| part1(&inputs)));
    println!("Part 2: {}", timed(|| part2(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    const TEST_INPUT2: &str = "3-5
5-10

1";

    #[test]
    fn test_p1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(14, part2(&input));
    }

    #[test]
    fn test_p2_2() {
        let input = parse_input(&TEST_INPUT2);
        assert_eq!(8, part2(&input));
    }
}
