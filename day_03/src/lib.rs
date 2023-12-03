use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Problem {
    // position, part number, length
    numbers: Vec<((usize, usize), u32, usize)>,
    // position, symbol
    symbols: HashMap<(usize, usize), char>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut p = Problem::default();

        for (x, line) in s.lines().enumerate() {
            let number_slices = find_number_slices(line);
            let numbers = number_slices
                .iter()
                .map(|(y, s)| ((x, *y), s.parse::<u32>().unwrap(), s.len()));

            p.numbers.extend(numbers);

            let symbols = line.char_indices().filter_map(|(y, c)| {
                if c != '.' && !c.is_ascii_digit() {
                    Some(((x, y), c))
                } else {
                    None
                }
            });

            p.symbols.extend(symbols);
        }

        Ok(p)
    }
}

fn find_number_slices(s: &str) -> Vec<(usize, String)> {
    let mut i = 0;
    let mut v = Vec::new();

    while let Some(offset) = s[i..].find(|c: char| c.is_ascii_digit()) {
        let idx = i + offset;

        if let Some(j) = s[idx..].find(|c: char| !c.is_ascii_digit()) {
            let idx_end = idx + j;
            v.push((idx, s[idx..idx_end].to_string()));
            i = idx_end;
        } else {
            v.push((idx, s[idx..].to_string()));
            break;
        }
    }

    v
}

fn neighbour_offsets(i: usize, j: usize) -> &'static [(isize, isize)] {
    match (i, j) {
        (0, 0) => &[(0, 1), (1, 0), (1, 1)],
        (0, _) => &[(0, -1), (0, 1), (1, -1), (1, 0), (1, 1)],
        (_, 0) => &[(-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)],
        (_, _) => &[
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
        ],
    }
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> u32 {
    let Problem { numbers, symbols } = p;

    numbers
        .iter()
        .filter_map(|&((x, y), number, len)| {
            if (y..(y + len))
                .flat_map(|y| {
                    neighbour_offsets(x, y).iter().map(move |&(dx, dy)| {
                        ((x as isize + dx) as usize, (y as isize + dy) as usize)
                    })
                })
                .any(|k| symbols.contains_key(&k))
            {
                Some(number)
            } else {
                None
            }
        })
        .sum()
}

#[must_use]
pub fn solve_part_2(p: &Problem) -> u32 {
    let Problem { numbers, symbols } = p;

    let mut possible_gears = symbols
        .iter()
        .filter_map(|(&pos, &c)| {
            if c == '*' {
                Some((pos, Vec::<u32>::new()))
            } else {
                None
            }
        })
        .collect::<HashMap<(usize, usize), Vec<u32>>>();

    for &((x, y), number, len) in numbers {
        let neighbour_gear_indices = (y..(y + len))
            .flat_map(|y| {
                neighbour_offsets(x, y)
                    .iter()
                    .map(move |&(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
            })
            .filter(|k| possible_gears.contains_key(k))
            .collect::<HashSet<(usize, usize)>>();

        for gear in neighbour_gear_indices {
            possible_gears.entry(gear).and_modify(|nums| {
                nums.push(number);
            });
        }
    }

    possible_gears
        .iter()
        .filter_map(|(_, nums)| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_find_number_slices() {
        let s = "543.*...&231..123";

        assert_eq!(
            find_number_slices(s),
            vec![
                (0, "543".to_string()),
                (9, "231".to_string()),
                (14, "123".to_string())
            ]
        );
    }

    #[test]
    fn test_problem_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(
            p,
            Problem {
                numbers: vec![
                    ((0, 0), 467, 3),
                    ((0, 5), 114, 3),
                    ((2, 2), 35, 2),
                    ((2, 6), 633, 3),
                    ((4, 0), 617, 3),
                    ((5, 7), 58, 2),
                    ((6, 2), 592, 3),
                    ((7, 6), 755, 3),
                    ((9, 1), 664, 3),
                    ((9, 5), 598, 3)
                ],
                symbols: HashMap::from([
                    ((8, 5), '*'),
                    ((1, 3), '*'),
                    ((3, 6), '#'),
                    ((4, 3), '*'),
                    ((5, 5), '+'),
                    ((8, 3), '$')
                ])
            }
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 4361);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_2(&p), 467_835);
    }
}
