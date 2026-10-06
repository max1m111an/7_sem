use std::collections::{HashMap, HashSet};

use crate::sim::cell::{CellState, ConstructionKind};
use crate::sim::grid::{wrap, Grid};
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

/// True when the cells carrying `id` still spell `rot`.
///
/// The anchor is *some* cell of the shape, so the bounding box has to be
/// placed so that the anchor lands on the pattern cell it actually is. Older
/// code hung the box off the anchor towards the top-left, which silently
/// assumed the anchor was its bottom-right corner: every other cell of a fresh
/// complex then failed the check, was released on the next generation and the
/// grid oscillated between "all construction" and "all citizens" until the
/// cycle detector stopped the run. Every origin that puts the anchor inside
/// the shape is tried instead, and the first one whose box matches the cells
/// carrying `id` wins.
fn shape_holds(grid: &Grid, rot: &crate::sim::pattern::Pattern, id: u32, anchor: usize) -> bool {
    let min_x = rot.cells.iter().map(|(x, _)| *x as i32).min().unwrap_or(0);
    let min_y = rot.cells.iter().map(|(_, y)| *y as i32).min().unwrap_or(0);
    let w = rot.cells.iter().map(|(x, _)| *x as i32).max().unwrap_or(0) + 1 - min_x;
    let h = rot.cells.iter().map(|(_, y)| *y as i32).max().unwrap_or(0) + 1 - min_y;
    // A shape larger than the field would fold onto itself, and the
    // outside-the-box test would then contradict the inside test.
    if w as usize > grid.width || h as usize > grid.height {
        return false;
    }
    let (ax, ay) = grid.xy(anchor);

    rot.cells.iter().any(|(dx, dy)| {
        // Try the placement where the anchor plays this cell of the pattern.
        let ox0 = *dx as i32 - min_x;
        let oy0 = *dy as i32 - min_y;
        let origin_x = wrap(ax, -(ox0 as isize), grid.width);
        let origin_y = wrap(ay, -(oy0 as isize), grid.height);
        (0..h as isize).all(|oy| {
            (0..w as isize).all(|ox| {
                let inside = rot.cells.iter().any(|(rx, ry)| {
                    *rx as i32 - min_x == ox as i32 && *ry as i32 - min_y == oy as i32
                });
                // The box may reach across the seam of the torus instead of
                // failing a bounds test.
                let px = wrap(origin_x, ox, grid.width);
                let py = wrap(origin_y, oy, grid.height);
                inside == (grid.cells[grid.idx(px, py)].construction_id == Some(id))
            })
        })
    })
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
    // A shape may straddle the seam, but it must not fold onto itself: two of
    // its cells would then claim the same index and the pattern could never be
    // distinguished from a smaller one.
    if w as usize > grid.width || h as usize > grid.height {
        return None;
    }
    let mut cells = Vec::with_capacity(pattern.cells.len());
    for (dx, dy) in &pattern.cells {
        let px = wrap(x, *dx as isize - min_x as isize, grid.width);
        let py = wrap(y, *dy as isize - min_y as isize, grid.height);
        let idx = grid.idx(px, py);
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
