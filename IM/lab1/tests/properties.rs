//! Property tests for invariants that must hold for any input (§10.3).

use lab1::config::SimConfig;
use lab1::sim::{Brush, CellState, Sim, SimStatus};
use proptest::prelude::*;

/// A small script of edits, used to build varied starting grids.
fn grid_script() -> impl Strategy<Value = Vec<(u8, u8, u8)>> {
    prop::collection::vec((0u8..3, 0u8..24, 0u8..24), 0..40)
}

fn sim_from(script: &[(u8, u8, u8)], cfg: SimConfig) -> Sim {
    let mut sim = Sim::new(24, 24, cfg, lab1::sim::default_library());
    for &(what, x, y) in script {
        let brush = match what {
            0 => Brush::Citizen,
            1 => Brush::AntiCell,
            _ => Brush::Empty,
        };
        sim.paint(x as usize, y as usize, brush);
    }
    sim
}

proptest! {
    #[test]
    fn grid_size_is_preserved(script in grid_script()) {
        let mut sim = sim_from(&script, SimConfig::default());
        let n = sim.grid.cells.len();
        for _ in 0..25 {
            if !sim.step() {
                break;
            }
        }
        prop_assert_eq!(sim.grid.cells.len(), n);
        prop_assert_eq!(sim.grid.width, 24);
        prop_assert_eq!(sim.grid.height, 24);
    }

    #[test]
    fn citizen_stats_stay_in_bounds(script in grid_script()) {
        let cfg = SimConfig::default();
        let (c_max, f_max, l_max) = (cfg.c_max, cfg.f_max, cfg.l_max);
        let mut sim = sim_from(&script, cfg);
        for _ in 0..25 {
            if !sim.step() {
                break;
            }
            for c in &sim.grid.cells {
                if let Some(d) = c.citizen {
                    prop_assert!((0.0..=c_max).contains(&d.stress));
                    prop_assert!((0.0..=f_max).contains(&d.fatigue));
                    prop_assert!((0.0..=l_max).contains(&d.loyalty));
                }
            }
        }
    }

    #[test]
    fn construction_cells_are_well_formed(script in grid_script()) {
        let mut sim = sim_from(&script, SimConfig::default());
        for _ in 0..25 {
            if !sim.step() {
                break;
            }
            for c in &sim.grid.cells {
                if c.state == CellState::Construction {
                    prop_assert!(c.kind.is_some());
                    prop_assert!(c.construction_id.is_some());
                }
            }
        }
    }

    #[test]
    fn determinism_holds_for_any_script(script in grid_script()) {
        let run = || {
            let mut sim = sim_from(&script, SimConfig::default());
            for _ in 0..40 {
                if !sim.step() {
                    break;
                }
            }
            sim.grid.cells.clone()
        };
        prop_assert_eq!(run(), run());
    }

    #[test]
    fn stopped_sim_is_frozen(script in grid_script()) {
        let mut sim = sim_from(&script, SimConfig::default());
        let mut stopped = false;
        for _ in 0..40 {
            if !sim.step() {
                stopped = true;
                break;
            }
        }
        if stopped {
            let gen = sim.generation;
            let snapshot = sim.grid.cells.clone();
            for _ in 0..5 {
                prop_assert!(!sim.step());
            }
            prop_assert_eq!(sim.generation, gen);
            prop_assert_eq!(sim.grid.cells, snapshot);
        }
    }
}

#[test]
fn empty_grid_never_panics() {
    let mut sim = Sim::with_defaults(32, 32);
    assert!(!sim.step());
    assert_eq!(sim.status, SimStatus::GameOver);
}
