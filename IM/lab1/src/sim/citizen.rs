use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CitizenData {
    pub stress: f32,
    pub fatigue: f32,
    pub loyalty: f32,
}

impl CitizenData {
    pub fn neutral(l_max: f32) -> Self {
        Self {
            stress: 0.0,
            fatigue: 0.0,
            loyalty: l_max / 2.0,
        }
    }
}
