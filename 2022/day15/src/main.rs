use std::{collections::HashSet, fmt::Display};

use color_eyre::{eyre::eyre, Result};
use day15::{
    grid::{self, Grid, GridCoord, SparseDefaultGrid},
    sensors::Sensor,
};
use itertools::Itertools;
use log::debug;
use rayon::prelude::*;

const ROW_TO_CHECK: isize = 2000000;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
enum GridElement {
    Sensor,
    Beacon,
    Nothing,
}
impl Default for GridElement {
    fn default() -> Self {
        GridElement::Nothing
    }
}
impl Display for GridElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridElement::Sensor => "S",
                GridElement::Beacon => "B",
                GridElement::Nothing => ".",
            }
        )
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    let input = include_str!("input.txt");

    let sensors = input
        .lines()
        .map(|l| {
            Sensor::from_line(l)
                .map_err(|e| eyre!("Error while parsing sensor input: {}", e))
                .unwrap()
        })
        .collect_vec();

    let mut grid: SparseDefaultGrid<GridElement> = SparseDefaultGrid::new();
    for s in &sensors {
        debug!("Inserting sensor at {}", s.pos);
        grid.set(s.pos, GridElement::Sensor);
        debug!("Inserting beacon at {}", s.nearest_beacon);
        grid.set(s.nearest_beacon, GridElement::Beacon);
    }
    if grid.width() < 100 && grid.height() < 100 {
        println!("Sensor Grid:\n{}", grid);
    }

    let beacon_free_cells = sensors
        .par_iter()
        .filter_map(|s| {
            s.coverage_by_row(ROW_TO_CHECK).map(|r| {
                r.into_par_iter()
                    .filter_map(|x| {
                        let c = GridCoord { x, y: ROW_TO_CHECK };
                        match grid.at(&c) {
                            Some(GridElement::Beacon) => None,
                            Some(_) => Some(c),
                            None => Some(c),
                        }
                    })
                    .collect::<Vec<_>>()
            })
        })
        .flatten()
        .collect::<HashSet<_>>();

    println!(
        "Coverage for Row {ROW_TO_CHECK}: {}",
        beacon_free_cells.len(),
    );

    Ok(())
}
