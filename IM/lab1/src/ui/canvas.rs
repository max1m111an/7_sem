use egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions, Ui, Vec2};

use crate::sim::cell::{CellState, ConstructionKind};
use crate::sim::{Brush, Sim};
use crate::ui::palette;

/// Hard GPU texture side limit; egui/glow rejects anything larger.
pub const MAX_TEXTURE_SIDE: usize = 2048;
pub const MAX_CELL_PX: usize = 32;

/// Largest `cell_px` that keeps `width * cell_px` and `height * cell_px`
/// within the GPU texture limit.
pub fn max_cell_px(width: usize, height: usize) -> usize {
    let longest = width.max(height).max(1) as f32;
    let fit = (MAX_TEXTURE_SIDE as f32 / longest).floor() as usize;
    fit.clamp(1, MAX_CELL_PX)
}

/// A changed-cell bounding box in grid coordinates.
#[derive(Clone, Copy, Debug, Default)]
pub struct DirtyRect {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
    pub any: bool,
}

impl DirtyRect {
    /// Grows the box to include `(x, y)`.
    pub fn add(&mut self, x: usize, y: usize) {
        if !self.any {
            self.x0 = x;
            self.y0 = y;
            self.x1 = x;
            self.y1 = y;
            self.any = true;
            return;
        }
        self.x0 = self.x0.min(x);
        self.y0 = self.y0.min(y);
        self.x1 = self.x1.max(x);
        self.y1 = self.y1.max(y);
    }
}

