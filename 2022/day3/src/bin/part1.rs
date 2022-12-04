use std::{collections::HashSet, fs::read_to_string};

use day3::items::Item;
use rayon::prelude::*;

use color_eyre::{eyre::eyre, Result};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let input = read_to_string("input.txt")?;

    let prio_sum: Result<u64, _> = input
        .par_lines()
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            let items_l: HashSet<Item> = left.par_chars().map(|c| c.into()).collect();
            let items_r: HashSet<Item> = right.par_chars().map(|c| c.into()).collect();

            let mut in_both = &items_l & &items_r;

            match in_both.len() {
                1 => Ok(in_both.drain().next().unwrap().prio as u64),
                2.. => Err(eyre!("More than 1 duplicate item in compartment!")),
                _ => Err(eyre!("No duplicate item in compartment!")),
            }
        })
        .sum();
    match prio_sum {
        Ok(s) => {
            println!("Sum of all item priorities: {}", s);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
