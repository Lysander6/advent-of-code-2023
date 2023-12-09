use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    sequences: Vec<Vec<i64>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequences = s
            .lines()
            .flat_map(|l| {
                l.split_ascii_whitespace()
                    .map(str::parse)
                    .collect::<Result<_, _>>()
            })
            .collect();

        Ok(Problem { sequences })
    }
}

fn solve(seq: &[i64]) -> i64 {
    let a = seq.iter().rev().copied().collect::<Vec<_>>();

    let mut diff_seqs: Vec<Vec<i64>> = vec![a];
    let mut ptrs: Vec<usize> = vec![0];

    while *diff_seqs.last().unwrap().first().unwrap() != 0 {
        diff_seqs.push(vec![]);

        for (seq_i, ptr) in ptrs.iter_mut().enumerate() {
            let new = diff_seqs[seq_i][*ptr] - diff_seqs[seq_i][*ptr + 1];
            diff_seqs[seq_i + 1].push(new);

            *ptr += 1;
        }

        ptrs.push(0);
    }

    diff_seqs.into_iter().map(|s| s[0]).sum()
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> i64 {
    let Problem { sequences } = p;
    sequences.iter().map(|seq| solve(seq)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(
            p,
            Problem {
                sequences: vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45]
                ]
            }
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(&[0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 114);
    }
}
