use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Ground,
    Start,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'|' => Ok(Self::NorthSouth),
            b'-' => Ok(Self::EastWest),
            b'L' => Ok(Self::NorthEast),
            b'J' => Ok(Self::NorthWest),
            b'7' => Ok(Self::SouthWest),
            b'F' => Ok(Self::SouthEast),
            b'.' => Ok(Self::Ground),
            b'S' => Ok(Self::Start),
            c => bail!("couldn't parse Tile from {:?}", c),
        }
    }
}

#[derive(Debug)]
pub struct Problem {
    map: Vec<Vec<Tile>>,
    start_pos: (usize, usize),
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .flat_map(|l| {
                l.bytes()
                    .map(std::convert::TryInto::try_into)
                    .collect::<Result<_, _>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        let start_pos = map
            .iter()
            .enumerate()
            .find_map(|(i, v)| v.iter().position(|t| *t == Tile::Start).map(|j| (i, j)))
            .ok_or_else(|| anyhow!("couldn't find starting position"))?;

        Ok(Self { map, start_pos })
    }
}

// max_x, max_y are typically `arr.len() - 1`, `arr[0].len() - 1`
fn neighbour_indices_8dir(
    (x, y): (usize, usize),
    (max_x, max_y): (usize, usize),
) -> Vec<(usize, usize)> {
    match (
        x.checked_sub(1),
        (x < max_x).then_some(x + 1),
        y.checked_sub(1),
        (y < max_y).then_some(y + 1),
    ) {
        (None, Some(xp), None, Some(yp)) => vec![(x, yp), (xp, y), (xp, yp)],
        (None, Some(xp), Some(ym), None) => vec![(x, ym), (xp, y), (xp, ym)],
        (None, Some(xp), Some(ym), Some(yp)) => vec![(x, ym), (x, yp), (xp, y), (xp, ym), (xp, yp)],
        (Some(xm), None, None, Some(yp)) => vec![(x, yp), (xm, y), (xm, yp)],
        (Some(xm), None, Some(ym), None) => vec![(x, ym), (xm, y), (xm, ym)],
        (Some(xm), None, Some(ym), Some(yp)) => vec![(x, ym), (x, yp), (xm, y), (xm, ym), (xm, yp)],
        (Some(xm), Some(xp), None, Some(yp)) => vec![(x, yp), (xm, y), (xp, y), (xm, yp), (xp, yp)],
        (Some(xm), Some(xp), Some(ym), None) => vec![(x, ym), (xm, y), (xp, y), (xm, ym), (xp, ym)],
        (Some(xm), Some(xp), Some(ym), Some(yp)) => vec![
            (x, ym),
            (x, yp),
            (xm, y),
            (xp, y),
            (xm, ym),
            (xm, yp),
            (xp, ym),
            (xp, yp),
        ],
        // all unreachable unless we are on a 1-wide/1-tall map (spoiler: we are not)
        (None | Some(_), None | Some(_), None, None)
        | (None, None, None | Some(_), None | Some(_)) => unreachable!(),
    }
}

// max_x, max_y are typically `arr.len() - 1`, `arr[0].len() - 1`
fn neighbour_indices_4dir(
    (x, y): (usize, usize),
    (max_x, max_y): (usize, usize),
) -> Vec<(usize, usize)> {
    match (
        x.checked_sub(1),
        (x < max_x).then_some(x + 1),
        y.checked_sub(1),
        (y < max_y).then_some(y + 1),
    ) {
        (None, Some(xp), None, Some(yp)) => vec![(x, yp), (xp, y)],
        (None, Some(xp), Some(ym), None) => vec![(x, ym), (xp, y)],
        (None, Some(xp), Some(ym), Some(yp)) => vec![(x, ym), (x, yp), (xp, y)],
        (Some(xm), None, None, Some(yp)) => vec![(x, yp), (xm, y)],
        (Some(xm), None, Some(ym), None) => vec![(x, ym), (xm, y)],
        (Some(xm), None, Some(ym), Some(yp)) => vec![(x, ym), (x, yp), (xm, y)],
        (Some(xm), Some(xp), None, Some(yp)) => vec![(x, yp), (xm, y), (xp, y)],
        (Some(xm), Some(xp), Some(ym), None) => vec![(x, ym), (xm, y), (xp, y)],
        (Some(xm), Some(xp), Some(ym), Some(yp)) => vec![(x, ym), (x, yp), (xm, y), (xp, y)],
        // all unreachable unless we are on a 1-wide/1-tall map (spoiler: we are not)
        (None | Some(_), None | Some(_), None, None)
        | (None, None, None | Some(_), None | Some(_)) => unreachable!(),
    }
}

fn get_exit_pos(
    entry_pos: (usize, usize),
    tile_pos: (usize, usize),
    tile: &Tile,
) -> (usize, usize) {
    match tile {
        Tile::NorthSouth => {
            if entry_pos.0 + 1 == tile_pos.0 {
                (tile_pos.0 + 1, tile_pos.1)
            } else {
                (tile_pos.0 - 1, tile_pos.1)
            }
        }
        Tile::EastWest => {
            if entry_pos.1 + 1 == tile_pos.1 {
                (tile_pos.0, tile_pos.1 + 1)
            } else {
                (tile_pos.0, tile_pos.1 - 1)
            }
        }
        Tile::NorthEast => {
            if entry_pos.0 == tile_pos.0 {
                (tile_pos.0 - 1, tile_pos.1)
            } else {
                (tile_pos.0, tile_pos.1 + 1)
            }
        }
        Tile::NorthWest => {
            if entry_pos.0 == tile_pos.0 {
                (tile_pos.0 - 1, tile_pos.1)
            } else {
                (tile_pos.0, tile_pos.1 - 1)
            }
        }
        Tile::SouthWest => {
            if entry_pos.0 == tile_pos.0 {
                (tile_pos.0 + 1, tile_pos.1)
            } else {
                (tile_pos.0, tile_pos.1 - 1)
            }
        }
        Tile::SouthEast => {
            if entry_pos.0 == tile_pos.0 {
                (tile_pos.0 + 1, tile_pos.1)
            } else {
                (tile_pos.0, tile_pos.1 + 1)
            }
        }
        // we never enter ground tiles, and we stop before we loop back to start tile
        Tile::Ground | Tile::Start => unreachable!(),
    }
}

fn get_start_pipes(
    map: &Vec<Vec<Tile>>,
    start_pos: (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    let start_pipe_candidates =
        neighbour_indices_4dir(start_pos, (map.len() - 1, map[0].len() - 1));

    let (x, y) = start_pos;

    let start_pipes = start_pipe_candidates
        .into_iter()
        .filter_map(|(nx, ny)| match map[nx][ny] {
            Tile::NorthSouth | Tile::NorthEast | Tile::NorthWest if nx == x + 1 => Some((nx, ny)),
            Tile::NorthSouth | Tile::SouthEast | Tile::SouthWest
                if x.checked_sub(1).is_some_and(|xm| nx == xm) =>
            {
                Some((nx, ny))
            }
            Tile::EastWest | Tile::NorthWest | Tile::SouthWest if ny == y + 1 => Some((nx, ny)),
            Tile::EastWest | Tile::NorthEast | Tile::SouthEast
                if y.checked_sub(1).is_some_and(|ym| ny == ym) =>
            {
                Some((nx, ny))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    debug_assert!(start_pipes.len() == 2);

    (start_pipes[0], start_pipes[1])
}

// 1. Go both ways and count steps
// 2. When you end up on same tile, return count of steps

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { map, start_pos } = p;

    let (mut first_pipe_pos, mut second_pipe_pos) = get_start_pipes(map, *start_pos);
    let mut first_pipe_entry = *start_pos;
    let mut second_pipe_entry = *start_pos;

    let mut result = 1;
    while first_pipe_pos != second_pipe_pos {
        let temp1 = first_pipe_pos;
        first_pipe_pos = get_exit_pos(
            first_pipe_entry,
            first_pipe_pos,
            &map[first_pipe_pos.0][first_pipe_pos.1],
        );
        first_pipe_entry = temp1;

        let temp2 = second_pipe_pos;
        second_pipe_pos = get_exit_pos(
            second_pipe_entry,
            second_pipe_pos,
            &map[second_pipe_pos.0][second_pipe_pos.1],
        );
        second_pipe_entry = temp2;

        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const TEST_INPUT_2: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(p.start_pos, (1, 1));
        assert_eq!(p.map.len(), 5);
        assert_eq!(p.map[0].len(), 5);
        assert_eq!(p.map[0][3], Tile::SouthEast);
        assert_eq!(p.map[3][1], Tile::NorthEast);
    }

    #[test]
    fn test_solve_part_1() {
        let p1: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p1), 4);
        let p2: Problem = TEST_INPUT_2.parse().unwrap();
        assert_eq!(solve_part_1(&p2), 8);
    }
}
