use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    vec,
};

use log::{debug, trace};

use crate::moves::{Direction, Move};

#[derive(Debug, Clone)]
pub struct Rope {
    head: Cell,
    head_visited: HashSet<Cell>,
    next: Segment,
    length: usize,
}

impl Rope {
    pub fn new() -> Self {
        Rope::with_length(2)
    }

    /// Create a longer rope. Will `panic!()` if length is less than 2
    pub fn with_length(length: usize) -> Self {
        if length < 2 {
            panic!()
        }
        let mut r = Rope {
            head: Cell { x: 0, y: 0 },
            next: Segment::new(Cell { x: 0, y: 0 }),
            head_visited: HashSet::new(),
            length,
        };
        if length > 2 {
            for _ in 0..length - 1 {
                r.next.add_segment();
            }
        }
        r
    }

    pub fn move_head(&mut self, mov: &Move) {
        for _ in 0..mov.amount {
            self.move_head_by_1(mov.direction);
            self.next.follow(&self.head);
        }
    }

    fn move_head_by_1(&mut self, direction: Direction) {
        let new_head = match direction {
            Direction::Up => Cell {
                x: self.head.x,
                y: self.head.y + 1,
            },
            Direction::Down => Cell {
                x: self.head.x,
                y: self.head.y - 1,
            },
            Direction::Left => Cell {
                x: self.head.x - 1,
                y: self.head.y,
            },
            Direction::Right => Cell {
                x: self.head.x + 1,
                y: self.head.y,
            },
        };
        debug!("Moved Head {} -> {}", self.head, new_head);
        self.head_visited.insert(new_head);
        self.head = new_head;
    }

    pub fn visited_count_head(&self) -> usize {
        self.head_visited.len()
    }

    pub fn visited_count_tail(&self) -> usize {
        self.visited_count_segment(self.length - 1)
    }

    pub fn visited_count_segment(&self, segment: usize) -> usize {
        if self.length <= segment {
            panic!()
        }
        if segment == 0 {
            self.head_visited.len()
        } else {
            self.next.visited_count(segment, 1)
        }
    }

    pub fn segment_positions(&self) -> Vec<Cell> {
        let mut positions = vec![self.head];
        positions.extend(self.next.segment_positions());
        positions
    }
}

impl Default for Rope {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Segment {
    pos: Cell,
    next: Option<Box<Segment>>,
    visited: HashSet<Cell>,
}
#[derive(Debug, Hash, PartialEq, Eq, Default, Clone, Copy)]
pub struct Cell {
    x: i32,
    y: i32,
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl Segment {
    pub fn new(pos: Cell) -> Self {
        Segment {
            pos,
            next: None,
            visited: vec![Cell { x: 0, y: 0 }]
                .into_iter()
                .collect::<HashSet<_>>(),
        }
    }

    pub fn add_segment(&mut self) {
        if self.next.is_none() {
            self.next = Some(Box::new(Segment::new(self.pos)));
        } else {
            self.next.as_mut().unwrap().add_segment();
        }
    }

    pub fn follow(&mut self, parent: &Cell) {
        trace!("Parent at {}, we're at {}", parent, self.pos);
        let x_diff = parent.x - self.pos.x;
        let y_diff = parent.y - self.pos.y;
        trace!("Calculated x/y differences: {},{}", x_diff, y_diff);
        let new_pos = match (x_diff.abs(), y_diff.abs()) {
            (0, 0) => {
                trace!("Parent and Segment are on top of each other, not moving Segment");
                self.pos
            }
            (0, 1) | (1, 0) | (1, 1) => {
                trace!("Parent and Segment are next to each other, not moving Segment");
                self.pos
            }
            (2, 0) | (0, 2) => {
                trace!("Parent and Segment are in straight line, following with Segment");
                // Can't just match on direction as a longer rope can swing back and forth
                match (x_diff, y_diff) {
                    (1.., 0) => Cell {
                        x: self.pos.x + 1,
                        y: self.pos.y,
                    },
                    (i32::MIN..=-1, 0) => Cell {
                        x: self.pos.x - 1,
                        y: self.pos.y,
                    },
                    (0, 1..) => Cell {
                        x: self.pos.x,
                        y: self.pos.y + 1,
                    },
                    (0, i32::MIN..=-1) => Cell {
                        x: self.pos.x,
                        y: self.pos.y - 1,
                    },
                    _ => unreachable!(),
                }
            }
            (2, 1) | (1, 2) | (2, 2) => {
                trace!("Parent is offset, following with Segment diagonally");
                match (x_diff.is_negative(), y_diff.is_negative()) {
                    (true, true) => Cell {
                        x: self.pos.x - 1,
                        y: self.pos.y - 1,
                    },
                    (true, false) => Cell {
                        x: self.pos.x - 1,
                        y: self.pos.y + 1,
                    },
                    (false, true) => Cell {
                        x: self.pos.x + 1,
                        y: self.pos.y - 1,
                    },
                    (false, false) => Cell {
                        x: self.pos.x + 1,
                        y: self.pos.y + 1,
                    },
                }
            }
            _ => unreachable!(),
        };
        if new_pos != self.pos {
            debug!("Moved Segment {}->{}", self.pos, new_pos);
            self.pos = new_pos;
            self.visited.insert(new_pos);
            if self.next.is_some() {
                self.next.as_mut().unwrap().follow(&self.pos);
            }
        }
    }

    pub fn segment_positions(&self) -> VecDeque<Cell> {
        match &self.next {
            Some(n) => {
                let mut p = n.segment_positions();
                p.push_front(self.pos);
                p
            }
            None => {
                let mut p = VecDeque::new();
                p.push_front(self.pos);
                p
            }
        }
    }

    pub fn visited_count(&self, wanted: usize, depth: usize) -> usize {
        if wanted == depth {
            self.visited.len()
        } else {
            match &self.next {
                Some(n) => n.visited_count(wanted, depth + 1),
                None => panic!(),
            }
        }
    }
}
