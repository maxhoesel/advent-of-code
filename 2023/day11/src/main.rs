use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

use anyhow::{anyhow, Result};
use itertools::Itertools;

const GALAXY: char = '#';
const VOID: char = '.';

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}
impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.row.cmp(&other.row) {
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            o => o,
        }
    }
}

#[derive(Debug, Clone)]
struct Universe {
    galaxies: Vec<Pos>,
    width: usize,
    height: usize,
}

impl Universe {
    fn from_grid(input: &str) -> Result<Universe> {
        let mut galaxies = vec![];

        for (row, line) in input.lines().enumerate() {
            for (col, char) in line.char_indices() {
                match char {
                    GALAXY => {
                        galaxies.push(Pos { row, col });
                    }
                    VOID => (),
                    e => return Err(anyhow!("Unknown object in universe: {e}")),
                }
            }
        }
        Ok(Universe {
            width: galaxies.iter().map(|p| p.col).max().unwrap_or(0) + 1,
            height: galaxies.iter().map(|p| p.row).max().unwrap_or(0) + 1,
            galaxies,
        })
    }
    /// Expand the universe by the given `factor`. Said factor must be at least 2, which would imply a doubling
    fn expand(&mut self, factor: usize) {
        let empty_rows = (0..self.height)
            .filter(|row| !self.galaxies.iter().any(|g| g.row == *row))
            .collect_vec();
        let empty_cols = (0..self.width)
            .filter(|col| !self.galaxies.iter().any(|g| g.col == *col))
            .collect_vec();
        // move galaxies to the right and bottom depending on how many empty lines there are above and to the left
        // does not preserve coordinates, but that doesn't really matter because the question didn't specify a reference frame :)
        self.galaxies = self
            .galaxies
            .iter()
            .map(|g| Pos {
                row: g.row + empty_rows.iter().filter(|row| row < &&g.row).count() * (factor - 1),
                col: g.col + empty_cols.iter().filter(|col| col < &&g.col).count() * (factor - 1),
            })
            .collect();
        self.width = self.galaxies.iter().map(|p| p.col).max().unwrap_or(0) + 1;
        self.height = self.galaxies.iter().map(|p| p.row).max().unwrap_or(0) + 1;
    }
    fn galaxy_distance_sum(&self) -> usize {
        self.galaxies
            .iter()
            .cartesian_product(self.galaxies.clone())
            .map(|(a, b)| a.row.abs_diff(b.row) + a.col.abs_diff(b.col))
            .sum::<usize>()
            / 2
    }
}
impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut galaxies = BinaryHeap::from_iter(self.galaxies.iter().map(Reverse));
        let void_row = VOID.to_string().repeat(self.width);

        let mut last_galaxy = &Pos { row: 0, col: 0 };
        // write the first symbol manually
        if let Some(Reverse(first)) = galaxies.peek() {
            if first.col == 0 && first.row == 0 {
                galaxies.pop();
                write!(f, "{GALAXY}")?;
            } else {
                write!(f, "{VOID}")?;
            }
        }
        while let Some(Reverse(current_galaxy)) = galaxies.pop() {
            if current_galaxy.row == last_galaxy.row {
                write!(
                    f,
                    "{}{GALAXY}",
                    VOID.to_string()
                        .repeat(current_galaxy.col - 1 - last_galaxy.col)
                )?;
                last_galaxy = current_galaxy;
            } else {
                // Galaxy on new row
                // 1. fill current row
                writeln!(
                    f,
                    "{}",
                    VOID.to_string().repeat(self.width - 1 - last_galaxy.col)
                )?;
                // 2. insert empty void rows
                let void_rows = current_galaxy.row - 1 - last_galaxy.row;
                for _ in 0..void_rows {
                    writeln!(f, "{}", void_row)?;
                }
                // 3. fill up row up to next galaxy
                write!(f, "{}{GALAXY}", VOID.to_string().repeat(current_galaxy.col))?;
                last_galaxy = current_galaxy;
            }
        }
        // fill in the last row
        writeln!(
            f,
            "{}",
            VOID.to_string().repeat(self.width - 1 - last_galaxy.col)
        )?;
        Ok(())
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let test_universe = Universe::from_grid(TEST)?;
    let input_universe = Universe::from_grid(INPUT)?;

    println!("{}", test_universe);
    let mut test_expand_single = test_universe.clone();
    test_expand_single.expand(2);
    println!("{}", test_expand_single);
    assert_eq!(test_expand_single.galaxy_distance_sum(), 374);

    let mut test_expand_ten = test_universe.clone();
    test_expand_ten.expand(10);
    println!("{}", test_expand_ten);
    assert_eq!(test_expand_ten.galaxy_distance_sum(), 1030);

    let mut test_expand_hundred = test_universe.clone();
    test_expand_hundred.expand(100);
    assert_eq!(test_expand_hundred.galaxy_distance_sum(), 8410);

    let mut input_expand_once = input_universe.clone();
    input_expand_once.expand(2);
    println!(
        "Input expanded once distance sum: {}",
        input_expand_once.galaxy_distance_sum()
    );

    let mut input_expand_million = input_universe.clone();
    input_expand_million.expand(1_000_000);
    println!(
        "Input expanded once distance sum: {}",
        input_expand_million.galaxy_distance_sum()
    );

    Ok(())
}
