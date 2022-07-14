use crate::Minefield;

pub struct CellOpener;

pub enum OpenResult {
    Success,
    Fail
}

impl CellOpener {
    pub fn open_cell(mine_field: &mut Minefield, x: usize, y: usize) -> OpenResult {
        OpenResult::Success
    }
}