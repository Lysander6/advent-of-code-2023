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
    let to_digit = |c: &char| c.to_digit(10);

    p.lines
        .iter()
        .map(|l| {
            let nums = l.chars().filter(char::is_ascii_digit).collect::<Vec<_>>();

            let a = nums.first().and_then(to_digit).unwrap_or(0);
            let b = nums.last().and_then(to_digit).unwrap_or(0);

            a * 10 + b
        })
        .sum()
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> u32 {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let to_digit_with_idx = |&(idx, c): &(usize, char)| Some((idx, c.to_digit(10).unwrap_or(0)));

    let mut result = 0;

    for line in &p.lines {
        let first_word_digit_idx = (1u32..)
            .zip(digits)
            .filter_map(|(val, word)| line.find(word).map(|idx| (idx, val)))
            .min();

        let last_word_digit_idx = (1u32..)
            .zip(digits)
            .filter_map(|(val, word)| line.rfind(word).map(|idx| (idx, val)))
            .max();

        let digit_chars = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_digit())
            .collect::<Vec<_>>();

        let first_real_digit_idx = digit_chars.first().and_then(to_digit_with_idx);
        let last_real_digit_idx = digit_chars.last().and_then(to_digit_with_idx);

        let a = match (first_word_digit_idx, first_real_digit_idx) {
            (None, None) => 0,
            (None, Some((_, v))) | (Some((_, v)), None) => v,
            (Some((idx_a, val_a)), Some((idx_b, val_b))) => {
                if idx_a < idx_b {
                    val_a
                } else {
                    val_b
                }
            }
        };

        let b = match (last_word_digit_idx, last_real_digit_idx) {
            (None, None) => 0,
            (None, Some((_, v))) | (Some((_, v)), None) => v,
            (Some((idx_a, val_a)), Some((idx_b, val_b))) => {
                if idx_a > idx_b {
                    val_a
                } else {
                    val_b
                }
            }
        };

        result += a * 10 + b;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 142);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT_2.parse().unwrap();
        assert_eq!(solve_part_2(&p), 281);
    }
}
