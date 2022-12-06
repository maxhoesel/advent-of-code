use std::{collections::HashSet, fs::read_to_string, hash::Hash};

const WINDOW_SIZE: usize = 14;

use color_eyre::eyre::{eyre, Context, Result};
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().wrap_err("Setting up color-eyre")?;
    env_logger::init();

    let input = read_to_string("input.txt")?;

    let offset = input
        .chars()
        .collect::<Vec<char>>()
        .windows(WINDOW_SIZE)
        .enumerate()
        .find_map(|(offset, window)| {
            if window.iter().collect::<HashSet<&char>>().len() == WINDOW_SIZE {
                Some(offset + WINDOW_SIZE)
            } else {
                None
            }
        })
        .ok_or(eyre!("No marker found"))?;

    println!("Offset: {}", offset);

    Ok(())
}
