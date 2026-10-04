pub mod anticell;
pub mod cell;
pub mod citizen;
pub mod construction;
pub mod grid;
pub mod movement;
pub mod pattern;
pub mod resources;
pub mod rules;

pub use cell::{Cell, CellState, ConstructionKind};
pub use citizen::CitizenData;
pub use construction::Registry;
pub use grid::Grid;
pub use pattern::{Pattern, PatternLibrary};
pub use resources::{Resources, Stocks};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use anyhow::Result;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

use crate::config::{RenderConfig, SimConfig};

pub const DEFAULT_WIDTH: usize = 128;
pub const DEFAULT_HEIGHT: usize = 128;
pub const MAX_TICKS_PER_FRAME: usize = 6;
pub const CYCLE_HISTORY: usize = 8;
pub const CYCLE_REPEATS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimStatus {
    Running,
    Paused,
    GameOver,
    CycleDetected,
}

impl SimStatus {
    pub fn label(self) -> &'static str {
        match self {
            SimStatus::Running => "Running",
            SimStatus::Paused => "Paused",
            SimStatus::GameOver => "Game Over",
            SimStatus::CycleDetected => "Цикл обнаружен",
        }
    }
}

/// Painted cell kinds available from the toolbar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brush {
    Empty,
    Citizen,
    AntiCell,
    Stamp(ConstructionKind),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub generation: u64,
    pub n_citizen: usize,
    pub n_empty: usize,
    pub n_anticell: usize,
    pub n_overlap: usize,
    pub n_constructions: [usize; 5],
    pub mean_stress: f32,
    pub mean_fatigue: f32,
    pub mean_loyalty: f32,
    pub food: f32,
    pub water: f32,
    pub energy: f32,
}

/// Compact fingerprint of the whole grid, used for cycle detection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StateKey(u64);

impl StateKey {
    fn of_grid(grid: &Grid) -> Self {
        let mut h = DefaultHasher::new();
        grid.width.hash(&mut h);
        grid.height.hash(&mut h);
        for c in &grid.cells {
            c.state.hash(&mut h);
            c.kind.hash(&mut h);
            if let Some(d) = c.citizen {
                d.stress.to_bits().hash(&mut h);
                d.fatigue.to_bits().hash(&mut h);
                d.loyalty.to_bits().hash(&mut h);
            }
        }
        StateKey(h.finish())
    }
}

pub struct Sim {
    pub grid: Grid,
    pub config: SimConfig,
    pub render: RenderConfig,
    pub library: PatternLibrary,
    pub status: SimStatus,
    pub stats: Stats,
    pub generation: u64,
    pub cycle_length: Option<usize>,
    rng: ChaCha8Rng,
    scratch: Vec<Cell>,
    cycle_history: Vec<StateKey>,
    next_construction_id: u32,
}

impl Clone for Sim {
    fn clone(&self) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(self.config.seed);
        rng.set_stream(0);
        Self {
            grid: self.grid.clone(),
            config: self.config.clone(),
            render: self.render.clone(),
            library: self.library.clone(),
            status: self.status,
            stats: self.stats.clone(),
            generation: self.generation,
            cycle_length: self.cycle_length,
            rng,
            scratch: self.scratch.clone(),
            cycle_history: self.cycle_history.clone(),
            next_construction_id: self.next_construction_id,
        }
    }
}

pub fn default_library() -> PatternLibrary {
    PatternLibrary::from_embedded().unwrap_or_else(|_| PatternLibrary { patterns: Vec::new() })
}

impl Sim {
    pub fn new(width: usize, height: usize, config: SimConfig, library: PatternLibrary) -> Self {
        let render = RenderConfig::default();
        let grid = Grid::new(width, height);
        let mut sim = Self {
            grid,
            config,
            render,
            library,
            status: SimStatus::Paused,
            stats: Stats::default(),
            generation: 0,
            cycle_length: None,
            rng: ChaCha8Rng::seed_from_u64(0),
            scratch: Vec::new(),
            cycle_history: Vec::with_capacity(CYCLE_HISTORY),
            next_construction_id: 1,
        };
        sim.reset();
        sim
    }

    pub fn with_defaults(width: usize, height: usize) -> Self {
        let config = SimConfig::default();
        Self::new(width, height, config, default_library())
    }

