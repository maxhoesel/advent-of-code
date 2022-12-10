use std::fs::read_to_string;

use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use day9::{moves::Move, rope::Rope};
use log::debug;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let input = read_to_string("input.txt")?;

    let moves: Vec<Move> = input
        .lines()
        .map(|l| Move::try_from(l).map_err(|e| eyre!("Error while parsing moves: {}", e)))
        .collect::<Result<Vec<_>>>()
        .wrap_err("Reading moves")?;

    let mut rope = Rope::new();
    for m in &moves {
        rope.move_head(m);
        debug!("{:?}", rope.segment_positions());
    }
    println!(
        "2-rope Tail: Visited unique Locations: {}",
        rope.visited_count_tail()
    );

    let mut loooong_rope = Rope::with_length(10);
    for m in &moves {
        loooong_rope.move_head(m);
        debug!("Current Segment Locations (Head->...->Tail) {:?}", loooong_rope.segment_positions());
    }
    println!(
        "10-rope Tail: Visited unique Locations: {}",
        loooong_rope.visited_count_tail()
    );

    Ok(())
}
