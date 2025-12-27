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
