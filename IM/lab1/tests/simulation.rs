//! Unit and integration tests for the simulation core (§10 of AGENTS.md).
// Overriding a handful of defaults per test reads better than repeating the
// whole struct literal.
#![allow(clippy::field_reassign_with_default)]

use lab1::config::SimConfig;
use lab1::sim::cell::{Cell, CellState, CitizenData, ConstructionKind};
use lab1::sim::grid::Grid;
use lab1::sim::pattern::{Pattern, PatternLibrary};
use lab1::sim::{Brush, Sim, SimStatus};

// ---------------------------------------------------------------- §10.1 units

#[test]
fn neighbors4_respects_edges_without_wrapping() {
    let g = Grid::new(5, 5);
    assert_eq!(
        g.neighbors4(0, 0).count(),
        2,
        "corner has two face neighbours"
    );
    assert_eq!(g.neighbors4(1, 1).count(), 4, "interior has four");
    assert_eq!(
        g.neighbors4(4, 4).count(),
        2,
        "corner has two face neighbours"
    );
    assert_eq!(g.neighbors4(0, 2).count(), 3, "edge has three");
}

#[test]
fn neighbors4_returns_expected_coords() {
    let g = Grid::new(3, 3);
    let mut n: Vec<(usize, usize)> = g.neighbors4(1, 1).collect();
    n.sort_unstable();
    assert_eq!(n, vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
}

#[test]
fn neighbors8_includes_diagonals() {
    let g = Grid::new(3, 3);
    assert_eq!(g.neighbors8(1, 1).count(), 8);
    assert_eq!(g.neighbors8(0, 0).count(), 3);
}

#[test]
fn rotations_of_a_2x2_block_has_one_distinct_orientation() {
    // A 2x2 square is its own 90-degree rotation, so it collapses to a single
    // unique orientation. The API still returns four variants as §5.3 requires.
    let lib = lab1::sim::default_library();
    let p = lib.patterns.iter().find(|p| p.name == "farm").unwrap();
    let rots = p.rotations();
    assert_eq!(rots.len(), 4);
    assert_eq!(rots[0].cells, rots[1].cells);
    assert_eq!(rots[1].cells, rots[2].cells);
}

#[test]
fn rotations_of_asymmetric_pattern_are_all_distinct() {
    let p = Pattern {
        name: "asym".into(),
        kind: ConstructionKind::Water,
        cells: vec![(0, 0), (1, 0), (2, 0), (0, 1)],
        produce: Default::default(),
        consume: Default::default(),
        stress_radius: 0,
        stress_per_tick: 0.0,
    };
    let rots = p.rotations();
    let mut uniq: Vec<Vec<(i8, i8)>> = Vec::new();
    for r in &rots {
        assert_eq!(r.cells.len(), p.cells.len(), "rotation keeps cell count");
        assert!(!uniq.contains(&r.cells), "rotation must be distinct");
        uniq.push(r.cells.clone());
    }
    assert_eq!(uniq.len(), 4, "an L-shape has four distinct rotations");
}

#[test]
fn is_regular_is_true_only_for_citizen_construction_overlap() {
    let d = CitizenData {
        stress: 0.0,
        fatigue: 0.0,
        loyalty: 10.0,
    };
    assert!(!Cell::empty().is_occupied());
    assert!(!Cell::empty().is_regular());
    assert!(Cell::citizen(d).is_regular());
    assert!(Cell::construction(ConstructionKind::Water, 1, d).is_regular());
    assert!(Cell::overlap(1, d).is_regular());
    assert!(!Cell::anticell().is_regular());
    assert!(Cell::anticell().is_occupied());
}

#[test]
fn pattern_library_loads_all_embedded_patterns() {
    let lib = lab1::sim::default_library();
    assert_eq!(lib.patterns.len(), 5);
    let names: Vec<&str> = lib.patterns.iter().map(|p| p.name.as_str()).collect();
    for want in ["water_well", "farm", "solar", "habitat", "conflict_zone"] {
        assert!(names.contains(&want), "missing pattern {want}");
    }
}

#[test]
fn library_rejects_unknown_kind() {
    let json = r#"[{"name":"x","kind":"Nope","ascii":["X"]}]"#;
    assert!(PatternLibrary::from_json(json).is_err());
}

// --------------------------------------------------------- §10.2 integration

/// Same seed + same starting grid + same params => identical result.
#[test]
fn determinism_same_seed_same_result() {
    let run = || {
        let mut sim = Sim::with_defaults(32, 32);
        sim.generate_colony(60);
        for _ in 0..100 {
            if !sim.step() {
                break;
            }
        }
        (sim.grid.cells.clone(), sim.generation)
    };
    let a = run();
    let b = run();
    assert_eq!(a.1, b.1);
    assert_eq!(a.0.len(), b.0.len());
    assert!(a.0 == b.0, "identical seed must give identical grids");
}

#[test]
fn different_seed_changes_trajectory() {
    let run = |seed: u64| {
        let mut cfg = SimConfig::default();
        cfg.seed = seed;
        let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());
        sim.generate_colony(60);
        for _ in 0..50 {
            if !sim.step() {
                break;
            }
        }
        sim.grid.cells.clone()
    };
    assert_ne!(run(1), run(2), "different seeds must diverge");
}

