use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Resources {
    pub water: f32,
    pub food: f32,
    pub energy: f32,
    pub population: f32,
}

impl Resources {
    pub fn is_empty(&self) -> bool {
        self.water == 0.0 && self.food == 0.0 && self.energy == 0.0 && self.population == 0.0
    }
}

/// Live resource pools for the colony.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct Stocks {
    pub water: f32,
    pub food: f32,
    pub energy: f32,
}