use color_eyre::{eyre::eyre, Report, Result};
use day4::parse::range_from_str;
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
            let l_range = range_from_str(left)?;
            let r_range = range_from_str(right)?;

            let no_intersect = l_range.1 < r_range.0 || r_range.1 < l_range.0;

            debug!("Left: {:?}, Right: {:?}, Intersect: {}", l_range, r_range, !no_intersect);

            Ok(!no_intersect as u32)
        })
        .sum();

    match result {
        Ok(count) => println!("Total number of overlapping ranges: {}", count),
        Err(e) => return Err(e),
    }
    Ok(())
}
