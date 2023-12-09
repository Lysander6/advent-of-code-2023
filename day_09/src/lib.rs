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

    while diff_seqs
        .last()
        .and_then(|s| s.first())
        .is_some_and(|&n| n != 0)
    {
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

fn solve2(seq: &[i64]) -> i64 {
    let mut diff_seqs: Vec<Vec<i64>> = vec![seq.to_vec()];
    let mut ptrs: Vec<usize> = vec![0];

    'outer: while !diff_seqs
        .get(diff_seqs.len().saturating_sub(3))
        .is_some_and(|v| v.len() > 2 && v.iter().all(|&n| n == 0))
    {
        diff_seqs.push(vec![]);

        for (seq_i, ptr) in ptrs.iter_mut().enumerate() {
            if *ptr + 1 == diff_seqs[seq_i].len() {
                break 'outer;
            }
            let new = diff_seqs[seq_i][*ptr + 1] - diff_seqs[seq_i][*ptr];
            diff_seqs[seq_i + 1].push(new);

            *ptr += 1;
        }

        ptrs.push(0);
    }

    diff_seqs
        .iter()
        .rev()
        .filter_map(|v| v.first())
        .fold(0, |acc, s| -acc + s)
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> i64 {
    let Problem { sequences } = p;
    sequences.iter().map(|seq| solve(seq)).sum()
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> i64 {
    let Problem { sequences } = p;
    sequences.iter().map(|seq| solve2(seq)).sum()
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

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(solve2(&[1, 3, 6, 10, 15, 21]), 0);
        assert_eq!(solve2(&[10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_2(&p), 2);
    }

    #[test]
    fn test_optimized_solve2() {
        assert_eq!(
            solve2(&[
                3, 10, 27, 68, 169, 402, 899, 1895, 3816, 7471, 14456, 27943, 54110, 104_575,
                200_340, 377_950, 698_857, 1_263_392, 2_231_345, 3_852_006, 6_507_719
            ]),
            4
        );
    }
}
