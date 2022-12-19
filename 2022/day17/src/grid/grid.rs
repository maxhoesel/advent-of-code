use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
};

use super::{util::display_top_bot_line, DefaultGrid, GridCoord};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CellGrid<T> {
    origin: GridCoord,
    width: usize,
    height: usize,
    elements: Vec<T>,
}
impl<T> DefaultGrid<T> for CellGrid<T>
where
    T: Default + Clone + Display + Debug + PartialEq<T>,
{
    fn at(&self, pos: &GridCoord) -> Option<&T> {
        if pos.x < self.origin.x || pos.y < self.origin.y {
            return None;
        }
        self.elements.get(self._index(pos))
    }

    fn set(&mut self, pos: &GridCoord, element: T) -> Option<T> {
        if self._in_bounds(pos) {
            let i = self._index(pos);
            let old = std::mem::replace(&mut self.elements[i], element);
            return Some(old);
        }
        None
    }

    fn at_non_default(&self, pos: &GridCoord) -> Option<&T> {
        if pos.x < self.origin.x || pos.y < self.origin.y {
            return None;
        }
        match self.elements.get(self._index(pos)) {
            Some(e) => {
                if T::default() == *e {
                    return None;
                } else {
                    return Some(e);
                }
            }
            None => None,
        }
    }
}

impl<T> CellGrid<T>
where
    T: Default + Clone + Display + Debug,
{
    pub fn new(width: usize, height: usize, origin: GridCoord) -> Self {
        CellGrid {
            width,
            height,
            origin,
            elements: vec![T::default(); width * height],
        }
    }

    fn _index(&self, pos: &GridCoord) -> usize {
        self.width * (pos.y - self.origin.y) as usize + (pos.x - self.origin.x) as usize
    }
    fn _pos(&self, idx: usize) -> GridCoord {
        GridCoord {
            x: (idx % self.width) as isize - self.origin.x,
            y: (idx / self.width) as isize - self.origin.y,
        }
    }
    fn _in_bounds(&self, pos: &GridCoord) -> bool {
        pos.x >= self.origin.x
            && pos.y >= self.origin.y
            && pos.x < self.width as isize
            && pos.y < self.height as isize
    }
}
impl<T> Display for CellGrid<T>
where
    T: Default + Clone + Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display_top_bot_line(self.width as isize, 4, f)?;
        for y in self.origin.y..(self.height as isize + self.origin.y) {
            write!(f, "{y:>3} |")?;
            for x in self.origin.x..(self.width as isize + self.origin.x) {
                write!(f, "{}", self.elements[self._index(&GridCoord { x, y })])?;
            }
            writeln!(f, "|")?;
        }
        display_top_bot_line(self.width as isize, 4, f)?;
        Ok(())
    }
}
