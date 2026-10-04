use crate::config::SimConfig;
use crate::sim::anticell;
use crate::sim::cell::{Cell, CellState};
use crate::sim::citizen::CitizenData;
use crate::sim::construction::{self, OVERLAP_MULTIPLIER};
use crate::sim::grid::Grid;
use crate::sim::pattern::PatternLibrary;
use crate::sim::resources::Resources;

/// Per-generation aggregate state, computed from the *previous* generation.
#[derive(Clone, Debug, Default)]
pub struct GlobalCtx {
    pub n_citizen: usize,
    pub n_population: usize,
    pub production: Resources,
    pub stocks: Resources,
    pub hungry: bool,
    pub thirsty: bool,
    pub dark: bool,
    pub over: bool,
}

pub fn compute_global_counters(grid: &Grid, config: &SimConfig) -> GlobalCtx {
    let mut n_citizen = 0usize;
    let mut n_population = 0usize;
    let mut production = Resources::default();

    for c in &grid.cells {
        if c.state == CellState::Citizen {
            n_citizen += 1;
        }
        if matches!(c.state, CellState::Construction | CellState::Overlap) {
            let mult = if c.state == CellState::Overlap {
                OVERLAP_MULTIPLIER
            } else {
                1.0
            };
            if c.kind == Some(crate::sim::cell::ConstructionKind::Population) {
                n_population += 1;
            }
        }
    }

    // Production comes from construction complexes found in the current grid.
    let library = crate::sim::default_library();
    let detected = construction::detect(grid, &library);
    for d in &detected {
        for pattern in library.patterns.iter().filter(|p| p.kind == d.kind) {
            production.water += pattern.produce.water * d.cells.len() as f32;
            production.food += pattern.produce.food * d.cells.len() as f32;
            production.energy += pattern.produce.energy * d.cells.len() as f32;
            production.population += pattern.produce.population * d.cells.len() as f32;
        }
    }
    for c in &grid.cells {
        if matches!(c.state, CellState::Construction | CellState::Overlap) {
            if let Some(k) = c.kind {
                use crate::sim::cell::ConstructionKind::*;
                match k {
                    Water => production.water += 0.0,
                    Food => production.food += 0.0,
                    Energy => production.energy += 0.0,
                    Population => production.population += 0.0,
                    Conflict => {}
                }
            }
        }
    }

    let stocks = Resources {
        water: production.water,
        food: production.food,
        energy: production.energy,
        population: production.population,
    };

    let hungry = stocks.food < config.food_need(n_citizen);
    let thirsty = stocks.water < config.water_need(n_citizen);
    let dark = stocks.energy < config.energy_need(n_citizen);
    let over = n_citizen > config.population_max as usize;

    GlobalCtx {
        n_citizen,
        n_population,
        production,
        stocks,
        hungry,
        thirsty,
        dark,
        over,
    }
}

/// Rule 1: a citizen with zero loyalty becomes an anti-cell.
pub fn rule1_hunger(grid: &Grid, next: &mut [Cell]) {
    for (i, c) in grid.cells.iter().enumerate() {
        if c.state != CellState::Citizen {
            continue;
        }
        if c.citizen.map(|d| d.loyalty) == Some(0.0) {
            next[i] = Cell::anticell();
        }
    }
}

/// Rule 2: a citizen at max stress becomes an anti-cell.
pub fn rule2_stress(grid: &Grid, next: &mut [Cell], config: &SimConfig) {
    for (i, c) in grid.cells.iter().enumerate() {
        if c.state != CellState::Citizen {
            continue;
        }
        if let Some(d) = c.citizen {
            if d.stress >= config.c_max {
                next[i] = Cell::anticell();
            }
        }
    }
}

