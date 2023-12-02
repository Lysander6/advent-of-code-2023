use std::str::FromStr;

#[derive(Debug)]
pub struct Problem {
    lines: Vec<String>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Problem {
            lines: s.lines().map(std::borrow::ToOwned::to_owned).collect(),
        })
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> u32 {
    p.lines
        .iter()
        .map(|l| {
            let nums = l
                .chars()
                .filter(char::is_ascii_digit)
                .map(|c| c.to_digit(10).unwrap_or(0))
                .collect::<Vec<_>>();

            let a = *nums.first().unwrap_or(&0);
            let b = *nums.last().unwrap_or(&0);

            a * 10 + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 142);
    }
}
