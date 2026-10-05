use std::collections::HashMap;

use crate::config::SimConfig;
use crate::sim::cell::{Cell, CellState, CitizenData, ConstructionKind};
use crate::sim::construction::{self, OVERLAP_MULTIPLIER};
use crate::sim::grid::Grid;
use crate::sim::pattern::PatternLibrary;

/// Per-generation aggregate state, computed from the *previous* generation.
#[derive(Clone, Debug, Default)]
pub struct GlobalCtx {
    pub n_citizen: usize,
    pub n_population: usize,
    pub food: f32,
    pub water: f32,
    pub energy: f32,
    pub hungry: bool,
    pub thirsty: bool,
    pub dark: bool,
    pub over: bool,
}

/// One connected group of construction cells.
pub struct Complex {
    /// Index of the first cell, used as a stable id for its multiplier.
    pub root: usize,
    pub representative: usize,
    pub cells: Vec<usize>,
    pub overlapping: bool,
}

/// Partitions every construction/overlap cell into face-connected complexes.
fn complexes(grid: &Grid) -> (Vec<Complex>, HashMap<usize, f32>) {
    let n = grid.cells.len();
    let mut comp_of: Vec<Option<usize>> = vec![None; n];
    let mut comps: Vec<Complex> = Vec::new();

    for seed in 0..n {
        if comp_of[seed].is_some() {
            continue;
        }
        if !matches!(
            grid.cells[seed].state,
            CellState::Construction | CellState::Overlap
        ) {
            comp_of[seed] = Some(usize::MAX);
            continue;
        }
        let idx = comps.len();
        let mut stack = vec![seed];
        let mut cells = Vec::new();
        let mut overlapping = false;
        let mut representative = usize::MAX;
        while let Some(i) = stack.pop() {
            if comp_of[i].is_some() {
                continue;
            }
            comp_of[i] = Some(idx);
            if !matches!(
                grid.cells[i].state,
                CellState::Construction | CellState::Overlap
            ) {
                continue;
            }
            if grid.cells[i].state == CellState::Overlap {
                overlapping = true;
            }
            if representative == usize::MAX {
                representative = i;
            }
            cells.push(i);
            let (x, y) = grid.xy(i);
            for (nx, ny) in grid.neighbors4(x, y) {
                let j = grid.idx(nx, ny);
                if comp_of[j].is_none()
                    && matches!(
                        grid.cells[j].state,
                        CellState::Construction | CellState::Overlap
                    )
                {
                    stack.push(j);
                }
            }
        }
        comps.push(Complex {
            root: seed,
            representative,
            cells,
            overlapping,
        });
    }

    let mut mult = HashMap::new();
    for c in &comps {
        mult.insert(
            c.root,
            if c.overlapping {
                OVERLAP_MULTIPLIER
            } else {
                1.0
            },
        );
    }
    (comps, mult)
}

pub fn compute_global_counters(
    grid: &Grid,
    config: &SimConfig,
    library: &PatternLibrary,
) -> GlobalCtx {
    let mut n_citizen = 0usize;
    let mut n_population = 0usize;
    for c in &grid.cells {
        if c.state == CellState::Citizen {
            n_citizen += 1;
        }
        if matches!(c.state, CellState::Construction | CellState::Overlap)
            && c.kind == Some(ConstructionKind::Population)
        {
            n_population += 1;
        }
    }

    // §6.6: a complex is a connected component of construction cells joined by
    // face adjacency. Production is counted once per complex, and if any cell
    // of a complex is `Overlap` the whole complex gets the x1.5 multiplier.
    let (components, mult) = complexes(grid);

    let mut food = 0.0f32;
    let mut water = 0.0f32;
    let mut energy = 0.0f32;
    for comp in &components {
        let m = mult.get(&comp.root).copied().unwrap_or(1.0);
        // A complex contributes the output of the patterns that formed it,
        // which is identified by the kind of its representative cell.
        {
            let i = comp.representative;
            let kind = match grid.cells[i].kind {
                Some(k) => k,
                None => continue,
            };
            for p in library.patterns.iter().filter(|p| p.kind == kind) {
                food += (p.produce.food - p.consume.food) * m;
                water += (p.produce.water - p.consume.water) * m;
                energy += (p.produce.energy - p.consume.energy) * m;
            }
        }
    }

    GlobalCtx {
        n_citizen,
        n_population,
        food,
        water,
        energy,
        hungry: food < config.food_need(n_citizen),
        thirsty: water < config.water_need(n_citizen),
        dark: energy < config.energy_need(n_citizen),
        over: n_citizen > config.population_max as usize,
    }
}

