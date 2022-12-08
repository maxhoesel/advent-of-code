use std::fmt::Display;

use itertools::Itertools;
use log::debug;

#[derive(Debug, Clone)]
pub struct TreeGrid {
    grid: Vec<Vec<Tree>>,
    rows: usize,
    cols: usize,
}

impl TreeGrid {
    pub fn new(matrix: Vec<Vec<u8>>) -> Self {
        let mut tree_grid = TreeGrid {
            grid: Vec::new(),
            rows: 0,
            cols: 0,
        };
        let grid = matrix
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, height)| Tree {
                        row: r,
                        col: c,
                        height: *height,
                    })
                    .collect()
            })
            .collect_vec();
        tree_grid.rows = grid.len();
        tree_grid.cols = grid.iter().map(|r: &Vec<Tree>| r.len()).max().unwrap();
        tree_grid.grid = grid;
        tree_grid
    }
    pub fn at(&self, row: usize, col: usize) -> Option<&Tree> {
        self.grid.get(row)?.get(col)
    }
    pub fn height(&self) -> usize {
        self.rows
    }
    pub fn width(&self) -> usize {
        self.cols
    }
}
impl Display for TreeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for t in row {
                write!(f, "{}", t.height)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
struct Visibility {
    range: usize,
    to_edge: bool,
}
impl std::ops::Mul<Visibility> for Visibility {
    type Output = usize;

    fn mul(self, rhs: Visibility) -> Self::Output {
        self.range * rhs.range
    }
}
impl std::ops::Mul<usize> for Visibility {
    type Output = usize;

    fn mul(self, rhs: usize) -> Self::Output {
        self.range * rhs
    }
}
impl std::ops::Mul<Visibility> for usize {
    type Output = usize;

    fn mul(self, rhs: Visibility) -> Self::Output {
        self * rhs.range
    }
}

#[derive(Debug, Clone)]
pub struct Tree {
    row: usize,
    col: usize,
    pub height: u8,
}
impl Tree {
    pub fn is_visible(&self, grid: &TreeGrid) -> bool {
        if self.col == 0
            || self.row == 0
            || grid.at(self.row, self.col + 1).is_none()
            || grid.at(self.row + 1, self.col).is_none()
        {
            debug!("Tree {} is visible because it's on the outside", self);
            return true;
        }
        if self.visibility_to_left(grid).to_edge {
            debug!("Tree {} is visible from the left", self);
            return true;
        }
        if self.visibility_to_right(grid).to_edge {
            debug!("Tree {} is visible from the right", self);
            return true;
        }
        if self.visibility_to_top(grid).to_edge {
            debug!("Tree {} is visible from the top", self);
            return true;
        }
        if self.visibility_to_bot(grid).to_edge {
            debug!("Tree {} is visible from the bottom", self);
            return true;
        }

        debug!("Tree {} is not visible from any side", self);
        false
    }

    pub fn visibility_score(&self, grid: &TreeGrid) -> usize {
        let score = self.visibility_to_bot(grid)
            * self.visibility_to_left(grid)
            * self.visibility_to_right(grid)
            * self.visibility_to_top(grid);
        debug!("Visibility score for tree {}: {}", self, score);
        score
    }

    fn visibility_to_left(&self, grid: &TreeGrid) -> Visibility {
        if self.col == 0 {
            return Visibility {
                range: 0,
                to_edge: true,
            };
        }

        let mut i = 0;
        for left in (0..=self.col - 1).rev() {
            match self.higher_than(grid, self.row, left) {
                Some(true) => {
                    i += 1;
                }
                Some(false) => {
                    return Visibility {
                        range: i + 1,
                        to_edge: false,
                    }
                }
                None => break,
            }
        }
        Visibility {
            range: i,
            to_edge: true,
        }
    }

    fn visibility_to_right(&self, grid: &TreeGrid) -> Visibility {
        let mut i = 0;
        for right in self.col + 1.. {
            match self.higher_than(grid, self.row, right) {
                Some(true) => {
                    i += 1;
                }
                Some(false) => {
                    return Visibility {
                        range: i + 1,
                        to_edge: false,
                    }
                }
                None => break,
            }
        }
        Visibility {
            range: i,
            to_edge: true,
        }
    }

    fn visibility_to_top(&self, grid: &TreeGrid) -> Visibility {
        if self.row == 0 {
            return Visibility {
                range: 0,
                to_edge: true,
            };
        }

        let mut i = 0;
        for top in (0..=self.row - 1).rev() {
            match self.higher_than(grid, top, self.col) {
                Some(true) => {
                    i += 1;
                }
                Some(false) => {
                    return Visibility {
                        range: i + 1,
                        to_edge: false,
                    }
                }
                None => break,
            }
        }
        Visibility {
            range: i,
            to_edge: true,
        }
    }

    fn visibility_to_bot(&self, grid: &TreeGrid) -> Visibility {
        let mut i = 0;
        for bot in self.row + 1.. {
            match self.higher_than(grid, bot, self.col) {
                Some(true) => {
                    i += 1;
                }
                Some(false) => {
                    return Visibility {
                        range: i + 1,
                        to_edge: false,
                    }
                }
                None => break,
            }
        }
        Visibility {
            range: i,
            to_edge: true,
        }
    }

    fn higher_than(&self, grid: &TreeGrid, row: usize, col: usize) -> Option<bool> {
        grid.at(row, col).map(|t| t.height < self.height)
    }
}
impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}
