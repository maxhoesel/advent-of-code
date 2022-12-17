use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use super::GridCoord;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SparseDefaultGrid<T> {
    elements: HashMap<GridCoord, T>,
    default: T,
}
impl<T> SparseDefaultGrid<T>
where
    T: Display + Debug + Copy + Default,
{
    pub fn new() -> Self {
        SparseDefaultGrid {
            elements: HashMap::new(),
            default: T::default(),
        }
    }

    // Returns the element in the grid, returns none if the pos is out of bounds
    pub fn at(&self, pos: &GridCoord) -> Option<&T> {
        self.elements.get(pos).or(Some(&self.default))
    }
    /// Set an element in the HashGrid to `T`. Returns an option containing the previous element at that coordinate,
    /// or None if the position is out of bounds
    pub fn set(&mut self, pos: GridCoord, element: T) -> Option<T> {
        let old = self.elements.remove(&pos).or(Some(self.default));
        self.elements.insert(pos, element);
        old
    }

    pub fn y_min(&self) -> isize {
        self.elements.keys().map(|cord| cord.y).min().unwrap_or(0)
    }
    pub fn y_max(&self) -> isize {
        self.elements.keys().map(|cord| cord.y).max().unwrap_or(0)
    }
    pub fn x_min(&self) -> isize {
        self.elements.keys().map(|cord| cord.x).min().unwrap_or(0)
    }
    pub fn x_max(&self) -> isize {
        self.elements.keys().map(|cord| cord.x).max().unwrap_or(0)
    }

    pub fn width(&self) -> usize {
        self.x_max().abs_diff(self.x_min())
    }
    pub fn height(&self) -> usize {
        self.y_max().abs_diff(self.y_min())
    }
}

impl<T> Display for SparseDefaultGrid<T>
where
    T: Copy + Display + Debug + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_min()..=self.y_max() {
            write!(f, "{y:>3} |")?;
            for x in self.x_min()..=self.x_max() {
                write!(
                    f,
                    "{}",
                    self.elements
                        .get(&GridCoord { x, y })
                        .unwrap_or(&self.default)
                )?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}
