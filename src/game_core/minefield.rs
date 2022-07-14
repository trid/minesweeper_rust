use crate::game_core::array2d::Array2D;
use crate::game_core::defines::CellCoordinates;


#[derive(Clone)]
pub enum CellType {
    Mine,
    Counter(u8),
}

#[derive(Clone)]
#[derive(Default)]
pub struct Cell {
    pub cell_type: CellType,
    pub open: bool,
}

pub struct Minefield {
    field: Array2D<Cell>,
}

impl Minefield {
    pub fn new(field: Array2D<Cell>) -> Minefield {
        Minefield{field}
    }

    pub fn width(&self) -> usize {
        self.field.width()
    }

    pub fn height(&self) -> usize {
        self.field.height()
    }

    pub fn cell(&self, cell_coordinates: CellCoordinates) -> &Cell {
        &self.field[cell_coordinates]
    }

    pub fn open_cell(&mut self, cell_coordinates: CellCoordinates) {
        self.field[cell_coordinates].open = true;
    }
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Counter(0)
    }
}