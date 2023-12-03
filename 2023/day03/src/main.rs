use std::{rc::Rc, slice::Windows};

use anyhow::{anyhow, Result};
use itertools::Itertools;

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");
const EMPTY_CHAR: char = '.';
const GEAR_CHAR: char = '*';

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Schematic {
    all_numbers: Vec<Number>,
    grid: Rc<Grid>,
    gear_symbols: Vec<GearSymbol>,
}
impl Schematic {
    fn from_str(input: &str) -> Result<Schematic> {
        let height = input.lines().count();
        let width = input
            .lines()
            .next()
            .ok_or(anyhow!("Schematic is empty"))?
            .len();
        let mut grid = Grid::new(width, height);
        let mut numbers = vec![];
        let mut gear_symbols = vec![];

        for (row, line) in input.lines().enumerate() {
            for (col, entry) in line.char_indices() {
                // fill in the grid
                grid.inner[row][col] = {
                    if entry == EMPTY_CHAR {
                        GridEntry::Empty
                    } else if let Some(digit) = entry.to_digit(10) {
                        GridEntry::NumberDigit(digit)
                    } else if entry == GEAR_CHAR {
                        gear_symbols.push(GearSymbol {
                            pos: Pos { row, col },
                        });
                        GridEntry::Symbol(entry)
                    } else {
                        GridEntry::Symbol(entry)
                    }
                };
            }
        }
        let grid = Rc::new(grid);

        // 2nd iteration for number detection, we first need a finished grid to reference,
        // so we can't do this in the same loop
        for (row, line) in input.lines().enumerate() {
            let mut current_num_start = None;
            for (col, entry) in line.char_indices() {
                if entry.is_ascii_digit() && current_num_start.is_none() {
                    // Stumbled upon a new number, mark its start
                    current_num_start = Some(col);
                } else if let Some(start) = current_num_start {
                    let eol = col + 1 == width;
                    match (entry.is_ascii_digit(), eol) {
                        (true, true) => {
                            // number that reaches to the end of the line
                            numbers.push(Number {
                                value: line[start..=col].parse().unwrap(),
                                row,
                                start,
                                end: col,
                                grid: Rc::clone(&grid),
                            });
                            current_num_start = None;
                        }
                        (true, false) => {
                            // still reading a number
                            continue;
                        }
                        _ => {
                            // number ended normally
                            numbers.push(Number {
                                value: line[start..col].parse().unwrap(),
                                row,
                                start,
                                end: col - 1,
                                grid: Rc::clone(&grid),
                            });
                            current_num_start = None;
                        }
                    }
                }
            }
        }
        Ok(Schematic {
            all_numbers: numbers,
            grid,
            gear_symbols,
        })
    }

    fn part_numbers(&self) -> Vec<&Number> {
        self.all_numbers
            .iter()
            .filter(|n| n.is_part_number())
            .collect_vec()
    }

    fn find_number_by_pos(&self, pos: Pos) -> Option<&Number> {
        self.all_numbers
            .iter()
            .find(|n| n.positions().contains(&pos))
    }

    fn gear_ratios(&self) -> u32 {
        let mut total = 0;
        for gear_symbol in &self.gear_symbols {
            let mut numbers_found = vec![];
            for row in gear_symbol.pos.row.saturating_sub(1)..=gear_symbol.pos.row + 1 {
                let mut next_possible_numcell = 0;
                for col in gear_symbol.pos.col.saturating_sub(1)..=gear_symbol.pos.col + 1 {
                    if col < next_possible_numcell {
                        // still in a previous number, continue
                        continue;
                    }
                    if let Some(GridEntry::NumberDigit(_)) = self.grid.at(row, col) {
                        let num = self.find_number_by_pos(Pos { row, col }).unwrap();
                        numbers_found.push(num);
                        next_possible_numcell = num.end + 2
                    }
                }
            }
            if numbers_found.len() == 2 {
                total += numbers_found[0].value * numbers_found[1].value;
            }
        }
        total
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
    grid: Rc<Grid>,
}
impl Number {
    fn is_part_number(&self) -> bool {
        for row in self.row.saturating_sub(1)..=self.row + 1 {
            for col in self.start.saturating_sub(1)..=self.end + 1 {
                if let Some(GridEntry::Symbol(_)) = self.grid.at(row, col) {
                    return true;
                }
            }
        }
        false
    }
    fn positions(&self) -> Vec<Pos> {
        (self.start..=self.end)
            .map(|i| Pos {
                row: self.row,
                col: i,
            })
            .collect_vec()
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GearSymbol {
    pos: Pos,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Grid {
    inner: Vec<Vec<GridEntry>>,
    width: usize,
    height: usize,
}
impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            inner: vec![vec![GridEntry::Empty; width]; height],
            width,
            height,
        }
    }
    fn at(&self, row: usize, col: usize) -> Option<GridEntry> {
        if row < self.height && col < self.width {
            Some(self.inner[row][col])
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum GridEntry {
    Empty,
    NumberDigit(u32),
    Symbol(char),
}

#[tokio::main]
async fn main() -> Result<()> {
    let test_schematic = Schematic::from_str(TEST)?;
    let input_schematic = Schematic::from_str(INPUT)?;

    let test_part_numbers = test_schematic
        .part_numbers()
        .iter()
        .map(|num| num.value)
        .collect_vec();
    //dbg!(&test_part_numbers);
    println!(
        "Test: Sum of part numbers: {}",
        test_part_numbers.iter().sum::<u32>()
    );
    println!("Test: Gear ratios: {}", test_schematic.gear_ratios());

    let input_part_numbers = input_schematic
        .part_numbers()
        .iter()
        .map(|num| num.value)
        .collect_vec();

    println!(
        "Input: Sum of part numbers: {}",
        input_part_numbers.iter().sum::<u32>()
    );
    println!("Input: Gear ratios: {}", input_schematic.gear_ratios());

    //dbg!(input_part_numbers);
    Ok(())
}