    pub fn reset(&mut self) {
        self.grid = Grid::new(self.grid.width, self.grid.height);
        self.generation = 0;
        self.status = SimStatus::Paused;
        self.cycle_length = None;
        self.cycle_history.clear();
        self.next_construction_id = 1;
        self.rng = ChaCha8Rng::seed_from_u64(self.config.seed);
        self.refresh_stats();
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.grid = Grid::new(width, height);
        self.scratch = Vec::with_capacity(width * height);
        self.cycle_history.clear();
        self.cycle_length = None;
        self.status = SimStatus::Paused;
        self.generation = 0;
        self.refresh_stats();
    }

    pub fn save(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn load_json(&mut self, json: &str) -> Result<()> {
        let loaded: Sim = serde_json::from_str(json)?;
        self.config = loaded.config;
        self.render = loaded.render;
        self.rng = ChaCha8Rng::seed_from_u64(self.config.seed);
        self.generation = loaded.generation;
        self.cycle_history.clear();
        self.cycle_length = None;
        self.status = SimStatus::Paused;
        self.grid = loaded.grid;
        self.refresh_stats();
        Ok(())
    }

    pub fn generate_colony(&mut self, count: usize) {
        let mut idx: Vec<usize> = (0..self.grid.cells.len())
            .filter(|&i| self.grid.cells[i].state == CellState::Empty)
            .collect();
        let mut rng = ChaCha8Rng::seed_from_u64(self.config.seed ^ 0x5DEECE66D);
        use rand::seq::SliceRandom;
        idx.shuffle(&mut rng);
        for &i in idx.iter().take(count) {
            self.grid.cells[i] = Cell::citizen(CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: self.config.l_max / 2.0,
            });
        }
        self.refresh_stats();
    }

