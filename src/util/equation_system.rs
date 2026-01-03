use std::cmp::Ordering;

pub type GaussEliminationResult = Vec<Vec<f64>>;

fn shape(m: &Vec<Vec<f64>>) -> (usize, usize) {
    let height = m.len();
    assert_ne!(height, 0);
    let width = m[0].len();
    assert_ne!(width, 0);
    (width, height)
}

fn cmp_vec(a: &Vec<f64>, b: &Vec<f64>) -> Ordering {
    let len = a.len().min(b.len());
    for i in 0..len {
        let c = a[i].abs().total_cmp(&b[i].abs());
        if c != Ordering::Equal {
            return c;
        }
    }
    a.len().cmp(&b.len())
}

fn make_stair_shape(m: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let (width, height) = shape(m);

    let mut res = Vec::new();

    // Loop through all parameters and attempt to form a stair
    for i in 0..(width - 1) {
        // Find the first row (the pivot) with a non-zero parameter at the current index
        let pivot = m
            .iter()
            .enumerate()
            .filter(|(_, v)| v[i] != 0.0)
            .min_by(|a, b| cmp_vec(a.1, b.1));

        if let Some((pivot_index, pivot_value)) = pivot {
            let pivot_value = pivot_value.clone();
            res.push(pivot_value.clone());
            m.remove(pivot_index);

            for (j, row_value) in m.iter_mut().enumerate() {
                // Zero in this position means nothing to remove
                if row_value[i] == 0.0 {
                    continue;
                }

                // Remove all pivot row values
                let factor = row_value[i] / pivot_value[i];
                for l in 0..width {
                    row_value[l] -= pivot_value[l] * factor;
                }
                assert_eq!(row_value[i], 0.0);
            }
        }
    }
    res
}

pub fn gauss_elimination(m: &Vec<Vec<f64>>) -> GaussEliminationResult {
    println!("Running gauss elimination on {m:?}");
    let mut m = m.clone();
    let (width, height) = shape(&m);

    let mut m = make_stair_shape(&mut m);
    println!("After normalizing rows: {m:?}");

    m.sort_unstable_by(cmp_vec);

    let mut res = vec![vec![None; width - 1]];
    for (row_idx, row) in m.iter().enumerate() {
        let first_col = row.iter().enumerate().find(|&(i, p)| *p != 0.0);
        if first_col.is_none() {
            continue;
        }
        let (first_col_idx, col_value) = first_col.unwrap();

        let mut value = vec![0.0; res.len()];
        value[0] = row[width - 1];

        for col_idx in ((first_col_idx + 1)..(width - 1)).rev() {
            let mut v = row[col_idx];
            if v == 0.0 {
                continue;
            }

            for value_idx in 0..res.len() {
                value[value_idx] -= res[value_idx][col_idx].unwrap_or(0.0) * v;
            }

            if res[0][col_idx].is_none() {
                // This variable in the equation is assigned a parameter
                // instead.
                res[0][col_idx] = Some(0.0);
                let mut param_vec = vec![Some(0.0); width - 1];
                param_vec[col_idx] = Some(1.0);
                res.push(param_vec);
                value.push(-v);
            }
        }

        assert_eq!(value.len(), res.len());

        if res[0][first_col_idx].is_some() {
            continue;
        }

        for result_index in 0..value.len() {
            res[result_index][first_col_idx] = Some(value[result_index] / col_value);
        }
    }

    res.iter()
        .map(|r| r.iter().map(|v| v.unwrap()).collect())
        .collect()
}

pub fn equation_system_i64_to_f64(m: &Vec<Vec<i64>>) -> Vec<Vec<f64>> {
    m.iter()
        .map(|r| r.iter().map(|&v| v as f64).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![
            vec![0, 0, 0, 0, 1, 1, 3],
            vec![0, 1, 0, 1, 0, 1, 5],
            vec![0, 0, 1, 1, 1, 0, 4],
            vec![1, 1, 0, 1, 0, 0, 7],
        ];

        let r = gauss_elimination(&equation_system_i64_to_f64(&input));

        assert_eq!(
            r,
            equation_system_i64_to_f64(&vec![
                vec![2, 5, 1, 0, 3, 0],
                vec![1, -1, 1, 0, -1, 1],
                vec![0, -1, -1, 1, 0, 0]
            ])
        );
    }

    #[test]
    fn test_2() {
        let input = vec![
            vec![0, 0, 1, 0, 0, 1, 1, 168],
            vec![0, 0, 1, 0, 0, 1, 1, 168],
            vec![1, 1, 0, 1, 0, 0, 0, 27],
            vec![0, 1, 1, 0, 0, 1, 1, 171],
            vec![0, 0, 0, 1, 1, 1, 0, 29],
            vec![1, 0, 1, 1, 0, 0, 0, 27],
        ];
        let r = gauss_elimination(&equation_system_i64_to_f64(&input));

        assert_eq!(
            r,
            vec![
                vec![160.0, 3.0, 3.0, -136.0, 0.0, 165.0, 0.0],
                vec![-1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 1.0],
                vec![1.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0]
            ]
        );
    }

    #[test]
    fn test_3() {
        let input = vec![
            vec![
                1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 48.0,
            ],
            vec![
                1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 67.0,
            ],
            vec![
                1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 65.0,
            ],
            vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 30.0,
            ],
            vec![
                1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 85.0,
            ],
            vec![
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 51.0,
            ],
            vec![
                1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 90.0,
            ],
            vec![
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 65.0,
            ],
            vec![
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 63.0,
            ],
            vec![
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 35.0,
            ],
        ];

        let r = gauss_elimination(&input);

        assert_eq!(
            r,
            equation_system_i64_to_f64(&vec![
                vec![2, 5, 1, 0, 3, 0],
                vec![1, -1, 1, 0, -1, 1],
                vec![0, -1, -1, 1, 0, 0]
            ])
        );
    }

    #[test]
    fn test_5() {
        let input = vec![vec![2.0, 10.0]];

        let r = gauss_elimination(&input);

        assert_eq!(r, vec![vec![5.0]]);
    }

    #[test]
    fn test_4() {
        let input = vec![
            vec![1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 69.0],
            vec![1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 38.0],
            vec![0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 26.0],
            vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 29.0],
            vec![1.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 38.0],
            vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 36.0],
            vec![1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 60.0],
            vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 45.0],
        ];
        let r = gauss_elimination(&input);

        assert_eq!(
            r,
            equation_system_i64_to_f64(&vec![vec![5, 9, 13, 20, 1, 0, 3, 19]])
        );
    }
}
