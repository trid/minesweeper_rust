use crate::game_core::defines::{CellCoordinates, CoordinatesSet};
use crate::{CellType, Minefield};

pub struct CellOpener;

pub enum OpenResult {
    Success,
    Fail,
}

impl CellOpener {
    fn add_empty_tiles(cells_to_open: &mut CoordinatesSet, cell_coordinates: CellCoordinates, minefield: &mut Minefield) {
        if cell_coordinates.0 == usize::MAX || cell_coordinates.0 >= minefield.width() {
            return;
        }
        if cell_coordinates.1 == usize::MAX || cell_coordinates.1 >= minefield.height() {
            return;
        }

        let cell = minefield.cell(cell_coordinates);
        if !cell.open {
            if let CellType::Counter(count) = cell.cell_type {
                if count == 0 {
                    cells_to_open.insert(cell_coordinates);
                }
            }
        }
        minefield.open_cell(cell_coordinates);
    }

    fn open_tiles_breath_first(minefield: &mut Minefield, cell_coordinates: CellCoordinates) {
        let mut cells_to_open = CoordinatesSet::new();
        cells_to_open.insert(cell_coordinates);

        while !cells_to_open.is_empty() {
            let current_item = *cells_to_open.iter().next().unwrap();
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0.wrapping_sub(1), current_item.1), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0.wrapping_sub(1), current_item.1.wrapping_sub(1)), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0, current_item.1.wrapping_sub(1)), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0 + 1, current_item.1.wrapping_sub(1)), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0 + 1, current_item.1), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0 + 1, current_item.1 + 1), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0, current_item.1 + 1), minefield);
            Self::add_empty_tiles(&mut cells_to_open, (current_item.0.wrapping_sub(1), current_item.1 + 1), minefield);

            cells_to_open.remove(&current_item);
        }
    }


    pub fn open_cell(mine_field: &mut Minefield, cell_coordinates: CellCoordinates) -> OpenResult {
        let cell = mine_field.cell(cell_coordinates);
        if cell.open {
            return OpenResult::Success;
        }

        let cell_type = cell.cell_type.clone();
        mine_field.open_cell(cell_coordinates);

        if let CellType::Counter(counter) = cell_type {
            if counter == 0 {
                Self::open_tiles_breath_first(mine_field, cell_coordinates);
            }
            return OpenResult::Success;
        }

        OpenResult::Fail
    }
}