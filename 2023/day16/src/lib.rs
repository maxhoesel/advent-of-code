use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.row.cmp(&other.row) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.col.cmp(&other.col),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element<T>
where
    T: Clone + PartialEq,
{
    pub element: T,
    pub pos: Position,
}

#[derive(Debug, Clone)]
pub struct SparseGrid<T>
where
    T: Clone + PartialEq,
{
    elements: HashMap<Position, T>,
    max_row: usize,
    max_col: usize,
    most_right: HashSet<Position>,
    most_down: HashSet<Position>,
}
impl<T> SparseGrid<T>
where
    T: Clone + PartialEq,
{
    pub fn new() -> SparseGrid<T> {
        SparseGrid::default()
    }
    pub fn with_capacity(cap: usize) -> SparseGrid<T> {
        SparseGrid {
            elements: HashMap::with_capacity(cap),
            max_col: 0,
            max_row: 0,
            most_right: HashSet::new(),
            most_down: HashSet::new(),
        }
    }

    fn update_dimensions(&mut self) {
        let max_rows = self.elements.keys().max_set_by_key(|pos| pos.row);
        let max_cols = self.elements.keys().max_set_by_key(|pos| pos.col);
        if max_rows.is_empty() {
            self.max_col = 0;
            self.max_row = 0;
        } else {
            self.most_down = max_rows.iter().copied().copied().collect();
            self.most_right = max_cols.iter().copied().copied().collect();
            self.max_row = self.most_down.iter().next().unwrap().row;
            self.max_col = self.most_right.iter().next().unwrap().col;
        }
    }

    pub fn height(&self) -> usize {
        if self.elements.is_empty() {
            0
        } else {
            self.max_row + 1
        }
    }

    pub fn width(&self) -> usize {
        if self.elements.is_empty() {
            0
        } else {
            self.max_col + 1
        }
    }

    pub fn put(&mut self, pos: Position, element: T) -> Option<T> {
        if pos.row > self.max_row {
            self.max_row = pos.row;
            self.most_down.clear();
        }
        if pos.row == self.max_row {
            self.most_down.insert(pos);
        }

        if pos.col > self.max_col {
            self.max_col = pos.col;
            self.most_right.clear();
        }
        if pos.col == self.max_col {
            self.most_right.insert(pos);
        }

        self.elements.insert(pos, element)
    }
    pub fn pop(&mut self, pos: &Position) -> Option<Element<T>> {
        if pos.row == self.max_row {
            self.most_down.remove(pos);
        }
        if pos.col == self.max_col {
            self.most_right.remove(pos);
        }

        let e = self.elements.remove(pos).map(|e| Element {
            element: e,
            pos: *pos,
        });

        if self.most_down.is_empty() || self.most_right.is_empty() {
            self.update_dimensions()
        }

        e
    }
    pub fn get(&self, pos: &Position) -> Option<Element<&T>> {
        self.elements.get(pos).map(|e| Element {
            element: e,
            pos: *pos,
        })
    }
    pub fn next_in_direction(&self, from: &Position, direction: &Direction) -> Option<Element<&T>> {
        match direction {
            Direction::Up => self
                .elements
                .iter()
                .filter(|(p, _)| p.col == from.col && p.row < from.row)
                .max_by_key(|(p, _)| p.row)
                .map(|(p, elem)| Element {
                    element: elem,
                    pos: *p,
                }),
            Direction::Down => self
                .elements
                .iter()
                .filter(|(p, _)| p.col == from.col && p.row > from.row)
                .min_by_key(|(p, _)| p.row)
                .map(|(p, elem)| Element {
                    element: elem,
                    pos: *p,
                }),
            Direction::Left => self
                .elements
                .iter()
                .filter(|(p, _)| p.row == from.row && p.col < from.col)
                .max_by_key(|(p, _)| p.col)
                .map(|(p, elem)| Element {
                    element: elem,
                    pos: *p,
                }),
            Direction::Right => self
                .elements
                .iter()
                .filter(|(p, _)| p.row == from.row && p.col > from.col)
                .min_by_key(|(p, _)| p.col)
                .map(|(p, elem)| Element {
                    element: elem,
                    pos: *p,
                }),
        }
    }
    pub fn find(&self, element: T) -> Option<Element<&T>> {
        self.elements
            .iter()
            .find(|v| v.1 == &element)
            .map(|e| Element {
                element: e.1,
                pos: *e.0,
            })
    }
    pub fn find_all(&self, element: T) -> Vec<Element<&T>> {
        self.elements
            .iter()
            .filter(|v| v.1 == &element)
            .map(|e| Element {
                element: e.1,
                pos: *e.0,
            })
            .collect_vec()
    }
    pub fn contains(&self, element: T) -> bool {
        self.elements.values().contains(&element)
    }
    pub fn contains_position(&self, pos: &Position) -> bool {
        self.elements.contains_key(pos)
    }
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Display for SparseGrid<T>
where
    T: Clone + PartialEq + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.elements.is_empty() {
            write!(f, "[]")?;
            return Ok(());
        }
        let out = ((0..self.height()).map(|row| {
            (0..self.width())
                .map(|col| {
                    self.get(&Position { row, col })
                        .map_or(".".to_string(), |elem| elem.element.to_string())
                })
                .collect::<String>()
        }))
        .fold(
            String::with_capacity(
                self.elements.len() * self.elements.iter().next().unwrap().1.to_string().len(),
            ),
            |mut acc, s| {
                if !acc.is_empty() {
                    acc.push('\n');
                }
                acc.push_str(&s);
                acc
            },
        );

        write!(f, "{}", out)
    }
}

