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

/// Вид живой клетки в игре «Жизнь».
///
/// Вид — это не состояние (`CellState` колонии не расширяется), а метка
/// поверх `Citizen`: живая клетка всегда несёт ровно один вид, мёртвая —
/// не несёт ничего.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Species {
    /// Добыча: живёт по B3/S23 среди своих и гибнет рядом с хищником.
    Prey,
    /// Хищник: рождается стаей рядом с добычей и умирает от голода.
    Predator,
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
    /// Вид живой клетки игры «Жизнь»; `None` у мёртвых клеток и у клеток
    /// колонии, которые игру не играют.
    #[serde(default)]
    pub species: Option<Species>,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            state: CellState::Empty,
            kind: None,
            construction_id: None,
            citizen: None,
            frozen: None,
            species: None,
        }
    }

    pub fn citizen(data: CitizenData) -> Self {
        Self {
            state: CellState::Citizen,
            kind: None,
            construction_id: None,
            citizen: Some(data),
            frozen: None,
            species: None,
        }
    }

    pub fn construction(kind: ConstructionKind, id: u32, frozen: CitizenData) -> Self {
        Self {
            state: CellState::Construction,
            kind: Some(kind),
            construction_id: Some(id),
            citizen: None,
            frozen: Some(frozen),
            species: None,
        }
    }

    pub fn overlap(id: u32, frozen: CitizenData) -> Self {
        Self {
            state: CellState::Overlap,
            kind: None,
            construction_id: Some(id),
            citizen: None,
            frozen: Some(frozen),
            species: None,
        }
    }

    pub fn anticell() -> Self {
        Self {
            state: CellState::AntiCell,
            kind: None,
            construction_id: None,
            citizen: None,
            frozen: None,
            species: None,
        }
    }

    /// A live cell of the Game of Life — a prey by default.
    ///
    /// The colony rules below keep their own states; Life only ever creates
    /// `Citizen` (alive) and `Empty` (dead), so a live cell carries no citizen
    /// data — [`Cell::is_alive`] is the test. The species tag is what tells
    /// prey from predators.
    pub fn alive() -> Self {
        Self::with_species(Species::Prey)
    }

    /// A live cell of the given species.
    pub fn with_species(species: Species) -> Self {
        Self {
            state: CellState::Citizen,
            kind: None,
            construction_id: None,
            citizen: None,
            frozen: None,
            species: Some(species),
        }
    }

    /// Whether this cell is alive in the Game of Life.
    pub fn is_alive(&self) -> bool {
        self.state == CellState::Citizen
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
