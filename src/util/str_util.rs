pub fn transpose(lines: &[&str]) -> Vec<String> {
    let mut res = Vec::new();

    for line in lines {
        for (i, c) in line.char_indices() {
            // Make room in the result list
            while res.len() < i + 1 {
                res.push(String::new())
            }

            res[i].push(c);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = "A1
B2
C3";
        let result = transpose(&input.lines().collect::<Vec<_>>());

        assert_eq!(vec!["ABC", "123"], result);
    }
}