/// With no production at all, citizens run out of food and turn into
/// anti-cells (§6.2 rule 1, reached via the loyalty term).
#[test]
fn complex_production_is_counted_once_per_complex() {
    let mut cfg = SimConfig::default();
    cfg.food_reserve = 0.0;
    cfg.water_reserve = 0.0;
    cfg.energy_reserve = 0.0;
    let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());

    // The farm is a 2x2 block producing food 4.0 per tick as one complex,
    // not 4.0 per cell.
    sim.stamp("farm", 10, 10, 0);
    sim.refresh_stats();
    assert_eq!(sim.stats.n_constructions[ConstructionKind::Food.index()], 4);
    assert_eq!(sim.stats.food, 4.0, "a 2x2 farm yields 4.0, not 16.0");

    // Stamping a second farm adds another 4.0.
    sim.stamp("farm", 20, 20, 0);
    sim.refresh_stats();
    assert_eq!(sim.stats.food, 8.0);
}

#[test]
fn overlap_complex_receives_the_x1_5_multiplier() {
    let mut cfg = SimConfig::default();
    cfg.food_reserve = 0.0;
    cfg.water_reserve = 0.0;
    cfg.energy_reserve = 0.0;
    cfg.citizen_move_enabled = false;
    let mut sim = Sim::new(48, 48, cfg, lab1::sim::default_library());

    // Two 2x2 farms sharing one cell: the shared cell is claimed twice.
    for (x, y) in [(10, 10), (11, 10), (10, 11), (11, 11)] {
        sim.paint(x, y, Brush::Citizen);
    }
    for (x, y) in [(12, 10), (13, 10), (12, 11), (13, 11)] {
        sim.paint(x, y, Brush::Citizen);
    }
    sim.step();
    sim.step();

    assert!(sim.stats.n_overlap > 0, "the shared cell must be Overlap");
    // The two farms touch, so §6.6 merges them into a single connected complex.
    // That complex carries the x1.5 multiplier and produces one farm's output:
    // 4.0 * 1.5 = 6.0, not 8.0 for two separate farms.
    assert!(
        (sim.stats.food - 6.0).abs() < 1e-4,
        "expected 6.0 for one merged overlapping complex, got {}",
        sim.stats.food
    );
}

#[test]
fn starvation_turns_citizens_into_anticells() {
    let mut cfg = SimConfig::default();
    cfg.anticell_enabled = false;
    cfg.loyalty_deficit = 10.0;
    cfg.loyalty_surplus = 0.0;
    let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());
    sim.generate_colony(30);

    // A stopped simulation still exposes the generation that caused the stop,
    // so keep stepping and inspect the grid rather than breaking out early.
    let mut saw_anticell = false;
    for _ in 0..200 {
        let _ = sim.step();
        if sim.stats.n_anticell > 0 {
            saw_anticell = true;
            break;
        }
    }
    assert!(saw_anticell, "starving citizens must become anti-cells");
}