#[derive(Default)]
pub struct Canvas {
    texture: Option<TextureHandle>,
    /// CPU-side copy of the texture. Dirty-region updates patch this buffer so
    /// untouched cells are never recoloured.
    buffer: Option<ColorImage>,
    /// Colour of every grid cell as of the last upload.
    prev: Vec<Color32>,
    /// Texture size the current `buffer`/`texture` were built for.
    tex_size: [usize; 2],
    full_rebuild: bool,
    dragging: bool,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            texture: None,
            buffer: None,
            prev: Vec::new(),
            tex_size: [0, 0],
            full_rebuild: true,
            dragging: false,
        }
    }

    /// Drop the cached colours so the next frame re-uploads everything.
    pub fn invalidate(&mut self) {
        self.full_rebuild = true;
        self.prev.clear();
        self.dragging = false;
    }

    pub fn is_full_rebuild(&self) -> bool {
        self.full_rebuild
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        sim: &mut Sim,
        tool: crate::ui::tools::Tool,
        rotation: usize,
        drag: &mut Option<(usize, usize)>,
    ) {
        let (w, h) = (sim.grid.width, sim.grid.height);
        let cap = max_cell_px(w, h);
        let mut cell_px = sim.render.cell_px.clamp(1, MAX_CELL_PX).min(cap);
        sim.render.cell_px = cell_px;

        let zoom = ui.input(|i| i.zoom_delta());
        if (zoom - 1.0).abs() > f32::EPSILON {
            let scaled = (cell_px as f32 * zoom).round() as i64;
            cell_px = scaled.clamp(1, cap as i64) as usize;
            sim.render.cell_px = cell_px;
        }

        let dirty = self.sync_colors(sim);
        if self.full_rebuild || dirty.any {
            self.upload(ui.ctx(), sim, cell_px, dirty);
        }

        let size = Vec2::new((w * cell_px) as f32, (h * cell_px) as f32);
        let tex_id = self.texture.as_ref().map(|t| t.id()).unwrap_or_default();
        let image = egui::Image::new((tex_id, size)).sense(egui::Sense::click_and_drag());
        let resp = ui.add(image);
        let rect = resp.rect;

        self.handle_input(ui, sim, tool, rotation, drag, rect, cell_px, w, h);
    }

    /// Compare the grid against the cached colours, returning the bounding box
    /// of everything that changed. Empty when the grid is untouched, so a
    /// paused simulation uploads nothing.
    fn sync_colors(&mut self, sim: &Sim) -> DirtyRect {
        let n = sim.grid.cells.len();
        let mut dirty = DirtyRect::default();
        if self.prev.len() != n {
            self.prev = vec![palette::EMPTY; n];
            self.full_rebuild = true;
        }
        let w = sim.grid.width.max(1);
        for (i, c) in sim.grid.cells.iter().enumerate() {
            let want = cell_color(c.state, c.kind, sim);
            if self.prev[i] != want {
                self.prev[i] = want;
                dirty.add(i % w, i / w);
            }
        }
        dirty
    }

    /// Repaints the dirty box into the CPU buffer and hands the buffer to the
    /// GPU. egui's `TextureHandle::set` replaces the whole texture, so the
    /// buffer is kept and only the changed cells are redrawn вЂ” that keeps the
    /// per-frame cost proportional to the number of changed cells.
    fn upload(&mut self, ctx: &Context, sim: &Sim, cell_px: usize, dirty: DirtyRect) {
        let w = sim.grid.width;
        let h = sim.grid.height;
        let tex_w = (w * cell_px).max(1);
        let tex_h = (h * cell_px).max(1);
        let tex_size = [tex_w, tex_h];

        let full = self.full_rebuild
            || self.texture.is_none()
            || self.tex_size != tex_size
            || self
                .buffer
                .as_ref()
                .map(|b| b.size != tex_size)
                .unwrap_or(true);

        let buf = self
            .buffer
            .get_or_insert_with(|| ColorImage::new(tex_size, palette::EMPTY));
        if buf.size != tex_size {
            *buf = ColorImage::new(tex_size, palette::EMPTY);
        }

        let (bx0, by0, bx1, by1) = if full {
            (0usize, 0usize, w.saturating_sub(1), h.saturating_sub(1))
        } else if dirty.any {
            (dirty.x0, dirty.y0, dirty.x1, dirty.y1)
        } else {
            return;
        };

        for y in by0..=by1.min(h.saturating_sub(1)) {
            for x in bx0..=bx1.min(w.saturating_sub(1)) {
                let i = y * w + x;
                paint_cell(buf, &sim.grid.cells[i], sim, x, y, cell_px);
            }
        }

        self.tex_size = tex_size;
        let handle = ctx.load_texture(
            "grid",
            buf.clone(),
            TextureOptions {
                magnification: egui::TextureFilter::Nearest,
                minification: egui::TextureFilter::Nearest,
                wrap_mode: egui::TextureWrapMode::ClampToEdge,
                mipmap_mode: None,
            },
        );
        self.texture = Some(handle);
        self.full_rebuild = false;
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_input(
        &mut self,
        ui: &mut Ui,
        sim: &mut Sim,
        tool: crate::ui::tools::Tool,
        rotation: usize,
        drag: &mut Option<(usize, usize)>,
        rect: egui::Rect,
        cell_px: usize,
        w: usize,
        h: usize,
    ) {
        if rect.width() <= 0.0 || rect.height() <= 0.0 {
            return;
        }
        let hovered = ui
            .input(|i| i.pointer.hover_pos())
            .filter(|p| rect.contains(*p));
        let cell_at = |p: egui::Pos2| -> Option<(usize, usize)> {
            let local = p - rect.min;
            let x = (local.x / cell_px as f32) as usize;
            let y = (local.y / cell_px as f32) as usize;
            if x < w && y < h {
                Some((x, y))
            } else {
                None
            }
        };

        if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Secondary)) {
            if let Some(pos) = hovered {
                if let Some((x, y)) = cell_at(pos) {
                    sim.paint(x, y, Brush::Empty);
                }
            }
        }

        if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary)) {
            if let Some(pos) = hovered {
                if let Some((x, y)) = cell_at(pos) {
                    *drag = Some((x, y));
                    self.dragging = true;
                    apply(sim, tool, rotation, x, y);
                }
            }
        } else if self.dragging && ui.input(|i| i.pointer.any_down()) {
            if let Some(pos) = hovered {
                if let Some((x, y)) = cell_at(pos) {
                    if let Some((px, py)) = *drag {
                        draw_line(sim, tool, rotation, px, py, x, y);
                    }
                    *drag = Some((x, y));
                }
            }
        }

        if !ui.input(|i| i.pointer.any_down()) {
            self.dragging = false;
            *drag = None;
        }
    }
}

