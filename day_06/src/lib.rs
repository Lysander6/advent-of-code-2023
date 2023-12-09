use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct Problem {
    records: Vec<(f64, f64)>,
}

impl FromStr for Problem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().flat_map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(str::parse::<f64>)
                .collect::<Result<Vec<_>, _>>()
        });

        let times = lines.next().ok_or_else(|| anyhow!("missing times"))?;
        let distances = lines.next().ok_or_else(|| anyhow!("missing distances"))?;
        let records = times.into_iter().zip(distances).collect();

        Ok(Self { records })
    }
}

/// T - race time
/// t - time the button was held
/// D - record distance
/// d - distance travelled
///
/// d = t(T - t)
///
/// ways to beat record
/// t(T - t) > D
///
/// t(T - t) - D > 0
/// -t^2 + Tt - D > 0
/// (so a=-1, b=T, c=-D in common nomenclature)
/// Δ = T^2 - 4D
/// (-T + sqrt(T^2 - 4(-1)(-D))) / 2(-1) < t < (-T - sqrt(T^2 - 4(-1)(-D))) / 2(-1)
/// (T - sqrt(Δ)) / 2 < t < (T + sqrt(Δ)) / 2
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
pub fn count_ways_to_win(race_time: f64, record_distance: f64) -> usize {
    let t = race_time;
    let d = record_distance;

    let delta = t * t - 4.0 * d;
    let left = (t - delta.sqrt()) / 2.0;
    let right = (t + delta.sqrt()) / 2.0;
    // Find first (/last) whole number strictly greater (/less) than found root(s), keeping in
    // mind the case where it already falls at exact whole number (last example case graciously
    // provided by the puzzle creator). Then, having that inclusive range, calculate number of
    // whole numbers inside it (abs diff between them + 1).
    let result = ((left + 1.0).floor() - (right - 1.0).ceil()).abs() + 1.0;

    result.floor() as usize
}

#[must_use]
pub fn solve_part_1(p: &Problem) -> usize {
    let Problem { records } = p;

    records
        .iter()
        .map(|&(race_time, record_distance)| count_ways_to_win(race_time, record_distance))
        .product()
}

/// # Errors
///
/// This function will return an error if either race times or distance records can't be merged
/// into a single value.
pub fn solve_part_2(p: &Problem) -> Result<usize, anyhow::Error> {
    let Problem { records } = p;
    let (race_times, distance_records): (Vec<_>, Vec<_>) = records.iter().copied().unzip();

    let merge_numbers = |v: &Vec<f64>| -> Result<f64, anyhow::Error> {
        let m = v
            .iter()
            .map(std::string::ToString::to_string)
            .reduce(|acc, s| acc + &s)
            .ok_or_else(|| anyhow!("couldn't parse merged string"))?
            .parse()?;

        Ok(m)
    };

    let race_time = merge_numbers(&race_times)?;
    let record_distance = merge_numbers(&distance_records)?;

    Ok(count_ways_to_win(race_time, record_distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_input_parsing() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(
            p,
            Problem {
                records: vec![(7.0, 9.0), (15.0, 40.0), (30.0, 200.0)]
            }
        );
    }

    #[test]
    fn test_count_ways_to_win() {
        assert_eq!(count_ways_to_win(7.0, 9.0), 4);
        assert_eq!(count_ways_to_win(15.0, 40.0), 8);
        assert_eq!(count_ways_to_win(30.0, 200.0), 9);
    }

    #[test]
    fn test_solve_part_1() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_1(&p), 288);
    }

    #[test]
    fn test_solve_part_2() {
        let p: Problem = TEST_INPUT.parse().unwrap();
        assert_eq!(solve_part_2(&p).unwrap(), 71503);
    }
}
