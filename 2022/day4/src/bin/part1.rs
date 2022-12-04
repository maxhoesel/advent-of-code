use color_eyre::{eyre::eyre, Report, Result};
use day4::range::range_tuple;
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
            let l_range = range_tuple(left)?;
            let r_range = range_tuple(right)?;

            let contained = (l_range.0 - r_range.0 >= 0 && l_range.1 - r_range.1 <= 0) || (l_range.0 - r_range.0 <= 0 && l_range.1 - r_range.1 >= 0);

            debug!("Left: {:?}, Right: {:?}, Contained: {}", l_range, r_range, contained);

            Ok(contained as u32)
        })
        .sum();

    match result {
        Ok(count) => println!("Total number of contained ranges: {}", count),
        Err(e) => return Err(e),
    }
    Ok(())
}