impl<T> Default for SparseGrid<T>
where
    T: Clone + PartialEq,
{
    fn default() -> Self {
        Self {
            elements: Default::default(),
            max_row: Default::default(),
            max_col: Default::default(),
            most_right: Default::default(),
            most_down: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insertion() {
        let mut map = SparseGrid::new();
        map.put(Position { row: 0, col: 0 }, 1);
        map.put(Position { row: 0, col: 1 }, 2);
        map.put(Position { row: 1, col: 1 }, 3);
        assert_eq!(map.get(&Position { row: 0, col: 0 }).unwrap().element, &1);
        assert_eq!(map.get(&Position { row: 0, col: 1 }).unwrap().element, &2);
        assert_eq!(map.get(&Position { row: 1, col: 1 }).unwrap().element, &3);
        assert!(map.contains(1))
    }

    #[test]
    fn put_overwriting() {
        let mut map = SparseGrid::new();
        map.put(Position { row: 0, col: 0 }, 1);
        map.put(Position { row: 0, col: 0 }, 2);
        assert_eq!(map.get(&Position { row: 0, col: 0 }).unwrap().element, &2);
    }

    #[test]
    fn pop() {
        let mut map = SparseGrid::new();
        map.put(Position { row: 0, col: 0 }, 1);
        assert_eq!(map.pop(&Position { row: 0, col: 0 }).unwrap().element, 1);
        assert!(map.is_empty())
    }

    #[test]
    fn dimensions() {
        let mut map = SparseGrid::new();
        map.put(Position { row: 0, col: 0 }, 1);
        assert_eq!(map.width(), 1);
        assert_eq!(map.height(), 1);

        map.put(Position { row: 0, col: 4 }, 1);
        assert_eq!(map.width(), 5);
        assert_eq!(map.height(), 1);

        map.put(Position { row: 7, col: 4 }, 1);
        assert_eq!(map.width(), 5);
        assert_eq!(map.height(), 8);

        map.pop(&Position { row: 0, col: 4 });
        assert_eq!(map.width(), 5);
        assert_eq!(map.height(), 8);

        map.pop(&Position { row: 7, col: 4 });
        assert_eq!(map.width(), 1);
        assert_eq!(map.height(), 1);

        map.pop(&Position { row: 0, col: 0 });
        assert_eq!(map.width(), 0);
        assert_eq!(map.height(), 0);
    }

    #[test]
    fn find_direction() {
        let mut map = SparseGrid::new();
        map.put(Position { row: 0, col: 0 }, 1);
        map.put(Position { row: 0, col: 3 }, 2);
        map.put(Position { row: 0, col: 5 }, 3);
        map.put(Position { row: 2, col: 0 }, 4);
        map.put(Position { row: 2, col: 5 }, 5);
        map.put(Position { row: 2, col: 9 }, 6);

        assert!(map
            .next_in_direction(&Position { row: 0, col: 0 }, &Direction::Up)
            .is_none());
        assert!(map
            .next_in_direction(&Position { row: 0, col: 0 }, &Direction::Left)
            .is_none());
        assert_eq!(
            map.next_in_direction(&Position { row: 0, col: 0 }, &Direction::Right)
                .unwrap()
                .element,
            &2
        );
        assert_eq!(
            map.next_in_direction(&Position { row: 0, col: 0 }, &Direction::Down)
                .unwrap()
                .element,
            &4
        );
    }
}
