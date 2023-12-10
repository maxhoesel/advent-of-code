use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    num::TryFromIntError,
    ops::Deref,
};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

const EMPTY: char = '.';
const START: char = 'S';

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, EnumIter)]
enum Segment {
    UpRight,
    RightDown,
    DownLeft,
    LeftUp,
    UpDown,
    LeftRight,
}
impl TryFrom<char> for Segment {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::UpRight),
            'F' => Ok(Self::RightDown),
            '7' => Ok(Self::DownLeft),
            'J' => Ok(Self::LeftUp),
            '|' => Ok(Self::UpDown),
            '-' => Ok(Self::LeftRight),
            e => Err(anyhow!("Not a pipe: {e}")),
        }
    }
}
impl Segment {
    fn connections(&self) -> Vec<Direction> {
        match self {
            Segment::UpRight => vec![Direction::Up, Direction::Right],
            Segment::RightDown => vec![Direction::Down, Direction::Right],
            Segment::DownLeft => vec![Direction::Left, Direction::Down],
            Segment::LeftUp => vec![Direction::Up, Direction::Left],
            Segment::UpDown => vec![Direction::Up, Direction::Down],
            Segment::LeftRight => vec![Direction::Left, Direction::Right],
        }
    }
    fn next_direction(&self, entered: Direction) -> Result<Direction> {
        match (self, entered) {
            (Segment::UpRight, Direction::Right) => Ok(Direction::Up),
            (Segment::UpRight, Direction::Up) => Ok(Direction::Right),
            (Segment::RightDown, Direction::Right) => Ok(Direction::Down),
            (Segment::RightDown, Direction::Down) => Ok(Direction::Right),
            (Segment::DownLeft, Direction::Left) => Ok(Direction::Down),
            (Segment::DownLeft, Direction::Down) => Ok(Direction::Left),
            (Segment::LeftUp, Direction::Left) => Ok(Direction::Up),
            (Segment::LeftUp, Direction::Up) => Ok(Direction::Left),
            (Segment::UpDown, Direction::Up) => Ok(Direction::Down),
            (Segment::UpDown, Direction::Down) => Ok(Direction::Up),
            (Segment::LeftRight, Direction::Left) => Ok(Direction::Right),
            (Segment::LeftRight, Direction::Right) => Ok(Direction::Left),
            e => Err(anyhow!("From direction not valid for {self:?}: {e:?}")),
        }
    }
    fn fits_together(&self, other: Segment, attach_at: Direction) -> bool {
        self.connections().contains(&attach_at) || other.connections().contains(&attach_at.rev())
    }
}
impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Segment::UpRight => "L",
                Segment::RightDown => "F",
                Segment::DownLeft => "7",
                Segment::LeftUp => "J",
                Segment::UpDown => "|",
                Segment::LeftRight => "-",
            }
        )
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, EnumIter)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn rev(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    fn neighbour(&self, dir: Direction) -> Option<Pos> {
        match dir {
            Direction::Left if self.col > 0 => Some(Pos {
                row: self.row,
                col: self.col - 1,
            }),
            Direction::Right => Some(Pos {
                row: self.row,
                col: self.col + 1,
            }),
            Direction::Up if self.row > 0 => Some(Pos {
                row: self.row - 1,
                col: self.col,
            }),
            Direction::Down => Some(Pos {
                row: self.row + 1,
                col: self.col,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Element {
    pos: Pos,
    segment: Option<Segment>,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.segment {
                Some(s) => s.to_string(),
                None => EMPTY.to_string(),
            }
        )
    }
}
impl Element {
    fn neighbours(&self) -> impl Iterator<Item = Pos> + '_ {
        Direction::iter().filter_map(|dir| self.pos.neighbour(dir))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<Element>>,
    start: Pos,
}
impl Grid {
    fn from_input(input: &str) -> Result<Grid> {
        let height = input.lines().count();
        if !input.lines().map(|l| l.len()).all_equal() {
            return Err(anyhow!("Not a regular grid!"));
        }
        let width = input.lines().map(|l| l.len()).next().unwrap_or(0);

        let mut grid = vec![Vec::with_capacity(width); height];
        let mut start_pos = None;
        for (row, line) in input.lines().enumerate() {
            for (col, symb) in line.char_indices() {
                let element = Element {
                    pos: Pos { row, col },
                    segment: if symb == EMPTY {
                        None
                    } else if symb == START {
                        start_pos = Some(Pos { row, col });
                        None // start will be filled in later
                    } else {
                        Some(Segment::try_from(symb)?)
                    },
                };
                grid[row].push(element);
            }
        }

        let Some(start_pos) = start_pos else {
            return Err(anyhow!("No start element in pipe network"));
        };

        // Determine starting pipe type
        let mut possible_shapes: HashSet<Segment> = HashSet::from_iter(Segment::iter());
        for dir in Direction::iter() {
            let Some(neigh_pos) = start_pos.neighbour(dir) else {
                possible_shapes.retain(|s| !s.connections().contains(&dir));
                continue; // at the edge of the map
            };
            let Some(neigh) = grid.get(neigh_pos.row).and_then(|r| r.get(neigh_pos.col)) else {
                possible_shapes.retain(|s| !s.connections().contains(&dir));
                continue;
            };
            if let Some(neigh_seg) = neigh.segment {
                if !neigh_seg.connections().contains(&dir.rev()) {
                    possible_shapes.retain(|s| !s.connections().contains(&dir.rev()));
                }
            } else {
                possible_shapes.retain(|s| !s.connections().contains(&dir));
            }
        }
        if possible_shapes.len() != 1 {
            return Err(anyhow!("Start piece is ambiguous or not a loop piece!"));
        }
        let start_seg = possible_shapes.into_iter().next().unwrap();
        grid[start_pos.row][start_pos.col].segment = Some(start_seg);

        Ok(Grid {
            width,
            height,
            grid,
            start: start_pos,
        })
    }
    fn at(&self, pos: Pos) -> Option<&Element> {
        self.grid.get(pos.row)?.get(pos.col)
    }
    fn try_step(&self, from: &Element, dir: Direction) -> Option<&Element> {
        let Some(from_seg) = from.segment else {
            return None;
        };
        let Some(to) = self.at(from.pos.neighbour(dir)?) else {
            return None;
        };
        let Some(to_seg) = to.segment else {
            return None;
        };
        if from_seg.fits_together(to_seg, dir) {
            return Some(to);
        }
        None
    }
    fn find_loop(&self) -> Option<Loop> {
        let start = self.at(self.start).unwrap();

        let mut len_count = 0;
        let mut elements = vec![start];
        let mut current = start;
        let mut step_dir = start.segment.unwrap().connections()[0];

        // Take the first step from start manually because we have two options here
        current = match self.try_step(current, step_dir) {
            Some(e) => e,
            None => return None,
        };
        len_count += 1;
        elements.push(current);

        // Keep going until we return to start
        loop {
            step_dir = current.segment?.next_direction(step_dir.rev()).ok()?;
            current = match self.try_step(current, step_dir) {
                Some(e) => e,
                None => return None,
            };
            len_count += 1;
            elements.push(current);
            if current.pos == self.start {
                let mut loop_grid = vec![Vec::with_capacity(self.width); self.height];
                #[allow(clippy::needless_range_loop)]
                for row in 0..self.height {
                    for col in 0..self.width {
                        loop_grid[row].push(Element {
                            pos: Pos { row, col },
                            segment: None,
                        });
                    }
                }
                for element in &elements {
                    loop_grid[element.pos.row][element.pos.col] = **element;
                }
                let grid = Grid {
                    width: self.width,
                    height: self.height,
                    grid: loop_grid,
                    start: self.start,
                };
                return Some(Loop {
                    len: len_count / 2,
                    elements: elements.iter().map(|e| **e).collect_vec(),
                    grid,
                });
            }
        }
    }
    fn find_connected_empty_plane(&self, start: &Element) -> Vec<&Element> {
        todo!()
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(|e| e.to_string()).join(""))
                .join("\n")
        )
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Loop {
    len: usize,
    elements: Vec<Element>,
    grid: Grid,
}
impl Display for Loop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Length: {}", self.len)?;
        self.grid.fmt(f)
    }
}
impl Loop {
    fn contained_elements(&self) -> Vec<&Element> {
        let mut visited_outside: HashSet<&Element> = HashSet::new();
        let loop_neighbours: VecDeque<&Element> = self
            .elements
            .iter()
            .map(|element| {
                element
                    .neighbours()
                    .filter_map(|pos| self.grid.at(pos).filter(|neigh| neigh.segment.is_none()))
            })
            .flatten()
            .collect();

        for neigh in loop_neighbours {
            let mut visisted = HashSet::new();
            let mut todo: VecDeque<_> = neigh
                .neighbours()
                .filter_map(|pos| self.grid.at(pos).filter(|neigh| neigh.segment.is_none()))
                .filter(|e| !visited_outside.contains(e))
                .collect();
            while let Some(current) = todo.pop_front() {
                visisted.insert(current);
                for new in current
                    .neighbours()
                    .filter_map(|pos| self.grid.at(pos).filter(|neigh| neigh.segment.is_none()))
                    .filter(|neigh| !visisted.contains(neigh))
                {
                    todo.push_back(new);
                }
            }
            if visisted.iter().any(|element| {
                element.pos.row == 0
                    || element.pos.row + 1 == self.grid.height
                    || element.pos.col == 0
                    || element.pos.col + 1 == self.grid.width
            }) {
                // we have reached the edge, this block is outside
                visited_outside.extend(visisted.iter());
            }
        }
        todo!()
    }
}

const TEST1: &str = include_str!("test1.txt");
const TEST2: &str = include_str!("test2.txt");
const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let test1_grid = Grid::from_input(TEST1)?;
    println!("{}", test1_grid);
    let test1_loop = test1_grid.find_loop().ok_or(anyhow!("No loop"))?;
    println!("{}", test1_loop);

    let test2_grid = Grid::from_input(TEST2)?;
    println!("{}", test2_grid);
    let test2_loop = test2_grid.find_loop().ok_or(anyhow!("No loop"))?;
    println!("{}", test2_loop);

    let input_grid = Grid::from_input(INPUT)?;
    let input_loop = input_grid.find_loop().ok_or(anyhow!("No loop"))?;
    println!("{}", input_loop);
    Ok(())
}
