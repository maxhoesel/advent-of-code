use color_eyre::{
    eyre::{eyre, Context},
    Result,
};

use day4::range::{range_from_str, RangeInclusiveExt};
use log::debug;
use rayon::{prelude::ParallelIterator, str::ParallelString};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().wrap_err("Setting up color-eyre")?;
    env_logger::init();

    let input = fs::read_to_string("input.txt").wrap_err("Reading input.txt")?;

    let result: Result<u32, _> = input
        .par_lines()
        .map(|line| {
            let Some((left, right)) = line.split_once(',') else {
                return Err(eyre!("Invalid line: does not contain two comma-separated ranges: {}", line));
            };
            let left_range = range_from_str(left).wrap_err(format!("generating range for {}", left))?;
            let right_range = range_from_str(right).wrap_err(format!("generating range for {}", right))?;

            let contained = left_range.contains_range(&right_range) || right_range.contains_range(&left_range);

            debug!("Left: {:?}, Right: {:?}, Contained: {}", left_range, right_range, contained);

            Ok(contained as u32)
        })
        .sum();

    match result {
        Ok(count) => println!("Total number of contained ranges: {}", count),
        Err(e) => return Err(e),
    }
    Ok(())
}
