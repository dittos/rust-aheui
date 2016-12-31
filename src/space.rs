use cell::Cell;

#[derive(Debug)]
pub struct Space {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Space {
    pub fn from_cells(cells: Vec<Vec<Cell>>) -> Space {
        let width = cells.iter().map(|row| row.len()).max().unwrap_or(0);
        let height = cells.len();
        Space {
            width: width,
            height: height,
            cells: cells.iter().map(|row| {
                let mut filled = row.clone();
                filled.resize(width, Cell::empty());
                filled
            }).collect()
        }
    }

    pub fn cell_at(&self, x: i32, y: i32) -> &Cell {
        &self.cells[y as usize][x as usize]
    }
}
