use crate::config::SimConfig;
use crate::sim::cell::{Cell, CellState};
use crate::sim::grid::Grid;

/// Direction priority for the anti-cell wave: up -> left -> right -> down.
/// Each entry is `(dx, dy)`; up means `dy == -1`.
pub const WAVE_PRIORITY: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

/// Rule 7: an anti-cell touching a regular cell destroys it and itself.
pub fn rule7_anticell_explosion(grid: &Grid, next: &mut [Cell]) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            if grid.cells[i].state != CellState::AntiCell {
                continue;
            }
            let mut hit = false;
            for (nx, ny) in grid.neighbors4(x, y) {
                let j = grid.idx(nx, ny);
                if grid.cells[j].is_regular() {
                    next[j] = Cell::empty();
                    hit = true;
                }
            }
            if hit {
                next[i] = Cell::empty();
            }
        }
    }
}

/// Indices of anti-cells that will explode this generation.
pub fn explosion_sources(grid: &Grid) -> Vec<usize> {
    let mut out = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            if grid.cells[i].state != CellState::AntiCell {
                continue;
            }
            if grid
                .neighbors4(x, y)
                .into_iter()
                .any(|(nx, ny)| grid.get(nx, ny).is_regular())
            {
                out.push(i);
            }
        }
    }
    out
}

/// Rule 8: a surviving anti-cell steps into the first empty N4 neighbour whose
/// next cell along the same direction is not empty.
///
/// Direction priority is fixed: up -> left -> right -> down. The first *empty*
/// neighbour in that order is the target; if the cell beyond it is also empty
/// the wave dissipates instead of moving. A target already claimed by another
/// anti-cell this generation is skipped, so two waves cannot merge (§6.9).
pub fn rule8_anticell_wave(
    grid: &Grid,
    next: &mut [Cell],
    config: &SimConfig,
    exploding: &[usize],
) {
    if !config.anticell_enabled {
        return;
    }
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            if grid.cells[i].state != CellState::AntiCell || exploding.contains(&i) {
                continue;
            }
            // This wave is already travelling somewhere else.
            if next[i].state != CellState::AntiCell {
                continue;
            }
            for (dx, dy) in WAVE_PRIORITY {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || ny < 0 || nx >= grid.width as isize || ny >= grid.height as isize {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                let j = grid.idx(nx, ny);
                if grid.cells[j].state != CellState::Empty {
                    continue;
                }
                let bx = nx as isize + dx;
                let by = ny as isize + dy;
                let beyond_filled = bx >= 0
                    && by >= 0
                    && (bx as usize) < grid.width
                    && (by as usize) < grid.height
                    && grid.get(bx as usize, by as usize).state != CellState::Empty;
                if beyond_filled {
                    next[j] = Cell::anticell();
                    next[i] = Cell::empty();
                }
                break;
            }
        }
    }
}
