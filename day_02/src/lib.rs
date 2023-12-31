use std::str::FromStr;

use anyhow::bail;

#[derive(Debug, Default, PartialEq, Eq)]
struct SubSet(usize, usize, usize); // red, green, blue

impl FromStr for SubSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut subset = SubSet::default();

        for color_count in s.split(", ") {
            let (count, color) = color_count.split_once(' ').expect("malformed color count");

            let count = count.parse()?;

            match color {
                "red" => {
                    subset.0 = count;
                }
                "green" => {
                    subset.1 = count;
                }
                "blue" => {
                    subset.2 = count;
                }
                c => bail!("unknown color {}", c),
            }
        }

        Ok(subset)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: usize,
    subsets: Vec<SubSet>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id_str, subsets) = s.split_once(": ").expect("malformed game");

        let id = game_id_str
            .split_once(' ')
            .expect("malformed game prefix")
            .1
            .parse::<usize>()?;

        let subsets = subsets
            .split("; ")
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Game { id, subsets })
    }
}

#[derive(Debug)]
pub struct Problem {
    games: Vec<Game>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Problem { games })
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { games } = p;

    games
        .iter()
        .filter_map(|Game { id, subsets }| {
            subsets
                .iter()
                .all(|&SubSet(r, g, b)| r <= 12 && g <= 13 && b <= 14)
                .then_some(id)
        })
        .sum()
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> usize {
    let Problem { games } = p;

    games
        .iter()
        .map(|Game { subsets, .. }| {
            let (max_r, max_g, max_b) = subsets
                .iter()
                .fold((0, 0, 0), |(x, y, z), &SubSet(r, g, b)| {
                    (x.max(r), y.max(g), z.max(b))
                });

            max_r * max_g * max_b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_subset_parsing() {
        assert_eq!("3 blue, 4 red".parse::<SubSet>().unwrap(), SubSet(4, 0, 3));
        assert_eq!(
            "1 red, 2 green, 6 blue".parse::<SubSet>().unwrap(),
            SubSet(1, 2, 6)
        );
        assert_eq!("2 green".parse::<SubSet>().unwrap(), SubSet(0, 2, 0));
        assert_eq!(
            "8 green, 6 blue, 20 red".parse::<SubSet>().unwrap(),
            SubSet(20, 8, 6)
        );
    }

    #[test]
    fn test_game_parsing() {
        assert_eq!(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
                .parse::<Game>()
                .unwrap(),
            Game {
                id: 5,
                subsets: vec![SubSet(6, 3, 1), SubSet(1, 2, 2)]
            }
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 8);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_2(&p), 2286);
    }
}
