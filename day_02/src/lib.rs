use std::str::FromStr;

use anyhow::bail;

#[derive(Debug, PartialEq, Eq)]
struct SubSet(usize, usize, usize); // red, green, blue

impl FromStr for SubSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for colour_count in s.split(", ") {
            match colour_count.split_once(' ') {
                Some((count, color)) => match color {
                    "red" => {
                        red = count.parse()?;
                    }
                    "green" => {
                        green = count.parse()?;
                    }
                    "blue" => {
                        blue = count.parse()?;
                    }
                    _ => unreachable!(),
                },
                None => unreachable!(),
            }
        }

        Ok(SubSet(red, green, blue))
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
        let Some((game_id_str, subsets)) = s.split_once(": ") else {
            bail!("malformed game");
        };

        let id = game_id_str
            .split_once(' ')
            .and_then(|(_, id)| id.parse::<usize>().ok())
            .unwrap();

        let subsets = subsets
            .split("; ")
            .map(str::parse)
            .collect::<Result<Vec<SubSet>, _>>()?;

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
        let games = s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?;

        Ok(Problem { games })
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { games } = p;

    games
        .iter()
        .filter_map(|Game { id, subsets }| {
            if subsets
                .iter()
                .all(|ss| ss.0 <= 12 && ss.1 <= 13 && ss.2 <= 14)
            {
                Some(id)
            } else {
                None
            }
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
}
