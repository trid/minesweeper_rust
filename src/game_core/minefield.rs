use crate::core::array2d::Array2D;


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

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.field[(x, y)]
    }
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Counter(0)
    }
}