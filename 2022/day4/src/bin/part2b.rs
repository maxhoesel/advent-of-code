use color_eyre::{eyre::eyre, Report, Result};
use day4::range::{range_from_str, RangeInclusiveExt};
use log::debug;
use rayon::{prelude::ParallelIterator, str::ParallelString};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = fs::read_to_string("input.txt")?;

    let result: Result<u32, _> = input
        .par_lines()
        .map(|line| {
            let Some((left, right)) = line.split_once(',') else {
                return Err(eyre!("Invalid line: does not contain two comma-separated ranges: {}", line));
            };
            let left_range = range_from_str(left)?;
            let right_range = range_from_str(right)?;

            let intersect = left_range.intersects_range(&right_range) || right_range.intersects_range(&left_range);

            debug!("Left: {:?}, Right: {:?}, Intersect: {}", left_range, right_range, intersect);

            Ok(intersect as u32)
        })
        .sum();

    match result {
        Ok(count) => println!("Total number of overlapping ranges: {}", count),
        Err(e) => return Err(e),
    }
    Ok(())
}
