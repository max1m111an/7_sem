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

    /// Face neighbours; edges do not wrap.
    pub fn neighbors4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let dirs: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
        dirs.into_iter().filter_map(move |(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                None
            } else {
                Some((nx as usize, ny as usize))
            }
        })
    }

    pub fn neighbors8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        (0..3isize)
            .flat_map(move |dy| (0..3isize).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| *dx != 1 || *dy != 1)
            .filter_map(move |(dx, dy)| {
                let nx = x as isize + dx - 1;
                let ny = y as isize + dy - 1;
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    None
                } else {
                    Some((nx as usize, ny as usize))
                }
            })
    }

    /// Cells whose centre lies within Chebyshev-free Euclidean distance `r` of
    /// `(x, y)`, clipped to the grid. The distance is measured from the centre
    /// cell, so a cell exactly `r` steps away is included.
    pub fn cells_in_radius(
        &self,
        x: usize,
        y: usize,
        r: usize,
    ) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let r = r as isize;
        let span = (2 * r + 1) as usize;
        (0..span)
            .flat_map(move |dy| (0..span).map(move |dx| (dx as isize, dy as isize)))
            // Measure from the centre, not from the top-left corner of the window.
            .filter(move |(dx, dy)| {
                let (ox, oy) = (dx - r, dy - r);
                ox * ox + oy * oy <= r * r
            })
            .filter_map(move |(dx, dy)| {
                let nx = x as isize + dx - r;
                let ny = y as isize + dy - r;
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    None
                } else {
                    Some((nx as usize, ny as usize))
                }
            })
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
