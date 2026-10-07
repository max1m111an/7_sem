#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Phyto,
    Zoo,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub state: CellState,
    pub energy: f32,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            state: CellState::Empty,
            energy: 0.0,
        }
    }
    pub fn phyto() -> Self {
        Self {
            state: CellState::Phyto,
            energy: 0.0,
        }
    }
    pub fn zoo(energy: f32) -> Self {
        Self {
            state: CellState::Zoo,
            energy,
        }
    }
}
