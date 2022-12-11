use std::fs::read_to_string;

use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use day11::{monkey::Monkey, parser::monkeys};
use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = read_to_string("input.txt").wrap_err("Reading input.txt")?;

    let (_, mut monkeys) = match monkeys(&input) {
        Ok(m) => m,
        Err(e) => return Err(eyre!("Error while reading monkeys: {}", e)),
    };

    let mut panic_monkeys = monkeys.clone();

    for i in 0..20 {
        println!("\nRound {}", i);
        for m in &monkeys {
            println!("  {}", m);
        }
        monkey_business(&mut monkeys, false)?;
    }

    for m in &monkeys {
        println!(
            "Monkey {} inspected items {} times",
            m.id(),
            m.inspect_count()
        );
    }
    let top_business = monkeys
        .iter()
        .map(|m| m.inspect_count())
        .sorted()
        .rev()
        .take(2)
        .collect_vec();
    println!(
        "Monkey business level after 20 rounds (non-panic): {}",
        top_business[0] * top_business[1]
    );

    for _ in 0..1000 {
        monkey_business(&mut panic_monkeys, true)?;
    }
    let top_business = panic_monkeys
        .iter()
        .map(|m| m.inspect_count())
        .sorted()
        .rev()
        .take(2)
        .collect_vec();
    println!(
        "Monkey business level after 1000 rounds (PANIC): {}",
        top_business[0] * top_business[1]
    );

    Ok(())
}

fn monkey_business(monkeys: &mut [Monkey], panic: bool) -> Result<()> {
    for i in 0..monkeys.len() {
        let mut m = monkeys.get(i).unwrap().clone();
        m.monkey_business(monkeys, panic)
            .map_err(|e| eyre!("Error during Monkey Business: {:?}", e))?;
        let _ = std::mem::replace(&mut monkeys[i], m);
    }
    Ok(())
}