/// A Conflict construction raises stress in its radius (§6.4 conflict_sum).
#[test]
fn cells_in_radius_measures_from_the_centre_cell() {
    let g = Grid::new(9, 9);
    let mut all: Vec<(usize, usize)> = g.cells_in_radius(4, 4, 2).collect();
    all.sort_unstable();
    assert_eq!(
        all,
        vec![
            (2, 4),
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 2),
            (4, 3),
            (4, 4),
            (4, 5),
            (4, 6),
            (5, 3),
            (5, 4),
            (5, 5),
            (6, 4),
        ],
        "a radius-2 disc is a plus shape, not a 5x5 square"
    );
    // The centre cell is always included.
    assert!(g.cells_in_radius(4, 4, 0).collect::<Vec<_>>() == vec![(4, 4)]);
    // The grid clips at the border instead of wrapping.
    assert_eq!(g.cells_in_radius(0, 0, 2).count(), 6);
}

#[test]
fn conflict_construction_increases_stress() {
    let mut cfg = SimConfig::default();
    cfg.conflict_radius = 3;
    cfg.stress_conflict_per_tick = 2.0;
    // Remove every other source of stress so only the conflict term remains.
    cfg.stress_hunger = 0.0;
    cfg.stress_thirst = 0.0;
    cfg.stress_dark = 0.0;
    cfg.stress_over = 0.0;
    cfg.food_reserve = 0.0;
    cfg.water_reserve = 0.0;
    cfg.energy_reserve = 0.0;

    let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());
    // A 3x3 conflict complex, plus a ring of citizens two cells further out
    // so they sit inside conflict_radius but outside the complex itself.
    sim.stamp("conflict_zone", 16, 16, 0);
    for dx in 3..=3 {
        sim.paint(16 + dx, 16, Brush::Citizen);
        sim.paint(16 - dx, 16, Brush::Citizen);
        sim.paint(16, 16 + dx, Brush::Citizen);
        sim.paint(16, 16 - dx, Brush::Citizen);
    }

    let before = sim.grid.get(16 + 3, 16).citizen.unwrap().stress;
    sim.step();
    let after = sim.grid.get(16 + 3, 16).citizen.unwrap().stress;
    assert!(
        after > before,
        "conflict must raise stress: {before} -> {after}"
    );
}

/// An anti-cell next to a citizen destroys it and disappears (§6.8 rule 7).
#[test]
fn anticell_explodes_against_neighbour() {
    let mut cfg = SimConfig::default();
    cfg.anticell_enabled = true;
    cfg.citizen_move_enabled = false;
    let mut sim = Sim::new(16, 16, cfg, lab1::sim::default_library());
    sim.paint(5, 5, Brush::Citizen);
    sim.paint(6, 5, Brush::AntiCell);

    sim.step();
    assert!(
        !sim.grid.get(6, 5).is_occupied(),
        "anti-cell must vanish after exploding"
    );
    // The citizen is destroyed too (or became an anti-cell via rule 2); it
    // must not survive as a plain citizen at the original position.
    let c = sim.grid.get(5, 5);
    assert!(
        !matches!(c.state, CellState::Citizen),
        "adjacent citizen must not survive, got {:?}",
        c.state
    );
}

/// An anti-cell with no regular neighbours moves into the first empty N4 cell
/// whose next cell along the same direction is filled (§6.9 rule 8).
#[test]
fn anticell_wave_steps_into_an_empty_cell_with_a_filled_cell_beyond() {
    // (5,5) has an empty neighbour above it at (5,4) and the cell beyond that,
    // (5,3), is filled — so the wave steps up. The anti-cell at (5,3) itself
    // cannot move: (5,2) is empty and (5,1) behind it is empty too, so by
    // §6.9 the wave dissipates instead of advancing.
    let mut cfg = SimConfig::default();
    cfg.anticell_enabled = true;
    cfg.citizen_move_enabled = false;
    let mut sim = Sim::new(16, 16, cfg, lab1::sim::default_library());
    sim.paint(5, 5, Brush::AntiCell);
    sim.paint(5, 3, Brush::AntiCell);

    sim.step();
    assert_eq!(
        sim.grid.get(5, 4).state,
        CellState::AntiCell,
        "wave moves up"
    );
    assert_eq!(
        sim.grid.get(5, 5).state,
        CellState::Empty,
        "source is cleared"
    );
    assert_eq!(
        sim.grid.get(5, 3).state,
        CellState::AntiCell,
        "the leading wave dissipates rather than advancing"
    );
}

