use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimConfig {
    pub population_max: u32,
    pub food_per_citizen: f32,
    pub water_capacity: f32,
    pub food_capacity: f32,
    pub energy_capacity: f32,
    pub water_need_per_citizen: f32,
    pub energy_need_per_citizen: f32,
    pub c_max: f32,
    pub f_max: f32,
    pub l_max: f32,
    pub stress_hunger: f32,
    pub stress_thirst: f32,
    pub stress_dark: f32,
    pub stress_over: f32,
    pub stress_conflict_per_tick: f32,
    pub conflict_radius: usize,
    pub fatigue_per_tick: f32,
    pub fatigue_rest: f32,
    pub rest_radius: usize,
    pub loyalty_surplus: f32,
    pub loyalty_deficit: f32,
    pub loyalty_stress: f32,
    pub citizen_move_enabled: bool,
    pub citizen_move_prob: f32,
    pub anticell_enabled: bool,
    pub seed: u64,
    /// Lower bound for the per-tick food requirement, so a small colony
    /// cannot live off a rounding error.
    #[serde(default)]
    pub food_reserve: f32,
    #[serde(default)]
    pub water_reserve: f32,
    #[serde(default)]
    pub energy_reserve: f32,
}

impl SimConfig {
    pub fn food_need(&self, citizens: usize) -> f32 {
        (citizens as f32 * self.food_per_citizen).max(self.food_reserve)
    }
    pub fn water_need(&self, citizens: usize) -> f32 {
        (citizens as f32 * self.water_need_per_citizen).max(self.water_reserve)
    }
    pub fn energy_need(&self, citizens: usize) -> f32 {
        (citizens as f32 * self.energy_need_per_citizen).max(self.energy_reserve)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RenderConfig {
    pub cell_px: usize,
    pub border_px: usize,
    pub show_grid_lines: bool,
    pub overlay_stress: bool,
    pub overlay_fatigue: bool,
    pub overlay_loyalty: bool,
    pub show_construction_outlines: bool,
    pub show_conflict_radius: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            population_max: 100,
            food_per_citizen: 0.1,
            water_capacity: 500.0,
            food_capacity: 500.0,
            energy_capacity: 500.0,
            water_need_per_citizen: 0.05,
            energy_need_per_citizen: 0.05,
            c_max: 100.0,
            f_max: 100.0,
            l_max: 100.0,
            stress_hunger: 2.0,
            stress_thirst: 1.5,
            stress_dark: 1.5,
            stress_over: 3.0,
            stress_conflict_per_tick: 0.5,
            conflict_radius: 3,
            fatigue_per_tick: 0.2,
            fatigue_rest: 0.5,
            rest_radius: 2,
            loyalty_surplus: 0.3,
            loyalty_deficit: 1.0,
            loyalty_stress: 0.5,
            citizen_move_enabled: true,
            citizen_move_prob: 0.15,
            anticell_enabled: true,
            seed: 42,
            food_reserve: 2.0,
            water_reserve: 2.0,
            energy_reserve: 2.0,
        }
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            cell_px: 4,
            border_px: 1,
            show_grid_lines: true,
            overlay_stress: false,
            overlay_fatigue: false,
            overlay_loyalty: false,
            show_construction_outlines: true,
            show_conflict_radius: false,
        }
    }
}
