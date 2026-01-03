#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Indicator {
    On,
    Off,
}

impl Indicator {
    pub fn toggle(&mut self) {
        *self = match self {
            Indicator::Off => Indicator::On,
            Indicator::On => Indicator::Off,
        };
    }
}

type Input = Vec<(Vec<Indicator>, Vec<Vec<usize>>, Vec<usize>)>;

fn read_num(input: &[u8], i: &mut usize) -> usize {
    let mut num = 0;
    loop {
        match input[*i] {
            c if c >= b'0' && c <= b'9' => num = num * 10 + (c - b'0') as usize,
            _ => {
                return num;
            }
        }
        *i += 1;
    }
}

fn read_numbers(input: &[u8], i: &mut usize) -> Vec<usize> {
    let mut res = Vec::new();
    while *i < input.len() {
        match input[*i] {
            b']' | b')' | b'}' => {
                *i += 1;
                return res;
            }
            b',' => {
                *i += 1;
            }
            _ => {
                res.push(read_num(input, i));
            }
        };
    }
    panic!("No closing character")
}

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut indicators = None;
            let mut buttons = Vec::new();
            let mut joltage_req = None;

            let l = l.as_bytes();
            let mut i = 0;
            while i < l.len() {
                match l[i] {
                    b'[' => {
                        let mut v = Vec::new();
                        i += 1;
                        while l[i] != b']' {
                            v.push(match l[i] {
                                b'.' => Indicator::Off,
                                b'#' => Indicator::On,
                                _ => panic!("Weird input {}", l[i] as char),
                            });
                            i += 1;
                        }
                        indicators = Some(v);
                    }
                    b'(' => {
                        i += 1;
                        buttons.push(read_numbers(l, &mut i));
                    }
                    b'{' => {
                        i += 1;
                        joltage_req = Some(read_numbers(l, &mut i));
                    }
                    b' ' => {}
                    _ => panic!("Weird input {}", l[i] as char),
                };
                i += 1;
            }

            (
                indicators.expect("No indicators"),
                buttons,
                joltage_req.expect("No joltage requirement"),
            )
        })
        .collect()
}

fn part2(input: &Input) -> i64 {
    let mut num_presses = 0;
    for (_, buttons, joltages) in input {
        println!("One row");
        let max_presses: Vec<usize> = buttons
            .iter()
            .map(|b| {
                b.iter()
                    .map(|&joltage_index| joltages[joltage_index] + 1)
                    .filter(|&j| j > 0)
                    .min()
                    .unwrap()
            })
            .collect();

        let total_presses: usize = max_presses
            .iter()
            .copied()
            .reduce(|a, p| a.max(1) * p.max(1))
            .unwrap();
        let mut min_num_presses = i64::MAX;

        let mut press = 1;
        'press: while press < total_presses {
            let mut current_joltages = vec![0; joltages.len()];
            let mut remaining_presses = press;
            let mut current_num_presses = 0;
            let mut button_factor = 1;
            for btn_idx in 0..buttons.len() {
                let btn_max_presses = max_presses[btn_idx];
                let num_presses = remaining_presses % btn_max_presses;
                remaining_presses = remaining_presses / btn_max_presses;
                current_num_presses += num_presses;

                for &joltage_index in &buttons[btn_idx] {
                    current_joltages[joltage_index] += num_presses;
                    if current_joltages[joltage_index] > joltages[joltage_index] {
                        press += ((btn_max_presses - num_presses) * button_factor).max(1);
                        continue 'press;
                    }
                }
            }

            if current_joltages
                .iter()
                .zip(joltages)
                .all(|(j0, j1)| *j0 == *j1)
            {
                min_num_presses = min_num_presses.min(current_num_presses as i64);
            }
            press += 1;
        }
        num_presses += min_num_presses;
    }
    num_presses
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_p2() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(33, part2(&input));
    }

    #[test]
    fn test_p2_2() {
        let input = parse_input(&"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(12, part2(&input));
    }

    #[test]
    fn test_p2_3() {
        let input = parse_input(
            &"[..##..] (0,5) (1,2,3,4,5) (1,3,4,5) (3,4) (2,3,5) (0,1,2,5) {29,40,23,42,39,52}",
        );
        assert_eq!(part2(&input), 71);
    }

    #[test]
    fn test_p2_4() {
        let input = parse_input(
            &"[..#...#.#] (2,7) (1,4,7) (0,1,3,4,5,6,8) (2,3,4,5,6,7,8) (1,4,6,7) (0,2,4,5,6,7,8) (0,5,7) (0,1,3,5,6,7,8) (0,4,6) (0,1,2,5,6,7,8) (0,1,2,3,5) {237,230,49,207,213,228,221,72,200}",
        );
        assert_eq!(part2(&input), 267);
    }

    #[test]
    fn test_p2_5() {
        let input = parse_input(&"[####] (1,3) (2,3) (3) (0,1,2) (0,2) (0,1) {21,23,21,27}");
        assert_eq!(part2(&input), 48);
    }
}
