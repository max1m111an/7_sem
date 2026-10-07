use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SimConfig {
    pub food_value: f32,
    pub reproduction_cost: f32,
    pub child_start_energy: f32,
    pub seed: u64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            food_value: 20.0,
            reproduction_cost: 30.0,
            child_start_energy: 10.0,
            seed: 42,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub cell_px: usize,
    pub border_px: usize,
    pub show_grid_lines: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            cell_px: 8,
            border_px: 1,
            show_grid_lines: false,
        }
    }
}
