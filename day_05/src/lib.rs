use std::{ops::Range, str::FromStr};

use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    src_range: Range<usize>,
    dst_range: Range<usize>,
    length: usize,
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [dst_range_start, src_range_start, length]: [usize; 3] = s
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?[..3]
            .try_into()?;

        Ok(Self {
            src_range: (src_range_start..(src_range_start + length)),
            dst_range: (dst_range_start..(dst_range_start + length)),
            length,
        })
    }
}

#[derive(Debug)]
pub struct Problem {
    seeds: Vec<usize>,
    mappings: Vec<Vec<Mapping>>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");

        let seeds = parts.next().ok_or_else(|| anyhow!("no seeds"))?;
        let seeds = seeds
            .strip_prefix("seeds: ")
            .ok_or_else(|| anyhow!("unexpected seeds prefix"))?;
        let seeds = seeds
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        let mappings = parts
            .map(|mapping_description| {
                let (_, mappings) = mapping_description.split_once(":\n").ok_or_else(|| {
                    anyhow!(
                        "couldn't split mapping description: {:?}",
                        mapping_description
                    )
                })?;

                mappings
                    .lines()
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Problem { seeds, mappings })
    }
}

/// # Errors
///
/// Returns error when `seeds` vector is empty and therefore minimal location can't be found.
pub fn solve_part_1(p: &Problem) -> Result<usize, anyhow::Error> {
    let Problem { seeds, mappings } = p;
    let mut locations = seeds.clone();

    for mapping in mappings {
        for location in &mut locations {
            if let Some(map) = mapping.iter().find(|a| a.src_range.contains(location)) {
                *location -= map.src_range.start;
                *location += map.dst_range.start;
            }
        }
    }

    locations
        .iter()
        .min()
        .copied()
        .ok_or_else(|| anyhow!("couldn't find min"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_mapping_parsing() {
        assert_eq!(
            "50 98 2".parse::<Mapping>().unwrap(),
            Mapping {
                src_range: (98..100),
                dst_range: (50..52),
                length: 2
            }
        );
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p).unwrap(), 35);
    }
}
