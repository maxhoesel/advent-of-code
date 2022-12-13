use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
}
impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            cells: vec![],
            width,
            height,
            start: None,
            end: None,
        }
    }

    pub fn insert_cell(&mut self, x: usize, y: usize, elevation: u8) -> &Cell {
        let index = y * self.width + x;
        let newcell = Cell { x, y, elevation };
        match self.cells.get(index) {
            Some(_) => {
                let _ = std::mem::replace(&mut &self.cells[index], &newcell);
            }
            None => {
                self.cells.insert(index, newcell);
            }
        }
        self.cell_at(x, y).unwrap()
    }
    pub fn start(&self) -> Option<&Cell> {
        self.start.and_then(|(x, y)| self.cell_at(x, y))
    }
    pub fn set_start(&mut self, x: usize, y: usize) {
        self.start = Some((x, y));
    }
    pub fn end(&self) -> Option<&Cell> {
        self.end.and_then(|(x, y)| self.cell_at(x, y))
    }
    pub fn set_end(&mut self, x: usize, y: usize) {
        self.end = Some((x, y));
    }

    pub fn cell_at(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.width + x)
    }
    pub fn cell_by_id(&self, id: u32) -> Option<&Cell> {
        self.cells.get(id as usize)
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }
    pub fn cell_top(&self, cell: &Cell) -> Option<&Cell> {
        if cell.y == 0 {
            return None;
        }
        self.cell_at(cell.x, cell.y - 1)
    }
    pub fn cell_bot(&self, cell: &Cell) -> Option<&Cell> {
        self.cell_at(cell.x, cell.y + 1)
    }
    pub fn cell_left(&self, cell: &Cell) -> Option<&Cell> {
        if cell.x == 0 {
            return None;
        }
        self.cell_at(cell.x - 1, cell.y)
    }
    pub fn cell_right(&self, cell: &Cell) -> Option<&Cell> {
        self.cell_at(cell.x + 1, cell.y)
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.cells.chunks_exact(self.width) {
            writeln!(
                f,
                "|{}|",
                l.iter().fold(String::new(), |mut str, next| {
                    str.push_str((next.elevation as char).to_string().as_str());
                    str
                })
            )?
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default, Hash)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub elevation: u8,
}
impl Cell {
    pub fn cost_to(&self, other: &Cell) -> Option<u8> {
        if other.elevation > self.elevation + 1 || !self.is_neighbor(other) {
            return None;
        }
        Some(1)
    }

    pub fn is_neighbor(&self, other: &Cell) -> bool {
        matches!(
            (self.x.abs_diff(other.x), self.y.abs_diff(other.y)),
            (0, 1) | (1, 0)
        )
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}):{}", self.x, self.y, self.elevation)
    }
}
