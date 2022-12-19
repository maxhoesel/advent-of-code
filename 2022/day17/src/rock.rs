use itertools::Itertools;

use crate::grid::{DefaultGrid, GridCoord};

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct RockBuilder {
    counter: usize,
    shapes: Vec<Rock>,
}
impl RockBuilder {
    pub fn new() -> Self {
        RockBuilder {
            counter: 0,
            // Shape origin is bottom-left
            shapes: vec![
                Rock {
                    bits: vec![
                        // Wide
                        GridCoord { x: 0, y: 0 },
                        GridCoord { x: 1, y: 0 },
                        GridCoord { x: 2, y: 0 },
                        GridCoord { x: 3, y: 0 },
                    ],
                },
                Rock {
                    bits: vec![
                        // +
                        GridCoord { x: 1, y: 0 },
                        GridCoord { x: 0, y: 1 },
                        GridCoord { x: 1, y: 1 },
                        GridCoord { x: 2, y: 1 },
                        GridCoord { x: 1, y: 2 },
                    ],
                },
                Rock {
                    bits: vec![
                        // Corner, inverse L
                        GridCoord { x: 2, y: 2 },
                        GridCoord { x: 2, y: 1 },
                        GridCoord { x: 2, y: 0 },
                        GridCoord { x: 1, y: 0 },
                        GridCoord { x: 0, y: 0 },
                    ],
                },
                Rock {
                    bits: vec![
                        // High
                        GridCoord { x: 0, y: 0 },
                        GridCoord { x: 0, y: 1 },
                        GridCoord { x: 0, y: 2 },
                        GridCoord { x: 0, y: 3 },
                    ],
                },
                Rock {
                    bits: vec![
                        // Square
                        GridCoord { x: 0, y: 0 },
                        GridCoord { x: 0, y: 1 },
                        GridCoord { x: 1, y: 1 },
                        GridCoord { x: 1, y: 0 },
                    ],
                },
            ],
        }
    }
    pub fn drop_at_pos(&mut self, pos: &GridCoord) -> Rock {
        let r = Rock {
            bits: self.shapes[self.counter % self.shapes.len()]
                .bits
                .iter()
                .cloned()
                .map(|piece| GridCoord {
                    x: piece.x + pos.x,
                    y: piece.y + pos.y,
                })
                .collect_vec(),
        };
        self.counter += 1;
        r
    }
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Rock {
    bits: Vec<GridCoord>,
}
impl Rock {
    pub fn push(&mut self, direction: Direction) {
        for b in &mut self.bits {
            match direction {
                Direction::Left => b.x -= 1,
                Direction::Right => b.x += 1,
                Direction::Up => b.y += 1,
                Direction::Down => b.y -= 1,
            }
        }
    }
    pub fn push_back(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.push(Direction::Right),
            Direction::Right => self.push(Direction::Left),
            Direction::Up => self.push(Direction::Down),
            Direction::Down => self.push(Direction::Up),
        }
    }
    pub fn bits(&self) -> impl Iterator<Item = &GridCoord> {
        self.bits.iter()
    }

    pub fn left(&self) -> isize {
        self.bits.iter().map(|b| b.x).min().unwrap_or(0)
    }
    pub fn right(&self) -> isize {
        self.bits.iter().map(|b| b.x).max().unwrap_or(0)
    }
    pub fn top(&self) -> isize {
        self.bits.iter().map(|b| b.y).max().unwrap_or(0)
    }
    pub fn bot(&self) -> isize {
        self.bits.iter().map(|b| b.y).min().unwrap_or(0)
    }
    pub fn collides<T>(&self, grid: &dyn DefaultGrid<T>) -> bool
    where
        T: Default,
    {
        for b in &self.bits {
            if grid.at_non_default(b).is_some() {
                return true;
            }
        }
        return false;
    }
}
