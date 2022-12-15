use std::fmt::Display;

use itertools::Itertools;
use log::info;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Cave {
    height: usize,
    width: usize,
    elements: Vec<Element>,
}
impl Cave {
    pub fn new(height: usize, width: usize) -> Self {
        Cave {
            height,
            width,
            elements: vec![Element::Void; height * width],
        }
    }
    pub fn at(&self, pos: &CavePos) -> Option<&Element> {
        self.elements.get(self._index(pos))
    }
    pub fn set(&mut self, pos: &CavePos, element: Element) {
        if self._in_bounds(pos) {
            let i = self._index(pos);
            let _ = std::mem::replace(&mut self.elements[i], element);
        }
    }
    // Drop some sand into the cave at pos and see where it comes to rest.
    // Returns None if the sand falls into the abyss below 💀
    pub fn drop_sand(&mut self, pos: &CavePos) -> Option<CavePos> {
        info!("Dropping Sand at {}", pos);
        let mut current = pos.clone();
        loop {
            let options = vec![
                CavePos {
                    x: current.x,
                    y: current.y + 1,
                },
                CavePos {
                    x: current.x - 1,
                    y: current.y + 1,
                },
                CavePos {
                    x: current.x + 1,
                    y: current.y + 1,
                },
            ];

            let mut abyss_drop = false;
            let new = options
                .into_iter()
                .filter_map(|new| match self.at(&new) {
                    Some(Element::Void) => {
                        info!("Sand drops to {}", new);
                        Some(new)
                    }
                    None => {
                        info!("Sand drops into the endless abyss...");
                        abyss_drop = true;
                        return None;
                    }
                    Some(e) => {
                        info!("Cannot drop to {}, occupied by {:?}", new, e);
                        None
                    }
                })
                .next();
            if abyss_drop {
                return None;
            }
            match new {
                Some(new) => {
                    current = new;
                }
                None => break,
            };
        }
        self.set(&current, Element::Sand);
        info!("Sand has settled at {}", current);
        Some(current)
    }

    fn _index(&self, pos: &CavePos) -> usize {
        self.width * pos.y + pos.x
    }
    fn _pos(&self, idx: usize) -> CavePos {
        CavePos {
            x: idx % self.width,
            y: idx / self.width,
        }
    }
    fn _in_bounds(&self, pos: &CavePos) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}
impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements_x_cords = self
            .elements
            .iter()
            .enumerate()
            .filter(|e| !matches!(e.1, Element::Void))
            .map(|e| self._pos(e.0).x)
            .sorted()
            .collect_vec();
        let first_element_x = *elements_x_cords.first().unwrap();
        let last_element_x = *elements_x_cords.last().unwrap();

        for y in 0..self.height {
            write!(f, "|")?;
            if first_element_x != 0 {
                write!(f, "**")?;
            }
            for x in first_element_x..last_element_x {
                write!(f, "{}", self.elements[self._index(&CavePos { x, y })])?;
            }
            if last_element_x + 1 != self.width {
                write!(f, "**")?;
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct CavePos {
    pub x: usize,
    pub y: usize,
}
impl Display for CavePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Element {
    Void,
    Sand,
    Rock,
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Void => write!(f, "."),
            Element::Sand => write!(f, "o"),
            Element::Rock => write!(f, "#"),
        }
    }
}
