use std::collections::{HashMap, HashSet};

use crate::sim::cell::{CellState, ConstructionKind};
use crate::sim::grid::Grid;
use crate::sim::pattern::PatternLibrary;

/// Production/consumption multiplier for one construction complex.
/// Overlapping complexes get `OVERLAP_MULTIPLIER`.
pub const OVERLAP_MULTIPLIER: f32 = 1.5;

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
    library
        .patterns
        .iter()
        .filter(|p| p.kind == kind)
        .any(|pattern| {
            pattern
                .rotations()
                .into_iter()
                .any(|rot| shape_holds(grid, &rot, id, anchor))
        })
}

/// True when every cell of the bounding box either belongs to `rot` and carries
/// `id`, or lies outside `rot` and does not. A single mismatch means this
/// rotation is not the shape the complex was built from, so another rotation
/// must be tried.
fn shape_holds(grid: &Grid, rot: &crate::sim::pattern::Pattern, id: u32, anchor: usize) -> bool {
    let min_x = rot.cells.iter().map(|(x, _)| *x as i32).min().unwrap_or(0);
    let min_y = rot.cells.iter().map(|(_, y)| *y as i32).min().unwrap_or(0);
    let w = rot.cells.iter().map(|(x, _)| *x as i32).max().unwrap_or(0) + 1 - min_x;
    let h = rot.cells.iter().map(|(_, y)| *y as i32).max().unwrap_or(0) + 1 - min_y;
    let (ax, ay) = grid.xy(anchor);

    for oy in 0..h {
        for ox in 0..w {
            let inside = rot
                .cells
                .iter()
                .any(|(dx, dy)| *dx as i32 - min_x == ox && *dy as i32 - min_y == oy);
            // Cells outside the grid cannot hold part of this complex, so a
            // shape that would extend past the border cannot match.
            let px = ax as i32 - ox;
            let py = ay as i32 - oy;
            if px < 0 || py < 0 || px >= grid.width as i32 || py >= grid.height as i32 {
                if inside {
                    return false;
                }
                continue;
            }
            let idx = grid.idx(px as usize, py as usize);
            if inside != (grid.cells[idx].construction_id == Some(id)) {
                return false;
            }
        }
    }
    true
}

/// Detect pattern matches over a grid of citizens.
///
/// §5.4 tries every pattern and every rotation at every anchor cell, but a
/// single shape must be reported once: a 2x2 block matches the farm in all
/// four rotations because it is its own rotation. Otherwise rule 5 would count
/// one pattern four times and mark a plain construction as `Overlap`. Matching
/// rotations of the same pattern at the same anchor therefore collapse to one
/// detection.
pub fn detect(grid: &Grid, library: &PatternLibrary) -> Vec<Detection> {
    let mut out: Vec<Detection> = Vec::new();
    // Deduplicates a shape across symmetric rotations.
    let mut seen: HashSet<(usize, Vec<usize>)> = HashSet::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y).state != CellState::Citizen {
                continue;
            }
            for pattern in &library.patterns {
                for rot in pattern.rotations() {
                    if let Some(mut cells) = try_match(grid, &rot, x, y) {
                        cells.sort_unstable();
                        if !seen.insert((pattern.kind.index(), cells.clone())) {
                            continue;
                        }
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

fn try_match(
    grid: &Grid,
    pattern: &crate::sim::pattern::Pattern,
    x: usize,
    y: usize,
) -> Option<Vec<usize>> {
    let min_x = pattern
        .cells
        .iter()
        .map(|(dx, _)| *dx as i32)
        .min()
        .unwrap_or(0);
    let min_y = pattern
        .cells
        .iter()
        .map(|(_, dy)| *dy as i32)
        .min()
        .unwrap_or(0);
    let w = pattern
        .cells
        .iter()
        .map(|(dx, _)| *dx as i32)
        .max()
        .unwrap_or(0)
        + 1
        - min_x;
    let h = pattern
        .cells
        .iter()
        .map(|(_, dy)| *dy as i32)
        .max()
        .unwrap_or(0)
        + 1
        - min_y;
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
