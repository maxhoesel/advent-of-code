use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use super::{util::display_top_bot_line, DefaultGrid, GridCoord, Origin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseDefaultGrid<T> {
    elements: HashMap<GridCoord, T>,
    default: T,
    origin: Origin,
}
impl<T> DefaultGrid<T> for SparseDefaultGrid<T>
where
    T: Default + Clone + Display + Debug + Copy,
{
    fn at(&self, pos: &GridCoord) -> Option<&T> {
        self.elements.get(pos).or(Some(&self.default))
    }

    fn set(&mut self, pos: &GridCoord, element: T) -> Option<T> {
        let old = self.elements.remove(&pos).or(Some(self.default));
        self.elements.insert(*pos, element);
        old
    }

    fn at_non_default(&self, pos: &GridCoord) -> Option<&T> {
        self.elements.get(pos)
    }
}

impl<T> SparseDefaultGrid<T>
where
    T: Display + Debug + Copy + Default,
{
    pub fn new(origin: Origin) -> Self {
        SparseDefaultGrid {
            elements: HashMap::new(),
            default: T::default(),
            origin,
        }
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

    fn _write_line(&self, y: isize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.x_min()..=self.x_max() {
            write!(
                f,
                "{}",
                self.elements
                    .get(&GridCoord { x, y })
                    .unwrap_or(&self.default)
            )?;
        }
        writeln!(f, "|")
    }
}

impl<T> Display for SparseDefaultGrid<T>
where
    T: Copy + Display + Debug + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_top_bot_line((self.x_max().abs_diff(self.x_min()) + 1) as isize, 4, f)?;
        match self.origin {
            Origin::BotLeft => {
                for y in (self.y_min()..=self.y_max()).rev() {
                    write!(f, "{y:>3} |")?;
                    self._write_line(y, f)?;
                }
            }
            Origin::TopLeft => {
                for y in self.y_min()..=self.y_max() {
                    write!(f, "{y:>3} |")?;
                    self._write_line(y, f)?;
                }
            }
        }
        display_top_bot_line((self.x_max().abs_diff(self.x_min()) + 1) as isize, 4, f)?;
        Ok(())
    }
}
