use rand::Rng;

use crate::config::SimConfig;
use crate::sim::cell::{Cell, CellState};
use crate::sim::grid::Grid;

/// Number of occupied cells in N8 around `(x, y)`.
pub fn density(grid: &Grid, x: usize, y: usize) -> usize {
    grid.neighbors8(x, y)
        .into_iter()
        .filter(|(nx, ny)| grid.get(*nx, *ny).is_occupied())
        .count()
}

/// Rule 9: citizens drift towards the least dense empty face neighbour.
pub fn rule9_citizen_move(
    grid: &Grid,
    next: &mut [Cell],
    config: &SimConfig,
    rng: &mut impl Rng,
) {
    if !config.citizen_move_enabled {
        return;
    }
    let mut intents: Vec<(usize, usize)> = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = grid.idx(x, y);
            if grid.cells[i].state != CellState::Citizen || next[i].state != CellState::Citizen {
                continue;
            }
            if next[i].citizen != grid.cells[i].citizen {
                continue;
            }
            if rng.gen::<f32>() >= config.citizen_move_prob {
                continue;
            }
            let mut best: Option<(usize, usize)> = None;
            let mut best_density = usize::MAX;
            for (nx, ny) in grid.neighbors4(x, y) {
                let j = grid.idx(nx, ny);
                if grid.cells[j].state != CellState::Empty {
                    continue;
                }
                let d = density(grid, nx, ny);
                if d < best_density {
                    best_density = d;
                    best = Some((nx, ny));
                }
            }
            if let Some((nx, ny)) = best {
                intents.push((i, grid.idx(nx, ny)));
            }
        }
    }
    intents.sort_by_key(|(src, _)| *src);

    let mut claimed: Vec<usize> = Vec::with_capacity(intents.len());
    for (src, dst) in intents {
        if claimed.contains(&src) || claimed.contains(&dst) {
            continue;
        }
        if next[src].state != CellState::Citizen {
            continue;
        }
        let data = match next[src].citizen {
            Some(d) => d,
            None => continue,
        };
        next[src] = Cell::empty();
        next[dst] = Cell::citizen(data);
        claimed.push(src);
        claimed.push(dst);
    }
}