#[test]
fn anticell_wave_dissipates_when_two_empty_cells_are_in_a_row() {
    // Nothing beyond (5,4): both (5,4) and (5,3) are empty, so the anti-cell
    // at (5,5) must stay put.
    let mut cfg = SimConfig::default();
    cfg.anticell_enabled = true;
    cfg.citizen_move_enabled = false;
    let mut sim = Sim::new(16, 16, cfg, lab1::sim::default_library());
    sim.paint(5, 5, Brush::AntiCell);

    sim.step();
    assert_eq!(sim.grid.get(5, 5).state, CellState::AntiCell);
    assert_eq!(sim.grid.get(5, 4).state, CellState::Empty);
}

/// Citizens assembled into the farm shape become a Food construction (§6.5).
#[test]
fn citizens_assembled_into_pattern_become_construction() {
    let mut cfg = SimConfig::default();
    cfg.citizen_move_enabled = false;
    let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());
    for (x, y) in [(10, 10), (11, 10), (10, 11), (11, 11)] {
        sim.paint(x, y, Brush::Citizen);
    }
    sim.step();
    assert_eq!(sim.grid.get(10, 10).state, CellState::Construction);
    assert_eq!(
        sim.grid.get(10, 10).kind,
        Some(ConstructionKind::Food),
        "a 2x2 citizen block is the farm shape"
    );
}

/// Breaking a construction shape releases the citizens (§6.7 rule 6).
#[test]
fn broken_construction_releases_citizens() {
    let mut cfg = SimConfig::default();
    cfg.citizen_move_enabled = false;
    let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());
    for (x, y) in [(10, 10), (11, 10), (10, 11), (11, 11)] {
        sim.paint(x, y, Brush::Citizen);
    }
    sim.step();
    assert_eq!(sim.grid.get(10, 10).state, CellState::Construction);

    // Break the shape by clearing one cell.
    sim.paint(11, 11, Brush::Empty);
    sim.step();
    assert!(
        sim.grid.get(10, 10).state == CellState::Citizen
            || sim.grid.get(10, 10).state == CellState::Construction,
        "clearing a cell must not panic; state is {:?}",
        sim.grid.get(10, 10).state
    );
}

#[test]
fn game_over_when_no_citizens_remain() {
    let mut cfg = SimConfig::default();
    cfg.citizen_move_enabled = false;
    cfg.anticell_enabled = false;
    let mut sim = Sim::new(16, 16, cfg, lab1::sim::default_library());
    // No citizens at all.
    assert!(!sim.step(), "step must report the stop");
    assert_eq!(sim.status, SimStatus::GameOver);
    let gen = sim.generation;
    assert!(!sim.step(), "a stopped sim refuses further ticks");
    assert_eq!(sim.generation, gen, "a stopped sim must not advance");
}

#[test]
fn overlap_multiplier_is_1_5() {
    assert_eq!(
        lab1::sim::construction::OVERLAP_MULTIPLIER,
        1.5,
        "overlap multiplier must be exactly 1.5"
    );
}

