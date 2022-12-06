use std::fs;

use color_eyre::eyre::{eyre, Context, Result};
use day5::{instructions::parse_instructions, stacks::parse_stacks};
use log::debug;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().wrap_err("Setting up color-eyre")?;
    env_logger::init();

    let input = fs::read_to_string("input.txt").wrap_err("Reading input.txt")?;

    let (stack_input, instructions) = input.split_once("\n\n").ok_or(eyre!(
        "Malformed input, initial stacks and instructions must be separated by empty newline"
    ))?;

    let mut stacks = match parse_stacks(stack_input) {
        Ok((_, stacks)) => stacks,
        Err(e) => return Err(eyre!(e.to_string())),
    };
    let instructions = match parse_instructions(instructions) {
        Ok((_, instructions)) => instructions,
        Err(e) => return Err(eyre!(e.to_string())),
    };

    for inst in instructions {
        debug!("Instruction: {}", inst);
        let mut new_from = stacks.get(inst.from).unwrap().to_owned();
        let mut new_to = stacks.get(inst.to).unwrap().to_owned();
        debug!("Current from and to: {}, {}", new_from, new_to);
        for _ in 0..inst.amount {
            new_to.0.push(new_from.0.pop().unwrap());
        }
        debug!("Updated: {}, {}", new_from, new_to);
        let _ = std::mem::replace(&mut stacks[inst.from], new_from);
        _ = std::mem::replace(&mut stacks[inst.to], new_to);
    }

    println!(
        "Final tops: {}",
        stacks
            .iter()
            .map(|s| s.0.last().unwrap())
            .fold(String::new(), |a, b| a + b.to_string().as_str())
    );

    Ok(())
}
