use std::str::FromStr;

use anyhow::bail;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    CubeRock,
    RoundRock,
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::CubeRock),
            'O' => Ok(Self::RoundRock),
            c => bail!("couldn't parse Cell from {:?}", c),
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    map: Vec<Vec<Cell>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|l| l.chars().map(std::convert::TryInto::try_into).collect())
            .collect::<Result<_, _>>()?;

        Ok(Self { map })
    }
}

fn slide_north(map: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut map = map.to_owned();
    let rows = map.len();

    for col in 0..map[0].len() {
        let (cube_indices, round_indices): (Vec<_>, Vec<_>) = (0..map.len())
            .filter(|&i| map[i][col] == Cell::CubeRock || map[i][col] == Cell::RoundRock)
            .partition(|&i| map[i][col] == Cell::CubeRock);

        let mut fall_to_idx = 0;

        let mut cube_indices = cube_indices.into_iter().peekable();
        let mut round_indices = round_indices.into_iter().peekable();

        loop {
            while round_indices
                .peek()
                .is_some_and(|i| i < cube_indices.peek().unwrap_or(&usize::MAX))
            {
                let round_idx = round_indices.next().unwrap();
                map[round_idx][col] = Cell::Empty;
                map[fall_to_idx][col] = Cell::RoundRock;
                fall_to_idx += 1;
            }

            if cube_indices.peek().is_some() {
                fall_to_idx = cube_indices.next().unwrap() + 1;
            }

            if fall_to_idx >= rows || round_indices.peek().is_none() {
                break;
            }
        }
    }

    map
}

fn calculate_load(map: &[Vec<Cell>]) -> usize {
    let rows = map.len();

    map.iter()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|&c| *c == Cell::RoundRock).count() * (rows - i))
        .sum()
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { map } = p;

    let map = slide_north(map);

    calculate_load(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(p.map.len(), 10);
        assert_eq!(p.map[0].len(), 10);
        assert_eq!(p.map[0][5], Cell::CubeRock);
        assert_eq!(p.map[3][0], Cell::RoundRock);
        assert_eq!(p.map[0][1], Cell::Empty);
    }

    #[test]
    fn test_slide_north() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        let map = slide_north(&p.map);
        assert_eq!(map[0][1], Cell::RoundRock);
        assert_eq!(map[0][4], Cell::Empty);
        assert_eq!(map[1][4], Cell::CubeRock);
        assert_eq!(map[2][4], Cell::RoundRock);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 136);
    }
}