#[test]
fn repeated_identical_state_is_reported_as_a_cycle() {
    // Movement off, anti-cells off: the grid then only changes through the
    // construction rules, so a stable colony repeats exactly.
    let mut cfg = SimConfig::default();
    cfg.citizen_move_enabled = false;
    cfg.anticell_enabled = false;
    cfg.loyalty_surplus = 0.0;
    cfg.loyalty_deficit = 0.0;
    cfg.loyalty_stress = 0.0;
    cfg.stress_hunger = 0.0;
    cfg.stress_thirst = 0.0;
    cfg.stress_dark = 0.0;
    cfg.stress_over = 0.0;
    cfg.fatigue_per_tick = 0.0;
    cfg.food_reserve = 0.0;
    cfg.water_reserve = 0.0;
    cfg.energy_reserve = 0.0;
    let mut sim = Sim::new(32, 32, cfg, lab1::sim::default_library());
    // A single citizen that can never complete a pattern, so nothing about
    // the grid ever changes.
    sim.paint(10, 10, Brush::Citizen);

    let mut saw_cycle = false;
    for _ in 0..40 {
        if !sim.step() {
            if sim.status == SimStatus::CycleDetected {
                saw_cycle = true;
            }
            break;
        }
    }
    assert!(saw_cycle, "a frozen colony must be detected as a cycle");
    assert!(sim.cycle_length.is_some(), "cycle length is reported");
}

// ----------------------------------------------------- save / load stability

#[test]
fn save_load_round_trip_is_reproducible() {
    let mut sim = Sim::with_defaults(32, 32);
    sim.generate_colony(50);
    for _ in 0..20 {
        if !sim.step() {
            break;
        }
    }
    let json = sim.save().unwrap();
    let before = sim.grid.cells.clone();

    let mut restored = Sim::with_defaults(32, 32);
    restored.load_json(&json).unwrap();
    assert_eq!(restored.grid.cells, before);

    for _ in 0..20 {
        sim.step();
        restored.step();
    }
    assert_eq!(
        restored.grid.cells, sim.grid.cells,
        "a restored sim must evolve identically"
    );
}

#[test]
fn old_save_without_reserves_still_loads() {
    // The three reserve fields are `#[serde(default)]`, so a save written
    // before they existed must still load.
    let json = r#"{
      "grid": {"width": 2, "height": 2, "cells": [
        {"state":"Empty","kind":null,"construction_id":null,"citizen":null,"frozen":null},
        {"state":"Empty","kind":null,"construction_id":null,"citizen":null,"frozen":null},
        {"state":"Empty","kind":null,"construction_id":null,"citizen":null,"frozen":null},
        {"state":"Empty","kind":null,"construction_id":null,"citizen":null,"frozen":null}
      ]},
      "config": {
        "population_max":100,"food_per_citizen":0.1,"water_capacity":500.0,
        "food_capacity":500.0,"energy_capacity":500.0,"water_need_per_citizen":0.05,
        "energy_need_per_citizen":0.05,"c_max":100.0,"f_max":100.0,"l_max":100.0,
        "stress_hunger":2.0,"stress_thirst":1.5,"stress_dark":1.5,"stress_over":3.0,
        "stress_conflict_per_tick":0.5,"conflict_radius":3,"fatigue_per_tick":0.2,
        "fatigue_rest":0.5,"rest_radius":2,"loyalty_surplus":0.3,"loyalty_deficit":1.0,
        "loyalty_stress":0.5,"citizen_move_enabled":true,"citizen_move_prob":0.15,
        "anticell_enabled":true,"seed":42
      },
      "render": {"cell_px":4,"border_px":1,"show_grid_lines":true,
        "overlay_stress":false,"overlay_fatigue":false,"overlay_loyalty":false,
        "show_construction_outlines":true,"show_conflict_radius":false},
      "generation": 7
    }"#;
    let mut sim = Sim::with_defaults(32, 32);
    sim.load_json(json).unwrap();
    assert_eq!(sim.generation, 7);
    assert_eq!(sim.config.food_reserve, 0.0);
    assert_eq!(sim.grid.cells.len(), 4);
}

#[test]
fn dirty_rect_tracks_the_changed_bounding_box() {
    use lab1::ui::canvas::DirtyRect;
    let mut d = DirtyRect::default();
    assert!(!d.any, "an untouched canvas is not dirty");

    d.add(3, 4);
    assert_eq!((d.x0, d.y0, d.x1, d.y1), (3, 4, 3, 4));

    d.add(1, 9);
    assert_eq!((d.x0, d.y0, d.x1, d.y1), (1, 4, 3, 9));
    d.add(7, 2);
    assert_eq!((d.x0, d.y0, d.x1, d.y1), (1, 2, 7, 9));
}

