use std::{fmt::Display, fs::read_to_string};

use color_eyre::{eyre::Context, Result};
use day17::{
    grid::{DefaultGrid, GridCoord, SparseDefaultGrid},
    rock::{Direction, Rock, RockBuilder},
};
use itertools::Itertools;
use log::{debug, info};

const NUM_ROCKS: usize = 2022;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
enum Element {
    Empty,
    Filled,
}
impl Default for Element {
    fn default() -> Self {
        Element::Empty
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Empty => ".",
                Element::Filled => "#",
            }
        )
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let jets = read_to_string("input.txt")
        .wrap_err("Reading input.txt")?
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            e => panic!("Invalid jetstream input {e}"),
        })
        .collect_vec();

    let mut grid: SparseDefaultGrid<Element> = SparseDefaultGrid::new(day17::grid::Origin::BotLeft);
    let mut dropper = RockBuilder::new();

    let mut jet_counter = 0;
    let mut top = 0;
    for i in 0..NUM_ROCKS {
        let fall_pos = GridCoord {
            x: 3,
            y: grid.y_max() + 4,
        };
        let mut rock = dropper.drop_at_pos(&fall_pos);
        loop {
            let direction = jets[jet_counter % jets.len()];

            debug!(
                "Rock {}, iteration {}, pushing {:?}",
                i, jet_counter, direction
            );

            jet_counter += 1;

            rock.push(direction);
            if rock.collides(&grid) || rock.left() <= 0 || rock.right() > 7 {
                rock.push_back(direction)
            }

            rock.push(Direction::Down);
            if rock.collides(&grid) || rock.bot() == 0 {
                // Rests ontop of another rock or the ground
                rock.push(Direction::Up);
                insert_rock_into_grid(&rock, &mut grid);
                info!("{}", &grid);
                break;
            }
        }
    }

    println!("Height: {}", grid.y_max());

    Ok(())
}

fn insert_rock_into_grid(rock: &Rock, grid: &mut dyn DefaultGrid<Element>) {
    for b in rock.bits() {
        grid.set(b, Element::Filled);
    }
}
