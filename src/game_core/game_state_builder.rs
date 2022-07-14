use core::panicking::panic;
use std::collections::HashSet;

use rand::{Rng, thread_rng};
use crate::core::array2d::Array2D;

use crate::core::game_state::GameState;
use crate::core::minefield::Cell;
use crate::{CellType, Minefield};

pub struct GameStateBuilder;

type CellCoordinates = (usize, usize);
type CoordinatesSet = HashSet<CellCoordinates>;

impl GameStateBuilder {
    fn emplace_mine(mines: &mut CoordinatesSet, coordinates: CellCoordinates,
                    width: usize, height: usize) {
        let mut updated_coordinates = coordinates;
        while mines.contains(&updated_coordinates) {
            let mut new_x = updated_coordinates.0;
            let mut new_y = updated_coordinates.1;

            if coordinates.0 < width - 1 {
                new_x += 1;
            }
            else {
                new_x = 0;
                new_y += 1;
                new_y = new_y % height;
            }
            updated_coordinates = (new_x, new_y);
        }
        mines.insert(updated_coordinates);
    }

    fn lay_mines(width: usize, height: usize, mines_count: u8) -> CoordinatesSet {
        let mut result: CoordinatesSet = CoordinatesSet::new();

        for i in 0..mines_count {
            let x = thread_rng().gen_range(0..width);
            let y = thread_rng().gen_range(0..height);
            result.insert((x, y));
        }

        result
    }

    fn update_cell(cells: &mut Array2D<Cell>, cell_coordinates: CellCoordinates) {
        if cell_coordinates.0 == usize::MAX || cell_coordinates.0 >= cells.width() {
            return;
        }
        if cell_coordinates.1 == usize::MAX || cell_coordinates.1 >= cells.height() {
            return;
        }

        let mut cell = &mut cells[cell_coordinates];
        if let CellType::Counter(count) = cell.cell_type {
            cell.cell_type = CellType::Counter(count + 1);
        }
    }

    fn update_adjacent_cells(cells: &mut Array2D<Cell>, cell_coordinates: CellCoordinates) {
        update_cell(cells, cell_coordinates.0.wrapping_sub(1), cell_coordinates.1);
        update_cell(cells, cell_coordinates.0.wrapping_sub(1), cell_coordinates.1.wrapping_sub(1));
        update_cell(cells, cell_coordinates.0, cell_coordinates.1.wrapping_sub(1));
        update_cell(cells, cell_coordinates.0 + 1, cell_coordinates.1.wrapping_sub(1));
        update_cell(cells, cell_coordinates.0 + 1, cell_coordinates.1);
        update_cell(cells, cell_coordinates.0 + 1, cell_coordinates.1 + 1);
        update_cell(cells, cell_coordinates.0, cell_coordinates.1 + 1);
        update_cell(cells, cell_coordinates.0.wrapping_sub(1), cell_coordinates.1 + 1);
    }

    fn make_mine_field(width: usize, height: usize, mines: &CoordinatesSet) -> Minefield {
        let mut cells = Array2D::<Cell>::new(width, height);

        for item in mines {
            cells[item].cell_type = CellType::Mine;
            update_adjacent_cells(&cells, item);
        }

        Minefield::new(cells)
    }

    pub fn make_game_state(width: usize, height: usize, mines_count: u8) -> GameState {
        /*
            if (minesNumber > width * height) {
        throw std::logic_error("Impossible to place all bombs!");
    }

    auto mines = generateMinesPosition(width, height, minesNumber);
    auto minefield = makeMineField(width, height, mines);

    return GameState(minesNumber, std::move(minefield));
         */

        if mines_count as usize > width * height {
            panic("Too many bombs on level!");
        }

        let mines = lay_mines(width, height, mines_count);
        let minefield = make_mine_field(width, height, mines);

        GameState::new(Minefield::new(minefield))
    }
}