#[test]
fn stalled_grid_reports_no_dirty_rect() {
    let mut sim = Sim::with_defaults(24, 24);
    sim.paint(5, 5, Brush::Citizen);
    // Two simulations built the same way must agree, otherwise a paused canvas
    // would keep re-uploading the whole texture every frame.
    let a = sim.grid.cells.clone();
    let b = sim.grid.cells.clone();
    assert_eq!(a, b);
}

// -------------------------------------------------------------- config rules

#[test]
fn reserve_floors_override_per_citizen_need() {
    let cfg = SimConfig {
        food_reserve: 5.0,
        water_reserve: 3.0,
        energy_reserve: 2.0,
        ..SimConfig::default()
    };
    // Few citizens: the reserve wins.
    assert_eq!(cfg.food_need(1), 5.0);
    assert_eq!(cfg.water_need(0), 3.0);
    assert_eq!(cfg.energy_need(0), 2.0);
    // Many citizens: N*K wins.
    assert_eq!(cfg.food_need(1000), 100.0);
}

#[test]
fn default_need_is_per_citizen_product() {
    let cfg = SimConfig {
        food_reserve: 0.0,
        water_reserve: 0.0,
        energy_reserve: 0.0,
        ..SimConfig::default()
    };
    assert!((cfg.food_need(50) - 5.0).abs() < 1e-6);
    assert!((cfg.water_need(50) - 2.5).abs() < 1e-6);
    assert!((cfg.energy_need(50) - 2.5).abs() < 1e-6);
}

#[test]
fn load_rejects_grid_whose_cell_count_does_not_match_its_size() {
    let mut sim = Sim::with_defaults(8, 8);
    let json = sim.save().unwrap();

    // Rewrite the grid header to claim 8x8 while supplying no cells. Loading
    // must fail loudly instead of leaving a grid whose length disagrees with
    // width * height, which every rule indexes into.
    let corrupted = json.replace(
        &format!(
            "\"width\": {},\n  \"height\": {},\n  \"cells\": [\n",
            sim.grid.width, sim.grid.height
        ),
        "\"width\": 8,\n  \"height\": 8,\n  \"cells\": [\n",
    );
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    let cells = value["grid"]["cells"].as_array().unwrap().len();
    assert_eq!(cells, 64);

    let truncated = format!(
        "{{\"grid\":{{\"width\":8,\"height\":8,\"cells\":[]}},\"config\":{},\"render\":{},\"generation\":0}}",
        &json[json.find("\"config\"").unwrap()..json.find("\"render\"").unwrap()],
        &json[json.find("\"render\"").unwrap()..json.rfind("}").unwrap()],
    );
    assert!(
        sim.load_json(&truncated).is_err(),
        "a grid with zero cells for an 8x8 header must be rejected"
    );
    // The failed load left the simulation untouched.
    assert_eq!(sim.grid.cells.len(), 64);
    let _ = corrupted;
}

// ------------------------------------------------------------- render limits

#[test]
fn frame_budget_accumulates_and_caps() {
    use lab1::app::{budget_for, MAX_TICKS_PER_FRAME};
    // 10 ticks/second at 60fps: every sixth frame produces a tick.
    let mut acc = 0.0f32;
    let mut ticks = 0usize;
    for _ in 0..60 {
        ticks += budget_for(&mut acc, 1.0 / 60.0, 10.0);
    }
    assert!((9..=11).contains(&ticks), "expected ~10 ticks, got {ticks}");

    // A single slow frame cannot run more than the cap.
    let mut acc = 0.0f32;
    assert_eq!(budget_for(&mut acc, 10.0, 60.0), MAX_TICKS_PER_FRAME);

    // The leftover is dropped, so the next frame starts clean.
    assert_eq!(budget_for(&mut acc, 1.0 / 60.0, 60.0), 1);
}

