use crate::Minefield;


pub struct GameState {
    mine_field: Minefield,
}

impl GameState {
    pub fn new(mine_field: Minefield) -> Self {
        GameState { mine_field }
    }

    pub fn mine_field_mut(&mut self) -> &mut Minefield {
        &mut self.mine_field
    }
}