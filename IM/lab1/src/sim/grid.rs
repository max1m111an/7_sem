use serde::{Deserialize, Serialize};

use crate::sim::cell::Cell;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
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
        let dirs = [(0usize, -1isize), (-1, 0), (1, 0), (0, 1)];
        dirs.into_iter().filter_map(move |(dx, dy)| {
            let nx = x as isize + dx as isize;
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

    pub fn cells_in_radius(&self, x: usize, y: usize, r: usize) -> impl Iterator<Item = (usize, usize)> {
        let (w, h) = (self.width, self.height);
        let r = r as isize;
        (0..(2 * r + 1) as usize)
            .flat_map(move |dy| (0..(2 * r + 1) as usize).map(move |dx| (dx as isize, dy as isize)))
            .filter(|(dx, dy)| dx * dx + dy * dy <= r * r)
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