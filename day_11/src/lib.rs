use std::str::FromStr;

#[derive(Debug)]
pub struct Problem {
    map: Vec<Vec<bool>>,
    galaxies: Vec<(usize, usize)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();

        let galaxies = map
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                v.iter()
                    .enumerate()
                    .filter_map(move |(j, g)| g.then_some((i, j)))
            })
            .collect();

        Ok(Self { map, galaxies })
    }
}

fn expand_space(p: &Problem) -> Vec<(usize, usize)> {
    let Problem { map, galaxies } = p;
    let mut galaxies = galaxies.clone();

    for i in (0..map.len()).rev() {
        if map[i].iter().all(|g| !g) {
            for (x, _) in &mut galaxies {
                if *x > i {
                    *x += 1;
                }
            }
        }
    }

    for j in (0..map[0].len()).rev() {
        if (0..map.len()).map(|i| map[i][j]).all(|g| !g) {
            for (_, y) in &mut galaxies {
                if *y > j {
                    *y += 1;
                }
            }
        }
    }

    galaxies
}

fn dist((a_x, a_y): &(usize, usize), (b_x, b_y): &(usize, usize)) -> usize {
    a_x.abs_diff(*b_x) + a_y.abs_diff(*b_y)
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let expanded_galaxies = expand_space(p);

    let mut result = 0;

    for (i, a) in expanded_galaxies.iter().enumerate() {
        for b in &expanded_galaxies[(i + 1)..] {
            result += dist(a, b);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(p.map.len(), 10);
        assert_eq!(p.map[0].len(), 10);
        assert_eq!(p.galaxies.len(), 9);
        assert!(!p.map[0][0]);
        assert!(p.map[0][3]);
        assert_eq!(p.galaxies[0], (0, 3));
        assert_eq!(p.galaxies[8], (9, 4));
    }

    #[test]
    fn test_expand_space() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        let expanded_galaxies = expand_space(&p);
        assert_eq!(
            expanded_galaxies,
            vec![
                (0, 4),
                (1, 9),
                (2, 0),
                (5, 8),
                (6, 1),
                (7, 12),
                (10, 9),
                (11, 0),
                (11, 5)
            ]
        );
    }

    #[test]
    fn test_dist() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        let expanded_galaxies = expand_space(&p);
        assert_eq!(dist(&expanded_galaxies[4], &expanded_galaxies[8]), 9);
        assert_eq!(dist(&expanded_galaxies[0], &expanded_galaxies[6]), 15);
        assert_eq!(dist(&expanded_galaxies[2], &expanded_galaxies[5]), 17);
        assert_eq!(dist(&expanded_galaxies[7], &expanded_galaxies[8]), 5);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 374);
    }
}
