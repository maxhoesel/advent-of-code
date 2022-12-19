mod grid;
mod sparse_grid;

mod util;

pub use grid::CellGrid;
pub use sparse_grid::SparseDefaultGrid;

use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Origin {
    BotLeft,
    TopLeft,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: isize,
    pub y: isize,
}
impl Display for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub trait DefaultGrid<T>: Display {
    fn at(&self, pos: &GridCoord) -> Option<&T>;
    fn at_non_default(&self, pos: &GridCoord) -> Option<&T>;
    fn set(&mut self, pos: &GridCoord, element: T) -> Option<T>;
}
