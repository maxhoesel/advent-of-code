use std::{collections::HashSet, sync::Arc};

use day16::{Position, SparseGrid};
use futures::future::join_all;
use itertools::Itertools;
use maze_walker::{BeamLocation, MazeRunner, Mirror};

use anyhow::{anyhow, Result};
use tracing_subscriber::EnvFilter;

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

const EMPTY: char = '.';

mod maze_walker;

fn fill_grid(input: &str, map: &mut SparseGrid<Mirror>) {
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, elem) in row.char_indices() {
            if elem == EMPTY {
                continue;
            }
            map.put(
                Position {
                    row: row_idx,
                    col: col_idx,
                },
                Mirror::try_from(elem).expect("Not a mirror segment!"),
            );
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut test_grid = SparseGrid::new();
    fill_grid(TEST, &mut test_grid);
    println!("{}", test_grid);

    let test_grid = Arc::new(test_grid);
    let test_walker = MazeRunner::new(
        Arc::clone(&test_grid),
        BeamLocation {
            position: Position { row: 0, col: 0 },
            direction: day16::Direction::Right,
        },
    );
    let res = test_walker.results().await;
    println!(
        "Test count: {}",
        res.visited_fields
            .iter()
            .map(|v| v.position)
            .collect::<HashSet<_>>()
            .len()
    );
    let mut walked_grid = SparseGrid::new();
    for r in res.visited_fields {
        walked_grid.put(r.position, "#");
    }
    println!("{}", walked_grid);

    let mut possible_spawns = Vec::with_capacity(test_grid.height() * 2 + test_grid.width() * 2);
    possible_spawns.extend((0..test_grid.height()).map(|row| BeamLocation {
        position: Position { row, col: 0 },
        direction: day16::Direction::Right,
    }));
    possible_spawns.extend((0..test_grid.height()).map(|row| BeamLocation {
        position: Position {
            row,
            col: test_grid.width() - 1,
        },
        direction: day16::Direction::Left,
    }));
    possible_spawns.extend((0..test_grid.width()).map(|col| BeamLocation {
        position: Position { row: 0, col },
        direction: day16::Direction::Down,
    }));
    possible_spawns.extend((0..test_grid.width()).map(|col| BeamLocation {
        position: Position {
            row: test_grid.height() - 1,
            col,
        },
        direction: day16::Direction::Up,
    }));
    let best = possible_spawns
        .iter()
        .map(|start| {
            let test_walker = MazeRunner::new(Arc::clone(&test_grid), *start);
            tokio::spawn(async move { test_walker.results().await })
        })
        .collect_vec();
    let max_test = join_all(best).await;
    let max_test = max_test
        .iter()
        .map(|e| e.as_ref().map_err(|e| anyhow!("whatever: {e}")))
        .collect::<Result<Vec<_>>>()
        .unwrap();
    let max_test = max_test
        .iter()
        .max_by_key(|res| res.visited_fields.len())
        .unwrap();

    println!(
        "Test max possible {:?}",
        max_test
            .visited_fields
            .iter()
            .map(|v| v.position)
            .collect::<HashSet<_>>()
            .len()
    );

    let mut input_grid = SparseGrid::new();
    fill_grid(INPUT, &mut input_grid);
    //println!("{}", input_grid);
    let input_grid = Arc::new(input_grid);
    let input_walker = MazeRunner::new(
        Arc::clone(&input_grid),
        BeamLocation {
            position: Position { row: 0, col: 0 },
            direction: day16::Direction::Right,
        },
    );

    let res = input_walker.results().await;
    println!(
        "Input count: {}",
        res.visited_fields
            .iter()
            .map(|v| v.position)
            .collect::<HashSet<_>>()
            .len()
    );
    let mut walked_grid = SparseGrid::new();
    for r in res.visited_fields {
        walked_grid.put(r.position, "#");
    }
    //println!("{}", walked_grid);

    let mut possible_spawns = Vec::with_capacity(input_grid.height() * 2 + input_grid.width() * 2);
    possible_spawns.extend((0..input_grid.height()).map(|row| BeamLocation {
        position: Position { row, col: 0 },
        direction: day16::Direction::Right,
    }));
    possible_spawns.extend((0..input_grid.height()).map(|row| BeamLocation {
        position: Position {
            row,
            col: input_grid.width() - 1,
        },
        direction: day16::Direction::Left,
    }));
    possible_spawns.extend((0..input_grid.width()).map(|col| BeamLocation {
        position: Position { row: 0, col },
        direction: day16::Direction::Down,
    }));
    possible_spawns.extend((0..input_grid.width()).map(|col| BeamLocation {
        position: Position {
            row: input_grid.height() - 1,
            col,
        },
        direction: day16::Direction::Up,
    }));
    let best = possible_spawns
        .iter()
        .map(|start| {
            let input_walker = MazeRunner::new(Arc::clone(&input_grid), *start);
            tokio::spawn(async move { input_walker.results().await })
        })
        .collect_vec();
    let max_input = join_all(best)
        .await
        .iter()
        .map(|e| e.as_ref().map_err(|e| anyhow!("whatever: {e}")))
        .collect::<Result<Vec<_>>>()
        .unwrap()
        .iter()
        .map(|r| {
            r.visited_fields
                .iter()
                .map(|f| f.position)
                .collect::<HashSet<_>>()
        })
        .max_by_key(|res| res.len())
        .map(|r| r.len())
        .unwrap();

    println!("Input max possible {}", max_input);
}