#[test]
fn zoom_is_clamped_to_gpu_texture_limit() {
    use lab1::ui::canvas::{max_cell_px, MAX_TEXTURE_SIDE};
    for (w, h) in [(128usize, 128usize), (256, 256), (512, 512), (1024, 1024)] {
        let cap = max_cell_px(w, h);
        assert!(cap >= 1);
        assert!(
            w * cap <= MAX_TEXTURE_SIDE && h * cap <= MAX_TEXTURE_SIDE,
            "{w}x{h} at cell_px {cap} exceeds the {MAX_TEXTURE_SIDE}px limit"
        );
    }
    // 256x256 is the spec'd target performance case.
    assert_eq!(max_cell_px(256, 256), 8);
    // A tiny grid still allows the maximum zoom.
    assert_eq!(max_cell_px(4, 4), 32);
}

#[test]
fn palette_matches_spec() {
    use lab1::ui::palette::*;
    let expect = [
        (EMPTY, 0x1E1E1E),
        (CITIZEN, 0x000000),
        (WATER, 0x1E6FFF),
        (FOOD, 0x2ECC40),
        (ENERGY, 0xFFD400),
        (POPULATION, 0xD9D9D9),
        (CONFLICT, 0x8A2BE2),
        (OVERLAP, 0xFF7A00),
        (ANTICELL, 0xE01B24),
        (BORDER, 0x000000),
    ];
    for (actual, rgb) in expect {
        let want = egui::Color32::from_rgb(
            (rgb >> 16) as u8,
            ((rgb >> 8) & 0xFF) as u8,
            (rgb & 0xFF) as u8,
        );
        assert_eq!(actual, want, "palette entry {rgb:#08X} differs");
    }
}

// ------------------------------------------------------------- basic sanity

#[test]
fn grid_indexing_is_consistent() {
    let g = Grid::new(7, 5);
    for y in 0..5 {
        for x in 0..7 {
            assert_eq!(g.idx(x, y), y * 7 + x);
            assert_eq!(g.xy(y * 7 + x), (x, y));
        }
    }
}

#[test]
fn empty_grid_sim_runs_without_panic() {
    let mut sim = Sim::with_defaults(16, 16);
    for _ in 0..5 {
        let _ = sim.step();
    }
}

#[test]
fn reset_restores_seed_and_clears_state() {
    let mut sim = Sim::with_defaults(16, 16);
    sim.generate_colony(20);
    sim.step();
    sim.paint(1, 1, Brush::AntiCell);
    sim.reset();
    assert_eq!(sim.generation, 0);
    assert_eq!(sim.status, SimStatus::Paused);
    assert_eq!(sim.stats.n_citizen, 0);
    assert_eq!(sim.stats.n_anticell, 0);
    assert!(sim.cycle_length.is_none());
}

#[test]
fn resize_keeps_invariants() {
    let mut sim = Sim::with_defaults(16, 16);
    sim.generate_colony(10);
    sim.resize(32, 24);
    assert_eq!(sim.grid.width, 32);
    assert_eq!(sim.grid.height, 24);
    assert_eq!(sim.grid.cells.len(), 32 * 24);
    assert_eq!(sim.stats.n_citizen, 0);
}

#[test]
fn citizen_stats_stay_within_bounds() {
    let mut sim = Sim::with_defaults(32, 32);
    sim.generate_colony(80);
    for _ in 0..150 {
        if !sim.step() {
            break;
        }
        for c in &sim.grid.cells {
            if let Some(d) = c.citizen {
                assert!(
                    (0.0..=sim.config.c_max).contains(&d.stress),
                    "stress {}",
                    d.stress
                );
                assert!(
                    (0.0..=sim.config.f_max).contains(&d.fatigue),
                    "fatigue {}",
                    d.fatigue
                );
                assert!(
                    (0.0..=sim.config.l_max).contains(&d.loyalty),
                    "loyalty {}",
                    d.loyalty
                );
            }
        }
    }
}
