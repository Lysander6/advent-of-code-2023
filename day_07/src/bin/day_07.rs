use std::{env, fs};

use anyhow::Context;
use day_07::{solve, Problem};

fn main() -> Result<(), anyhow::Error> {
    let input_path = env::args().nth(1).context("missing path argument")?;
    let content = fs::read_to_string(input_path)?;
    let mut p: Problem = content.parse()?;

    let sum = solve(&mut p);
    println!("Part 2: {sum}");

    Ok(())
}
