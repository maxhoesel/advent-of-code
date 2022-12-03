use std::{collections::HashSet, fs::read_to_string};

use day3::items::{items_from_mask, mask_from_iter, Item};
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

            let mask_0 = mask_from_iter(&items_0);
            let mask_1 = mask_from_iter(&items_1);
            let mask_2 = mask_from_iter(&items_2);

            let mut badge = items_from_mask(mask_0 & mask_1 & mask_2);
            match badge.len() {
                1 => Ok(badge.pop().unwrap().prio as u64),
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
