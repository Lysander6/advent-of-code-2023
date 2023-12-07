use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;

#[derive(Debug)]
struct Card {
    winning: HashSet<u8>,
    numbers: HashSet<u8>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_winning, chosen) = s
            .split_once(" | ")
            .ok_or_else(|| anyhow!("couldn't split at ' | '"))?;

        let (_, winning) = card_winning
            .split_once(": ")
            .ok_or_else(|| anyhow!("couldn't split a ': '"))?;

        let winning = winning
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let numbers = chosen
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { winning, numbers })
    }
}

#[derive(Debug)]
pub struct Problem {
    cards: Vec<Card>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

/// # Errors
///
/// Returns error when count of intersection of winning and chosen numbers can't be casted down to
/// u32.
pub fn solve_part_1(p: &Problem) -> Result<u64, anyhow::Error> {
    let Problem { cards } = p;
    let mut result = 0;

    for Card { winning, numbers } in cards {
        let common: u32 = winning.intersection(numbers).count().try_into()?;

        if common > 0 {
            result += 2u64.pow(common.saturating_sub(1));
        }
    }

    Ok(result)
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> u64 {
    let Problem { cards } = p;
    let mut copies = vec![1u64; cards.len()];

    for (i, Card { winning, numbers }) in cards.iter().enumerate() {
        let common = winning.intersection(numbers).count();

        if common == 0 {
            continue;
        }

        let times = copies[i];
        for copy in &mut copies[(i + 1)..=(i + common)] {
            *copy += times;
        }
    }

    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p).unwrap(), 13);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_2(&p), 30);
    }
}
