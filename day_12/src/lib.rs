use std::str::FromStr;

use anyhow::bail;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for SpringState {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            c => bail!("couldn't parse SpringState from {:?}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ConditionRecord {
    record: Vec<SpringState>,
    criteria: Vec<usize>,
}

impl FromStr for ConditionRecord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((record, criteria)) = s.split_once(' ') else {
            bail!("couldn't split at ' '");
        };

        let record = record
            .chars()
            .map(std::convert::TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let criteria = criteria
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { record, criteria })
    }
}

#[derive(Debug)]
pub struct Problem {
    records: Vec<ConditionRecord>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            records: s.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

fn is_valid_arrangement(record: &[SpringState], criteria: &[usize]) -> bool {
    let mut record = record.iter().peekable();

    let mut criteria_ptr = 0;
    let mut expected_damaged_springs: isize = (*criteria.get(criteria_ptr).unwrap_or(&0))
        .try_into()
        .unwrap();

    loop {
        while record.next_if_eq(&&SpringState::Operational).is_some() {}

        if record.peek().is_none() {
            break;
        }

        while record.next_if_eq(&&SpringState::Damaged).is_some() {
            expected_damaged_springs -= 1;
        }

        if expected_damaged_springs == 0 {
            criteria_ptr += 1;
            expected_damaged_springs = (*criteria.get(criteria_ptr).unwrap_or(&0))
                .try_into()
                .unwrap();
            continue;
        }

        return false;
    }

    criteria_ptr == criteria.len()
}

// Part 1: brute force
// Determine number of missing damaged springs, then change this many `?` to `#` and rest to `.`
// and check if record is valid.

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { records } = p;

    let mut result = 0;

    for ConditionRecord { record, criteria } in records {
        let found_damaged = record
            .iter()
            .filter(|&s| *s == SpringState::Damaged)
            .count();
        let expected_damaged = criteria.iter().sum::<usize>();

        let missing_damaged = expected_damaged - found_damaged;
        let unknown_indices = record
            .iter()
            .enumerate()
            .filter_map(|(i, s)| (*s == SpringState::Unknown).then_some(i));

        for indices_to_damage in unknown_indices.combinations(missing_damaged) {
            let mut new_record: Vec<_> = record
                .clone()
                .into_iter()
                .map(|s| {
                    if s == SpringState::Unknown {
                        SpringState::Operational
                    } else {
                        s
                    }
                })
                .collect();

            for i in indices_to_damage {
                new_record[i] = SpringState::Damaged;
            }

            if is_valid_arrangement(&new_record, criteria) {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(p.records.len(), 6);
        assert_eq!(
            p.records[0],
            ConditionRecord {
                record: vec![
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Unknown,
                    SpringState::Operational,
                    SpringState::Damaged,
                    SpringState::Damaged,
                    SpringState::Damaged
                ],
                criteria: vec![1, 1, 3]
            }
        );
    }

    #[test]
    fn test_is_valid_arrangement() {
        // valid
        let ConditionRecord { criteria, record } = ".###.##.#... 3,2,1".parse().unwrap();
        assert!(is_valid_arrangement(&record, &criteria));
        let ConditionRecord { criteria, record } = ".###..##...# 3,2,1".parse().unwrap();
        assert!(is_valid_arrangement(&record, &criteria));

        // invalid
        let ConditionRecord { criteria, record } = ".#####...# 3,2,1".parse().unwrap();
        assert!(!is_valid_arrangement(&record, &criteria));
        let ConditionRecord { criteria, record } = ".###...## 3,2,1".parse().unwrap();
        assert!(!is_valid_arrangement(&record, &criteria));
        let ConditionRecord { criteria, record } = ".######. 3,2,1".parse().unwrap();
        assert!(!is_valid_arrangement(&record, &criteria));
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 21);
    }
}