/// Fills the `cell_px` square at `(x, y)` with its fill colour and, when
/// enabled, a black border. Borders are skipped entirely at `cell_px == 1`
/// because a single pixel per cell has no room for them (В§8.5).
fn paint_cell(
    img: &mut ColorImage,
    cell: &crate::sim::Cell,
    sim: &Sim,
    x: usize,
    y: usize,
    cell_px: usize,
) {
    let mut fill = cell_color(cell.state, cell.kind, sim);
    let b = sim.render.border_px.min(cell_px.saturating_sub(1));
    // В§8.6: overlay heat maps tint the cell body only, borders stay black.
    if cell.state == CellState::Citizen {
        if let Some(d) = cell.citizen {
            if sim.render.overlay_stress {
                fill = palette::heat(Color32::from_rgb(255, 0, 0), d.stress / sim.config.c_max);
            } else if sim.render.overlay_fatigue {
                fill = palette::heat(Color32::from_rgb(0, 0, 255), d.fatigue / sim.config.f_max);
            } else if sim.render.overlay_loyalty {
                fill = palette::heat(Color32::from_rgb(0, 255, 0), d.loyalty / sim.config.l_max);
            }
        }
    }
    for py in 0..cell_px {
        for px in 0..cell_px {
            let idx = (y * cell_px + py) * img.size[0] + (x * cell_px + px);
            let on_border = b > 0 && (px < b || py < b || px >= cell_px - b || py >= cell_px - b);
            img.pixels[idx] = if on_border { palette::BORDER } else { fill };
        }
    }
}

/// Fill colour for a cell, ignoring borders (В§8.5 palette).
fn cell_color(state: CellState, kind: Option<ConstructionKind>, _sim: &Sim) -> Color32 {
    match state {
        CellState::Empty => palette::EMPTY,
        CellState::AntiCell => palette::ANTICELL,
        CellState::Overlap => palette::OVERLAP,
        CellState::Construction => match kind {
            Some(ConstructionKind::Water) => palette::WATER,
            Some(ConstructionKind::Food) => palette::FOOD,
            Some(ConstructionKind::Energy) => palette::ENERGY,
            Some(ConstructionKind::Population) => palette::POPULATION,
            Some(ConstructionKind::Conflict) => palette::CONFLICT,
            None => palette::CITIZEN,
        },
        CellState::Citizen => palette::CITIZEN,
    }
}

fn apply(sim: &mut Sim, tool: crate::ui::tools::Tool, rotation: usize, x: usize, y: usize) {
    match tool {
        crate::ui::tools::Tool::Empty => sim.paint(x, y, Brush::Empty),
        crate::ui::tools::Tool::Citizen => sim.paint(x, y, Brush::Citizen),
        crate::ui::tools::Tool::AntiCell => sim.paint(x, y, Brush::AntiCell),
        other => {
            if let Some(name) = other.pattern_name() {
                sim.stamp(name, x, y, rotation);
            }
        }
    }
}

/// Bresenham line so dragging does not leave gaps between sampled cells (§8.4).
fn draw_line(
    sim: &mut Sim,
    tool: crate::ui::tools::Tool,
    rotation: usize,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
) {
    let (mut x, mut y) = (x0 as i64, y0 as i64);
    let (tx, ty) = (x1 as i64, y1 as i64);
    let dx = (tx - x).abs();
    let dy = (ty - y).abs();
    let sx = if x < tx { 1 } else { -1 };
    let sy = if y < ty { 1 } else { -1 };
    let mut err = dx - dy;
    loop {
        if x >= 0 && y >= 0 && (x as usize) < sim.grid.width && (y as usize) < sim.grid.height {
            apply(sim, tool, rotation, x as usize, y as usize);
        }
        if x == tx && y == ty {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}
