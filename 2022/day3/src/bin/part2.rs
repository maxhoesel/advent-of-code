use std::{collections::HashSet, fs::read_to_string};

use day3::items::Item;
use rayon::prelude::*;

use color_eyre::{eyre::eyre, Result};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let input = read_to_string("input.txt")?;
    let lines: Vec<_> = input.par_lines().collect();

    let prio_sum: Result<u64, _> = lines
        .par_chunks(3)
        .map(|group| {
            let items_0: HashSet<Item> = group[0].par_chars().map(|c| c.into()).collect();
            let items_1: HashSet<Item> = group[1].par_chars().map(|c| c.into()).collect();
            let items_2: HashSet<Item> = group[2].par_chars().map(|c| c.into()).collect();

            let mut combined = &(&items_0 & &items_1) & &items_2;

            match combined.len() {
                1 => Ok(combined.drain().next().unwrap().prio as u64),
                2.. => Err(eyre!("More than 1 item shared across the group!")),
                _ => Err(eyre!("No shared item found for group!")),
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