    pub fn paint(&mut self, x: usize, y: usize, brush: Brush) {
        if x >= self.grid.width || y >= self.grid.height {
            return;
        }
        let i = self.grid.idx(x, y);
        self.grid.cells[i] = match brush {
            Brush::Empty => Cell::empty(),
            Brush::Citizen => Cell::citizen(CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: self.config.l_max / 2.0,
            }),
            Brush::AntiCell => Cell::anticell(),
            Brush::Stamp(kind) => {
                if self.grid.cells[i].state != CellState::Citizen {
                    return;
                }
                let data = self.grid.cells[i].citizen.clone().unwrap_or(CitizenData {
                    stress: 0.0,
                    fatigue: 0.0,
                    loyalty: 0.0,
                });
                let id = self.next_construction_id;
                self.next_construction_id += 1;
                Cell::construction(kind, id, data)
            }
        };
        self.refresh_stats();
    }

    pub fn stamp(&mut self, name: &str, x: usize, y: usize, rotation: usize) {
        let pattern = match self
            .library
            .patterns
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.rotations()[rotation % 4].clone())
        {
            Some(p) => p,
            None => return,
        };
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        for (dx, dy) in &pattern.cells {
            min_x = min_x.min(*dx as i32);
            min_y = min_y.min(*dy as i32);
        }
        let mut cells = Vec::with_capacity(pattern.cells.len());
        for (dx, dy) in &pattern.cells {
            let px = x as i32 + (*dx as i32 - min_x);
            let py = y as i32 + (*dy as i32 - min_y);
            if px < 0 || py < 0 || px >= self.grid.width as i32 || py >= self.grid.height as i32 {
                continue;
            }
            cells.push(self.grid.idx(px as usize, py as usize));
        }
        if cells.len() != pattern.cells.len() {
            return;
        }
        let id = self.next_construction_id;
        self.next_construction_id += 1;
        for i in cells {
            let data = self.grid.cells[i].citizen.clone().unwrap_or(CitizenData {
                stress: 0.0,
                fatigue: 0.0,
                loyalty: 0.0,
            });
            self.grid.cells[i] = Cell::construction(pattern.kind, id, data);
        }
        self.refresh_stats();
    }

    /// One synchronous generation. Returns `false` once the sim has stopped.
    pub fn step(&mut self) -> bool {
        if matches!(self.status, SimStatus::GameOver | SimStatus::CycleDetected) {
            return false;
        }
        self.generation += 1;
        self.tick();
        self.refresh_stats();

        if self.stats.n_citizen == 0 {
            self.status = SimStatus::GameOver;
            return false;
        }

        let key = StateKey::of_grid(&self.grid);
        if let Some(pos) = self.cycle_history.iter().position(|k| *k == key) {
            self.cycle_length = Some(self.cycle_history.len() - pos);
            self.status = SimStatus::CycleDetected;
            return false;
        }
        self.cycle_history.push(key);
        if self.cycle_history.len() > CYCLE_HISTORY {
            self.cycle_history.remove(0);
        }
        true
    }

    fn tick(&mut self) {
        let ctx = self.counters();
        self.scratch.clear();
        self.scratch.resize(self.grid.cells.len(), Cell::empty());

        rules::rule1_hunger(&self.grid, &mut self.scratch);
        rules::rule2_stress(&self.grid, &mut self.scratch, &self.config);
        rules::rule3_citizen_update(&self.grid, &mut self.scratch, &self.config, &ctx);
        rules::rule4_construction_entry(&self.grid, &mut self.scratch, &self.library);
        rules::rule5_overlap(&self.grid, &mut self.scratch, &self.library);
        rules::rule6_construction_exit(&self.grid, &mut self.scratch);
        rules::rule7_anticell_explosion(&self.grid, &mut self.scratch);
        let exploding = anticell::explosion_sources(&self.grid);
        anticell::rule8_anticell_wave(&self.grid, &mut self.scratch, &self.config, &exploding);
        movement::rule9_citizen_move(&self.grid, &mut self.scratch, &self.config, &mut self.rng);

        std::mem::swap(&mut self.grid.cells, &mut self.scratch);
    }

    fn counters(&self) -> rules::GlobalCtx {
        rules::compute_global_counters(&self.grid, &self.config)
    }

    pub fn refresh_stats(&mut self) {
        let ctx = self.counters();
        let mut n_anticell = 0;
        let mut n_overlap = 0;
        let mut n_empty = 0;
        let mut n_constructions = [0usize; 5];
        for c in &self.grid.cells {
            match c.state {
                CellState::Empty => n_empty += 1,
                CellState::AntiCell => n_anticell += 1,
                CellState::Overlap => n_overlap += 1,
                CellState::Construction => {
                    if let Some(kind) = c.kind {
                        n_constructions[kind.index()] += 1;
                    }
                }
                CellState::Citizen => {}
            }
        }
        let (mut s, mut f, mut l, mut n) = (0.0f32, 0.0f32, 0.0f32, 0usize);
        for c in &self.grid.cells {
            if c.state == CellState::Citizen {
                if let Some(d) = c.citizen {
                    s += d.stress;
                    f += d.fatigue;
                    l += d.loyalty;
                    n += 1;
                }
            }
        }
        let inv = if n == 0 { 0.0 } else { 1.0 / n as f32 };
        self.stats = Stats {
            generation: self.generation,
            n_citizen: ctx.n_citizen,
            n_empty,
            n_anticell,
            n_overlap,
            n_constructions,
            mean_stress: s * inv,
            mean_fatigue: f * inv,
            mean_loyalty: l * inv,
            food: ctx.food,
            water: ctx.water,
            energy: ctx.energy,
        };
    }
}

impl Serialize for Sim {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("Sim", 6)?;
        st.serialize_field("grid", &self.grid)?;
        st.serialize_field("config", &self.config)?;
        st.serialize_field("render", &self.render)?;
        st.serialize_field("generation", &self.generation)?;
        st.serialize_field("next_construction_id", &self.next_construction_id)?;
        st.end()
    }
}

impl<'de> Deserialize<'de> for Sim {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            grid: Grid,
            config: SimConfig,
            render: RenderConfig,
            #[serde(default)]
            generation: u64,
            #[serde(default = "one")]
            next_construction_id: u32,
        }
        fn one() -> u32 {
            1
        }
        let raw = Raw::deserialize(d)?;
        let mut sim = Sim::new(
            raw.grid.width,
            raw.grid.height,
            raw.config,
            default_library(),
        );
        sim.grid = raw.grid;
        sim.generation = raw.generation;
        sim.next_construction_id = raw.next_construction_id;
        sim.status = SimStatus::Paused;
        sim.refresh_stats();
        Ok(sim)
    }
}