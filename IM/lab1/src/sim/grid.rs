use serde::{Deserialize, Serialize};

use crate::sim::cell::Cell;

#[derive(Clone, Debug, Serialize)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

/// Rejects a payload whose cell count disagrees with `width * height`: every
/// rule indexes the flat `cells` vector by `y * width + x`, so a mismatch would
/// panic or read out of bounds on the first tick.
impl<'de> Deserialize<'de> for Grid {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Raw {
            width: usize,
            height: usize,
            cells: Vec<Cell>,
        }
        let raw = Raw::deserialize(d)?;
        let expected = raw
            .width
            .checked_mul(raw.height)
            .ok_or_else(|| serde::de::Error::custom("grid dimensions overflow"))?;
        if raw.cells.len() != expected {
            return Err(serde::de::Error::custom(format!(
                "grid {}x{} needs {} cells, got {}",
                raw.width,
                raw.height,
                expected,
                raw.cells.len()
            )));
        }
        Ok(Self {
            width: raw.width,
            height: raw.height,
            cells: raw.cells,
        })
    }
}

/// Wraps `v + delta` into `0..len`: the field is a torus, so a step off one
/// edge re-enters at the opposite one. A degenerate `len == 0` grid returns 0
/// instead of panicking inside `rem_euclid`.
#[inline]
pub fn wrap(v: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (v as isize + delta).rem_euclid(len as isize) as usize
}

/// The canonical offsets `-r..=r` reduced modulo `len`: a balanced residue
/// window of `min(2r + 1, len)` entries, one per distinct cell of that axis
/// and biased towards the smaller absolute value. Using the closest
/// representative for both axes is what makes the distance check in
/// [`Grid::cells_in_radius`] measure the shortest way round the torus.
fn axis_span(len: usize, r: isize) -> (isize, isize) {
    let len = len as isize;
    (-r.min((len - 1) / 2), r.min(len / 2))
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::empty(); width * height],
        }
    }

    #[inline]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn xy(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let i = y * self.width + x;
        self.cells[i] = cell;
    }

    /// Face neighbours; the field is a torus, so the left neighbour of a cell
    /// in the first column sits in the last column.
    ///
    /// Directions come out in the order up, left, right, down — the anti-cell
    /// wave priority (§6.9) depends on it. A side of fewer than three cells
    /// would fold two of those directions onto the same cell (or onto the cell
    /// itself), so only the first representative is kept: real fields are far
    /// wider than that, but tests use tiny grids and duplicates would inflate
    /// [`crate::sim::movement::density`].
    pub fn neighbors4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let dirs: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        dirs.into_iter()
            .filter(move |(dx, dy)| {
                if *dx == 0 {
                    h >= 3 || (h == 2 && *dy < 0)
                } else {
                    w >= 3 || (w == 2 && *dx < 0)
                }
            })
            .map(move |(dx, dy)| (wrap(x, dx, w), wrap(y, dy, h)))
    }

    /// All eight neighbours, wrapping. `density()` counts occupied cells here,
    /// so a coordinate must not be produced twice even on a grid small enough
    /// for the 3x3 window to fold over itself.
    pub fn neighbors8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let dirs: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut out = [(0usize, 0usize); 8];
        let mut n = 0;
        'next: for (dx, dy) in dirs {
            let (nx, ny) = (wrap(x, dx, w), wrap(y, dy, h));
            // A side of one cell folds the window onto the cell itself, which
            // is not its own neighbour.
            if (nx, ny) == (x, y) {
                continue;
            }
            for &seen in &out[..n] {
                if seen == (nx, ny) {
                    continue 'next;
                }
            }
            out[n] = (nx, ny);
            n += 1;
        }
        out.into_iter().take(n)
    }

    /// Cells whose centre lies within Euclidean distance `r` of `(x, y)`. The
    /// distance is measured from the centre cell, so a cell exactly `r` steps
    /// away is included, and the disc wraps: a conflict zone in the first
    /// column still reaches the last one.
    ///
    /// Offsets run over [`axis_span`], the canonical representatives of
    /// `-r..=r` modulo each side length, so a grid narrower than the disc
    /// visits every cell once instead of once per lap around the torus.
    pub fn cells_in_radius(
        &self,
        x: usize,
        y: usize,
        r: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let r = r as isize;
        let (ox0, ox1) = axis_span(w, r);
        let (oy0, oy1) = axis_span(h, r);
        (oy0..=oy1)
            .flat_map(move |oy| (ox0..=ox1).map(move |ox| (ox, oy)))
            // Measure from the centre, not from the top-left corner of the window.
            .filter(move |(ox, oy)| ox * ox + oy * oy <= r * r)
            .map(move |(ox, oy)| (wrap(x, ox, w), wrap(y, oy, h)))
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        self.cells.swap(a, b);
    }

    /// Highest construction id present in the grid, or 0.
    pub fn max_construction_id(&self) -> u32 {
        self.cells
            .iter()
            .filter_map(|c| c.construction_id)
            .max()
            .unwrap_or(0)
    }
}