/// Rule 1: a citizen with zero loyalty becomes an anti-cell.
pub fn rule1_hunger(grid: &Grid, next: &mut [Cell]) {
    for (i, c) in grid.cells.iter().enumerate() {
        if c.state != CellState::Citizen {
            continue;
        }
        if c.citizen.is_some_and(|d| d.loyalty <= 0.0) {
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
        if c.citizen.is_some_and(|d| d.stress >= config.c_max) {
            next[i] = Cell::anticell();
        }
    }
}

/// Rule 3: update stress / fatigue / loyalty for every surviving citizen.
pub fn rule3_citizen_update(grid: &Grid, next: &mut [Cell], config: &SimConfig, ctx: &GlobalCtx) {
    let conflict_sum = conflict_field(grid, config);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            if grid.cells[i].state != CellState::Citizen || next[i].state == CellState::AntiCell {
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
                    let c = grid.get(nx, ny);
                    c.state == CellState::Construction
                        && c.kind == Some(ConstructionKind::Population)
                });
            let d_fatigue = config.fatigue_per_tick - config.fatigue_rest * f32::from(has_rest);

            let surplus = !ctx.hungry && !ctx.thirsty && !ctx.dark;
            let mut d_loyalty = if surplus {
                config.loyalty_surplus
            } else {
                -config.loyalty_deficit
            };
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
            if c.kind != Some(ConstructionKind::Conflict) {
                continue;
            }
            for (nx, ny) in grid.cells_in_radius(x, y, config.conflict_radius) {
                out[grid.idx(nx, ny)] += config.stress_conflict_per_tick;
            }
        }
    }
    out
}

/// Rule 4: citizens arranged into a pattern become a construction.
pub fn rule4_construction_entry(grid: &Grid, next: &mut [Cell], library: &PatternLibrary) {
    let mut next_id = grid.max_construction_id() + 1;
    for d in construction::detect(grid, library) {
        let mut cells = d.cells.clone();
        cells.sort_unstable();
        cells.dedup();
        if cells.iter().any(|&i| next[i].state != CellState::Citizen) {
            continue;
        }
        let id = next_id;
        next_id += 1;
        for &i in &cells {
            let data = next[i]
                .citizen
                .or(grid.cells[i].citizen)
                .unwrap_or(CitizenData {
                    stress: 0.0,
                    fatigue: 0.0,
                    loyalty: 0.0,
                });
            next[i] = Cell::construction(d.kind, id, data);
        }
    }
}

/// Rule 5: cells claimed by two or more patterns become `Overlap`.
pub fn rule5_overlap(grid: &Grid, next: &mut [Cell], library: &PatternLibrary) {
    let mut claims: HashMap<usize, u8> = HashMap::new();
    for d in construction::detect(grid, library) {
        for i in d.cells {
            *claims.entry(i).or_insert(0) += 1;
        }
    }
    for (i, n) in claims {
        if n < 2 {
            continue;
        }
        let id = grid.cells[i]
            .construction_id
            .unwrap_or_else(|| grid.max_construction_id() + 1);
        let data = next[i]
            .frozen
            .or(grid.cells[i].frozen)
            .or(grid.cells[i].citizen)
            .unwrap_or(CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: 0.0,
            });
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

pub use crate::sim::anticell::{rule7_anticell_explosion, rule8_anticell_wave};
