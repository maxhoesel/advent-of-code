use std::{collections::HashMap, fmt::Display};

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    row: usize,
    col: usize,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Boulder {
    pos: Pos,
    kind: BoulderKind,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum BoulderKind {
    Rolling,
    Fixed,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct BoulderField {
    boulders: Vec<Boulder>,
    height: usize,
    width: usize,
}
impl BoulderField {
    fn parse(input: &str) -> BoulderField {
        let mut boulders = vec![];

        for (row_idx, row) in input.lines().enumerate() {
            for (col_idx, char) in row.char_indices() {
                match char {
                    '#' => boulders.push(Boulder {
                        pos: Pos {
                            row: row_idx,
                            col: col_idx,
                        },
                        kind: BoulderKind::Fixed,
                    }),
                    'O' => boulders.push(Boulder {
                        pos: Pos {
                            row: row_idx,
                            col: col_idx,
                        },
                        kind: BoulderKind::Rolling,
                    }),
                    '.' => (),
                    e => panic!("Unknown char: {e}"),
                }
            }
        }
        let width = boulders
            .iter()
            .max_by_key(|b| b.pos.col)
            .map(|b| b.pos.col + 1)
            .unwrap_or(0);
        let height = boulders
            .iter()
            .max_by_key(|b| b.pos.row)
            .map(|b| b.pos.row + 1)
            .unwrap_or(0);

        assert!(height == width);

        BoulderField {
            boulders,
            height,
            width,
        }
    }

    fn rotate(&mut self) {
        for boulder in self.boulders.iter_mut() {
            // Reverse row
            boulder.pos.row = self.width - boulder.pos.row - 1;
            // Transpose
            std::mem::swap(&mut boulder.pos.row, &mut boulder.pos.col);
        }
    }

    fn roll_up(&mut self) {
        let mut boulders = self.boulders.iter_mut().collect_vec();
        for col in 0..=self.width {
            let mut boulders_in_col = boulders
                .iter_mut()
                .filter(|b| b.pos.col == col)
                .sorted_by_key(|b| b.pos.row)
                .collect_vec();
            // Roll em up!
            let mut previous_boulder: Option<&mut &mut &mut Boulder> = None;
            for boulder in boulders_in_col.iter_mut() {
                if boulder.kind == BoulderKind::Rolling {
                    match previous_boulder {
                        Some(prev) => boulder.pos.row = prev.pos.row + 1,
                        None => boulder.pos.row = 0,
                    }
                }
                previous_boulder = Some(boulder);
            }
        }
    }

    fn calculate_load_up(&self) -> usize {
        self.boulders
            .iter()
            .filter(|b| b.kind == BoulderKind::Rolling)
            .map(|b| self.height - b.pos.row)
            .sum()
    }

    fn cycle(&mut self) {
        self.roll_up();
        self.rotate();
        self.roll_up();
        self.rotate();
        self.roll_up();
        self.rotate();
        self.roll_up();
        self.rotate();
    }
}
impl Display for BoulderField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.boulders.iter().find(|b| b.pos == Pos { row, col }) {
                    Some(Boulder { pos: _, kind }) => match kind {
                        BoulderKind::Rolling => write!(f, "O"),
                        BoulderKind::Fixed => write!(f, "#"),
                    },
                    None => write!(f, "."),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

const TEST: &str = include_str!("test.txt");
const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut test_field = BoulderField::parse(TEST);
    //println!("{}", test_field);
    test_field.roll_up();
    //println!("{}", test_field);
    println!("Test load (North): {}", test_field.calculate_load_up());

    let mut input_field = BoulderField::parse(INPUT);
    input_field.roll_up();
    println!("Input load (North): {}", input_field.calculate_load_up());

    // test rotation
    let tmp = test_field.clone();
    test_field.rotate();
    test_field.rotate();
    test_field.rotate();
    test_field.rotate();
    assert_eq!(tmp, test_field);

    // test cycling
    test_field.cycle();
    test_field.cycle();
    test_field.cycle();
    println!("Cycle Test:\n{}", test_field);

    // as expected, this takes forever - about 2 weeks apparently
    /*
    let mut input_field = BoulderField::parse(INPUT);
    // Repeated duration - no cycle detection for now
    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] (Remaining: {eta}) {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap();
    for _ in (0..1_000_000_000).progress_with_style(style) {
        input_field.cycle();
    }
    */

    // also didn't quite work out
    /*
    let mut input_field = BoulderField::parse(INPUT);
    let mut input_field2 = input_field.clone();
    // Floyd's Cycle Detection Algorithm
    // input_field 2 is the fast one
    input_field.cycle();
    input_field2.cycle();
    input_field2.cycle();
    let spin = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template("[{pos}] {spin}").unwrap());
    while input_field != input_field2 {
        input_field.cycle();
        input_field2.cycle();
        input_field2.cycle();
        spin.inc(1);
    }
    // both iters are now at a point where they match

    // Cycle detected - find first repetition
    // to do so, reset the slow one and make both march until we find a match
    let mut mu = 0;
    let mut input_field = BoulderField::parse(INPUT);
    while input_field != input_field2 {
        input_field.cycle();
        input_field2.cycle();
        mu += 1;
    }

    // find the length of the cycle by making the other counter march until we come back around
    let mut lambda = 1;
    input_field2 = input_field.clone();
    input_field2.cycle();
    while input_field != input_field2 {
        input_field2.cycle();
        lambda += 1;
    }
    println!("Cycle detected at: {mu} with period {lambda}");
     */

    println!("{}", std::mem::size_of::<Boulder>());

    // simple, memory-intensive cycle detection
    let mut input_field = BoulderField::parse(INPUT);
    let mut count = 0;
    let mut seen: HashMap<BoulderField, usize> = HashMap::new();
    seen.insert(input_field.clone(), count);
    let spin = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template("[{pos}] {spin}").unwrap());
    let (start, len) = loop {
        input_field.cycle();
        if let Some(prev) = seen.get(&input_field) {
            break (prev, count - prev);
        }
        seen.insert(input_field.clone(), count);
        count += 1;
        spin.inc(1);
    };
    println!("Cycle detected at {start} with length {len}");

    println!("Input load (North): {}", input_field.calculate_load_up());
}
