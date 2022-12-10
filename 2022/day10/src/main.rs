use std::fs::read_to_string;

use color_eyre::{eyre::eyre, eyre::Context, Result};
use day10::{crt::Crt, interpreter::Interpreter, parser::command_list};
use log::{debug, info};

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = read_to_string("input.txt")
        .wrap_err("Reading input.txt")?
        .to_lowercase();

    let cmds = match command_list(&input) {
        Ok(cmds) => cmds,
        Err(e) => return Err(eyre!("Error while parsing commands: {}", e)),
    };

    let mut interpreter = Interpreter::with_input(cmds.as_slice());
    let mut crt = Crt::new();

    println!("Initial CRT Screen:\n{}", crt);

    let signal_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut total_strenghts = 0;
    let mut clock: i32 = 1;
    loop {
        debug!("Cycle {}, X: {}", clock, interpreter.x());
        if signal_cycles.contains(&clock) {
            let strength = interpreter.x() * clock;
            info!("Signal Strength at Cycle {}: {}", clock, strength);
            total_strenghts += strength;
        }
        crt.tick(interpreter.x());
        let done = interpreter.tick();
        if done {
            break;
        }
        clock += 1;
    }

    println!("Combined Signal Strength: {}", total_strenghts);
    println!("Final CRT Screen:\n{}", crt);
    Ok(())
}