/// Rule 3: update stress / fatigue / loyalty for every surviving citizen.
pub fn rule3_citizen_update(
    grid: &Grid,
    next: &mut [Cell],
    config: &SimConfig,
    ctx: &GlobalCtx,
) {
    let conflict_sum = conflict_field(grid, config);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            if grid.cells[i].state != CellState::Citizen {
                continue;
            }
            if next[i].state == CellState::AntiCell {
                continue;
            }
            let d = match grid.cells[i].citizen {
                Some(d) => d,
                None => continue,
            };

            let mut d_stress = 0.0;
            if ctx.hungry {
                d_stress += config.stress_hunger;
            }
            if ctx.thirsty {
                d_stress += config.stress_thirst;
            }
            if ctx.dark {
                d_stress += config.stress_dark;
            }
            if ctx.over {
                d_stress += config.stress_over;
            }
            d_stress += conflict_sum[i];

            let has_rest = grid
                .cells_in_radius(x, y, config.rest_radius)
                .any(|(nx, ny)| {
                    grid.get(nx, ny).state == CellState::Construction
                        && grid.get(nx, ny).kind
                            == Some(crate::sim::cell::ConstructionKind::Population)
                });
            let d_fatigue = config.fatigue_per_tick - config.fatigue_rest * has_rest as u8 as f32;

            let surplus = !ctx.hungry && !ctx.thirsty && !ctx.dark;
            let mut d_loyalty = 0.0;
            if surplus {
                d_loyalty += config.loyalty_surplus;
            } else {
                d_loyalty -= config.loyalty_deficit;
            }
            if d.stress > config.c_max / 2.0 {
                d_loyalty -= config.loyalty_stress;
            }

            next[i] = Cell::citizen(CitizenData {
                stress: (d.stress + d_stress).clamp(0.0, config.c_max),
                fatigue: (d.fatigue + d_fatigue).clamp(0.0, config.f_max),
                loyalty: (d.loyalty + d_loyalty).clamp(0.0, config.l_max),
            });
        }
    }
}

fn conflict_field(grid: &Grid, config: &SimConfig) -> Vec<f32> {
    let mut out = vec![0.0; grid.cells.len()];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            let c = &grid.cells[i];
            if !matches!(c.state, CellState::Construction | CellState::Overlap) {
                continue;
            }
            if c.kind != Some(crate::sim::cell::ConstructionKind::Conflict) {
                continue;
            }
            let amount = config.stress_conflict_per_tick;
            for (nx, ny) in grid.cells_in_radius(x, y, config.conflict_radius) {
                out[grid.idx(nx, ny)] += amount;
            }
        }
    }
    out
}

/// Rule 4: citizens arranged into a pattern become a construction.
pub fn rule4_construction_entry(
    grid: &Grid,
    next: &mut [Cell],
    library: &PatternLibrary,
) {
    let detected = construction::detect(grid, library);
    let mut id = grid.max_construction_id() + 1;
    for d in detected {
        let mut cells = d.cells.clone();
        cells.sort_unstable();
        cells.dedup();
        if cells.iter().any(|&i| next[i].state != CellState::Citizen) {
            continue;
        }
        for &i in &cells {
            let data = grid.cells[i].citizen.unwrap_or(CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: 0.0,
            });
            next[i] = Cell::construction(d.kind, id, data);
        }
        id += 1;
    }
}

/// Rule 5: cells claimed by two or more patterns become `Overlap`.
pub fn rule5_overlap(grid: &Grid, next: &mut [Cell], library: &PatternLibrary) {
    let mut claims: std::collections::HashMap<usize, u8> = std::collections::HashMap::new();
    for d in construction::detect(grid, library) {
        for i in d.cells {
            *claims.entry(i).or_insert(0) += 1;
        }
    }
    for (i, n) in claims {
        if n < 2 {
            continue;
        }
        let id = grid.cells[i].construction_id.unwrap_or(grid.max_construction_id() + 1);
        let data = next[i].frozen.or(grid.cells[i].citizen).or(grid.cells[i].frozen).unwrap_or(
            CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: 0.0,
            },
        );
        next[i] = Cell::overlap(id, data);
    }
}

/// Rule 6: a construction whose shape no longer exists releases its citizens.
pub fn rule6_construction_exit(grid: &Grid, next: &mut [Cell], library: &PatternLibrary) {
    for i in construction::broken_constructions(grid, library) {
        let data = next[i]
            .frozen
            .or(grid.cells[i].frozen)
            .or(grid.cells[i].citizen)
            .unwrap_or(CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: 0.0,
            });
        next[i] = Cell::citizen(data);
    }
}

pub use anticell::{rule7_anticell_explosion, rule8_anticell_wave};