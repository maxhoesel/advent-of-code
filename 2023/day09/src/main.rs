use itertools::Itertools;

use anyhow::{anyhow, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn predict_value(input: &[i64]) -> Result<i64> {
    if input.len() < 2 {
        Err(anyhow!("Reading has not converged in time!"))
    } else {
        let lower_line = input.windows(2).map(|win| win[1] - win[0]).collect_vec();
        if input.iter().all(|e| *e == 0) {
            Ok(0)
        } else {
            Ok(input.last().unwrap() + predict_value(&lower_line)?)
        }
    }
}

fn read_measurements_from_str(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|num| num.parse::<i64>().map_err(|e| anyhow!("{e}")))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()
}

#[tokio::main]
async fn main() -> Result<()> {
    let measurements_test = read_measurements_from_str(TEST)?;
    let predictions_test = measurements_test
        .par_iter()
        .map(|measurement| predict_value(measurement))
        .collect::<Result<Vec<_>>>()?;
    println!("{:?}", predictions_test);
    println!("{}", predictions_test.iter().sum::<i64>());

    let measurements_input = read_measurements_from_str(INPUT)?;
    let predictions_input = measurements_input
        .par_iter()
        .map(|measurement| predict_value(measurement))
        .collect::<Result<Vec<_>>>()?;
    println!("{}", predictions_input.iter().sum::<i64>());

    let measurements_test_rev = measurements_test
        .iter()
        .map(|ms| ms.iter().rev().cloned().collect_vec())
        .collect_vec();
    let hist_test = measurements_test_rev
        .par_iter()
        .map(|measurement| predict_value(measurement))
        .collect::<Result<Vec<_>>>()?;
    println!("{:?}", hist_test);
    println!("{}", hist_test.iter().sum::<i64>());

    let measurements_input_rev = measurements_input
        .iter()
        .map(|ms| ms.iter().rev().cloned().collect_vec())
        .collect_vec();
    let hist_input = measurements_input_rev
        .par_iter()
        .map(|measurement| predict_value(measurement))
        .collect::<Result<Vec<_>>>()?;
    println!("{}", hist_input.iter().sum::<i64>());

    Ok(())
}
