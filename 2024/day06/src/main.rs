use std::collections::HashSet;

use nalgebra::DMatrix;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}
impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Top,
            Direction::Right => Direction::Bottom,
            Direction::Top => Direction::Right,
            Direction::Bottom => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Element {
    Empty,
    Blocked,
}
impl TryFrom<char> for Element {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | '^' => Ok(Element::Empty),
            '#' => Ok(Element::Blocked),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    map: DMatrix<Element>,
    guard_start: Pos,
}
impl Puzzle {
    fn from_input(input: &str) -> Puzzle {
        let mut elements = Vec::with_capacity(input.len());
        let mut guard_start = Pos { row: 0, col: 0 };
        for (row, line) in input.lines().enumerate() {
            if let Some((guard_col, _)) = line.match_indices("^").next() {
                guard_start = Pos {
                    row,
                    col: guard_col,
                };
            }
            elements.extend(line.chars().map(|c| Element::try_from(c).unwrap()));
        }
        Puzzle {
            map: DMatrix::from_row_iterator(
                input.lines().count(),
                input.lines().next().unwrap().len(),
                elements,
            ),
            guard_start,
        }
    }

    fn solve_guard(&self) -> (HashSet<(Pos, Direction)>, bool) {
        let mut visited = HashSet::new();
        let mut guard_pos = self.guard_start;
        let mut guard_direction = Direction::Top;
        visited.insert((self.guard_start, guard_direction));
        loop {
            let maybe_ahead = match guard_direction {
                Direction::Left if guard_pos.col > 0 => Some(Pos {
                    row: guard_pos.row,
                    col: guard_pos.col - 1,
                }),
                Direction::Right if guard_pos.col < self.map.ncols() - 1 => Some(Pos {
                    row: guard_pos.row,
                    col: guard_pos.col + 1,
                }),
                Direction::Top if guard_pos.row > 0 => Some(Pos {
                    row: guard_pos.row - 1,
                    col: guard_pos.col,
                }),
                Direction::Bottom if guard_pos.row < self.map.nrows() - 1 => Some(Pos {
                    row: guard_pos.row + 1,
                    col: guard_pos.col,
                }),
                _ => None,
            };
            let Some(ahead) = maybe_ahead else {
                return (visited, true);
            };
            match self.map.get((ahead.row, ahead.col)).unwrap() {
                Element::Empty => {
                    if visited.contains(&(ahead, guard_direction)) {
                        // loop detected
                        return (visited, false);
                    }
                    guard_pos = ahead;
                    visited.insert((guard_pos, guard_direction));
                }
                Element::Blocked => guard_direction = guard_direction.rotate_right(),
            };
        }
    }
}

#[allow(dead_code)]
const SAMPLE: &str = include_str!("sample.txt");
#[allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

fn main() {
    let puzzle = Puzzle::from_input(INPUT);
    let visited = puzzle.solve_guard();

    println!(
        "Finished: {}, Visisted: {}",
        visited.1,
        visited
            .0
            .iter()
            .map(|visit| visit.0)
            .collect::<HashSet<_>>()
            .len()
    );

    let mut loop_opportunities = HashSet::new();
    for (tile, _) in visited.0 {
        if tile == puzzle.guard_start {
            continue;
        }
        let mut modified_puzzle = puzzle.clone();
        modified_puzzle.map[(tile.row, tile.col)] = Element::Blocked;
        if !modified_puzzle.solve_guard().1 {
            loop_opportunities.insert(tile);
        }
    }
    println!("Loop opportunities: {}", loop_opportunities.len())
}
