use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

impl TryFrom<char> for Instruction {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            c => bail!("couldn't parse instruction from {:?}", c),
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    instructions: Vec<Instruction>,
    map: HashMap<String, (String, String)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, map) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("couldn't split input"))?;

        let instructions = instructions
            .chars()
            .map(std::convert::TryInto::try_into)
            .collect::<Result<_, _>>()?;

        let map = map
            .lines()
            .map(|l| -> Result<_, anyhow::Error> {
                let (key, pair) = l
                    .split_once(" = ")
                    .ok_or_else(|| anyhow!("couldn't split {:?} at ' = '", l))?;

                Ok((
                    key.to_string(),
                    (pair[1..4].to_string(), pair[6..9].to_string()),
                ))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { instructions, map })
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { instructions, map } = p;
    let mut current_node = "AAA";

    for (i, instr) in instructions.iter().cycle().enumerate() {
        if current_node == "ZZZ" {
            return i;
        }

        let paths = &map[current_node];

        match instr {
            Instruction::Left => current_node = &paths.0,
            Instruction::Right => current_node = &paths.1,
        }
    }

    unreachable!()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a.rem_euclid(b))
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> Option<usize> {
    let Problem { instructions, map } = p;
    let starting_nodes = map.keys().filter(|k| k.ends_with('A'));

    let steps_to_reach = starting_nodes
        .map(|n| {
            let mut current_node = n;

            for (i, instr) in instructions.iter().cycle().enumerate() {
                if current_node.ends_with('Z') {
                    // TODO: check if directly checking last index isn't more efficient
                    return i;
                }

                let paths = &map[current_node];

                match instr {
                    Instruction::Left => current_node = &paths.0,
                    Instruction::Right => current_node = &paths.1,
                }
            }

            unreachable!()
        })
        .collect::<Vec<_>>();

    steps_to_reach.into_iter().reduce(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(p.instructions, vec![Instruction::Right, Instruction::Left]);
        assert_eq!(p.map.len(), 7);
        assert_eq!(p.map["AAA"], ("BBB".to_string(), "CCC".to_string()));
    }

    #[test]
    fn test_solve_part_1() {
        let p1: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p1), 2);
        let p2: Problem = TEST_INPUT_2.parse().unwrap();
        assert_eq!(solve_part_1(&p2), 6);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT_3.parse().unwrap();
        assert_eq!(solve_part_2(&p), Some(6));
    }
}
