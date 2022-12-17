use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    origin: GridCoord,
    width: usize,
    height: usize,
    elements: Vec<T>,
}
impl<T> Grid<T>
where
    T: Default + Clone + Display + Debug,
{
    pub fn new(width: usize, height: usize, origin: GridCoord) -> Self {
        Grid {
            width,
            height,
            origin,
            elements: vec![T::default(); width * height],
        }
    }
    pub fn at(&self, pos: &GridCoord) -> Option<&T> {
        if pos.x < self.origin.x || pos.y < self.origin.y {
            return None;
        }
        self.elements.get(self._index(pos))
    }
    /// Set an element in the grid to `T`. Returns an option containing the previous element at that coordinate,
    /// or None if the position is out of bounds
    pub fn set(&mut self, pos: &GridCoord, element: T) -> Option<T> {
        if self._in_bounds(pos) {
            let i = self._index(pos);
            let old = std::mem::replace(&mut self.elements[i], element);
            return Some(old);
        }
        None
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
impl<T> Display for Grid<T>
where
    T: Default + Clone + Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.origin.y..(self.height as isize + self.origin.y) {
            write!(f, "{y:>3} |")?;
            for x in self.origin.x..(self.width as isize + self.origin.x) {
                write!(f, "{}", self.elements[self._index(&GridCoord { x, y })])?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
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
