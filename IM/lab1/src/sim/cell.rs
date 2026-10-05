use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub use crate::sim::citizen::CitizenData;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellState {
    Empty,
    Citizen,
    Construction,
    Overlap,
    AntiCell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstructionKind {
    Water,
    Food,
    Energy,
    Population,
    Conflict,
}

impl ConstructionKind {
    pub fn index(self) -> usize {
        match self {
            ConstructionKind::Water => 0,
            ConstructionKind::Food => 1,
            ConstructionKind::Energy => 2,
            ConstructionKind::Population => 3,
            ConstructionKind::Conflict => 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub state: CellState,
    pub kind: Option<ConstructionKind>,
    pub construction_id: Option<u32>,
    pub citizen: Option<CitizenData>,
    pub frozen: Option<CitizenData>,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            state: CellState::Empty,
            kind: None,
            construction_id: None,
            citizen: None,
            frozen: None,
        }
    }

    pub fn citizen(data: CitizenData) -> Self {
        Self {
            state: CellState::Citizen,
            kind: None,
            construction_id: None,
            citizen: Some(data),
            frozen: None,
        }
    }

    pub fn construction(kind: ConstructionKind, id: u32, frozen: CitizenData) -> Self {
        Self {
            state: CellState::Construction,
            kind: Some(kind),
            construction_id: Some(id),
            citizen: None,
            frozen: Some(frozen),
        }
    }

    pub fn overlap(id: u32, frozen: CitizenData) -> Self {
        Self {
            state: CellState::Overlap,
            kind: None,
            construction_id: Some(id),
            citizen: None,
            frozen: Some(frozen),
        }
    }

    pub fn anticell() -> Self {
        Self {
            state: CellState::AntiCell,
            kind: None,
            construction_id: None,
            citizen: None,
            frozen: None,
        }
    }

    pub fn is_regular(&self) -> bool {
        matches!(
            self.state,
            CellState::Citizen | CellState::Construction | CellState::Overlap
        )
    }

    pub fn is_occupied(&self) -> bool {
        self.state != CellState::Empty
    }
}
