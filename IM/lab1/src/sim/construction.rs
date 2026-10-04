use std::collections::HashMap;

use crate::sim::cell::{CellState, ConstructionKind};
use crate::sim::grid::Grid;
use crate::sim::pattern::PatternLibrary;

/// Production/consumption multipliers for one construction complex.
/// Overlapping complexes get `OVERLAP_MULTIPLIER`.
pub const OVERLAP_MULTIPLIER: f32 = 1.5;

/// Per-generation aggregate resource production for the whole colony.
#[derive(Clone, Copy, Debug, Default)]
pub struct Production {
    pub water: f32,
    pub food: f32,
    pub energy: f32,
    pub population: f32,
    /// Sum of conflict stress contributions per cell index.
    pub conflict: Vec<f32>,
}

/// Assigns stable ids to construction complexes.
#[derive(Clone, Debug, Default)]
pub struct Registry {
    next: u32,
    index: HashMap<u32, usize>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            next: 1,
            index: HashMap::new(),
        }
    }

    pub fn allocate(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        self.index.insert(id, self.index.len());
        id
    }

    pub fn match_by_id(&self, id: u32) -> Option<usize> {
        self.index.get(&id).copied()
    }
}

/// Cells whose construction shape no longer matches the pattern that formed them.
pub fn broken_constructions(grid: &Grid, library: &PatternLibrary) -> Vec<usize> {
    let mut broken = Vec::new();
    for (i, cell) in grid.cells.iter().enumerate() {
        let id = match cell.construction_id {
            Some(id) if matches!(cell.state, CellState::Construction | CellState::Overlap) => id,
            _ => continue,
        };
        if !complex_alive(grid, library, id, i) {
            broken.push(i);
        }
    }
    broken
}

/// A complex is alive while every cell that carries `id` still belongs to the
/// shape it was built from.
fn complex_alive(grid: &Grid, library: &PatternLibrary, id: u32, anchor: usize) -> bool {
    let kind = match grid.cells[anchor].kind {
        Some(k) => k,
        None => return false,
    };
    let (ax, ay) = grid.xy(anchor);
    for pattern in library.patterns.iter().filter(|p| p.kind == kind) {
        for rot in pattern.rotations() {
            let min_x = rot.cells.iter().map(|(x, _)| *x as i32).min().unwrap_or(0);
            let min_y = rot.cells.iter().map(|(_, y)| *y as i32).min().unwrap_or(0);
            let w = rot.cells.iter().map(|(x, _)| *x as i32).max().unwrap_or(0) + 1 - min_x;
            let h = rot.cells.iter().map(|(_, y)| *y as i32).max().unwrap_or(0) + 1 - min_y;
            for oy in 0..h {
                for ox in 0..w {
                    let px = ax as i32 - ox;
                    let py = ay as i32 - oy;
                    if px < 0 || py < 0 || px >= grid.width as i32 || py >= grid.height as i32 {
                        continue;
                    }
                    let inside = rot
                        .cells
                        .iter()
                        .any(|(dx, dy)| *dx as i32 - min_x == ox && *dy as i32 - min_y == oy);
                    let idx = grid.idx(px as usize, py as usize);
                    let has_id = grid.cells[idx].construction_id == Some(id);
                    if inside != has_id {
                        return false;
                    }
                }
            }
            return true;
        }
    }
    false
}

/// Detect pattern matches over a grid of citizens. Returns `(anchor, kind, cells)`.
pub fn detect(grid: &Grid, library: &PatternLibrary) -> Vec<Detection> {
    let mut out: Vec<Detection> = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y).state != CellState::Citizen {
                continue;
            }
            for pattern in &library.patterns {
                for rot in pattern.rotations() {
                    if let Some(cells) = try_match(grid, &rot, x, y) {
                        out.push(Detection {
                            kind: pattern.kind,
                            cells,
                        });
                    }
                }
            }
        }
    }
    out
}

fn try_match(grid: &Grid, pattern: &crate::sim::pattern::Pattern, x: usize, y: usize) -> Option<Vec<usize>> {
    let min_x = pattern.cells.iter().map(|(dx, _)| *dx as i32).min().unwrap_or(0);
    let min_y = pattern.cells.iter().map(|(_, dy)| *dy as i32).min().unwrap_or(0);
    let w = pattern.cells.iter().map(|(dx, _)| *dx as i32).max().unwrap_or(0) + 1 - min_x;
    let h = pattern.cells.iter().map(|(_, dy)| *dy as i32).max().unwrap_or(0) + 1 - min_y;
    if x + w as usize > grid.width || y + h as usize > grid.height {
        return None;
    }
    let mut cells = Vec::with_capacity(pattern.cells.len());
    for (dx, dy) in &pattern.cells {
        let px = x as i32 + *dx as i32 - min_x;
        let py = y as i32 + *dy as i32 - min_y;
        let idx = grid.idx(px as usize, py as usize);
        let c = &grid.cells[idx];
        let ok = c.state == CellState::Citizen
            || (matches!(c.state, CellState::Construction | CellState::Overlap)
                && c.kind == Some(pattern.kind));
        if !ok {
            return None;
        }
        cells.push(idx);
    }
    Some(cells)
}

#[derive(Clone, Debug)]
pub struct Detection {
    pub kind: ConstructionKind,
    pub cells: Vec<usize>,
}