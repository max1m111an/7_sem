use egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions, Ui, Vec2};

use crate::sim::cell::{CellState, ConstructionKind};
use crate::sim::{Brush, Sim};
use crate::ui::palette;

pub use crate::sim::MAX_CELL_PX;
/// Hard GPU texture side limit; egui/glow rejects anything larger.
pub use crate::sim::MAX_TEXTURE_SIDE;

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
    /// Leftover wheel travel, so a slow scroll still ends up moving a whole
    /// zoom step instead of drifting between `cell_px` values.
    scroll_accum: f32,
    /// The `cell_px` the current grid was sized for, so the grid is refitted
    /// on a zoom change but not on every window resize.
    fitted_zoom: Option<usize>,
    /// Screen rect the grid was last painted into, for the vector overlays.
    pub rect: egui::Rect,
    /// Screen pixels per cell actually in effect. Equal to `render.cell_px`
    /// except for a sub-pixel stretch that hides the resize slack.
    pub display_cell_px: f32,
}

/// Wheel distance that equals one zoom step.
const ZOOM_STEP: f32 = 50.0;

/// Ctrl + mouse wheel is the zoom control. A bare wheel is left alone so it
/// stays available for whatever the surrounding panels want to do with it.
///
/// `raw_scroll_delta` is used rather than `zoom_delta` because Windows only
/// reports the latter for touchpads; a real wheel with Ctrl held arrives as a
/// plain scroll event. Wheel deltas are fractional on high-resolution wheels,
/// so travel is accumulated into whole steps.
fn steps_from_ctrl_wheel(ui: &Ui, accum: &mut f32) -> isize {
    let scroll = ui.input(|i| {
        if i.modifiers.ctrl {
            i.raw_scroll_delta.y
        } else {
            0.0
        }
    });
    if scroll == 0.0 {
        // Drop the remainder so a direction change cannot carry stale travel
        // into the next gesture.
        *accum = 0.0;
        return 0;
    }
    *accum += scroll;
    let mut steps = 0isize;
    while *accum >= ZOOM_STEP {
        *accum -= ZOOM_STEP;
        steps += 1;
    }
    while *accum <= -ZOOM_STEP {
        *accum += ZOOM_STEP;
        steps -= 1;
    }
    steps
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
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
            scroll_accum: 0.0,
            fitted_zoom: None,
            rect: egui::Rect::NOTHING,
            display_cell_px: 1.0,
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
        // Ctrl + RMB drag is the zoom control, and zoom drives the grid size,
        // so the colony always fills the whole middle area with square cells.
        let steps = steps_from_ctrl_wheel(ui, &mut self.scroll_accum);
        if steps != 0 {
            sim.zoom_by(steps);
        }

        let area = ui.available_size();
        // The grid is refitted only when the zoom changes. Doing it on window
        // resizes too would crop the world on every frame of a window drag.
        if self.fitted_zoom != Some(sim.render.cell_px) {
            sim.fit_view(
                sim.render.cell_px,
                area.x.round() as usize,
                area.y.round() as usize,
            );
            self.fitted_zoom = Some(sim.render.cell_px);
        }
        let (w, h) = (sim.grid.width, sim.grid.height);
        let cell_px = sim.render.cell_px;

        let dirty = self.sync_colors(sim);
        if self.full_rebuild || dirty.any {
            self.upload(ui.ctx(), sim, cell_px, dirty);
        }

        // Stretch to the panel instead of leaving dead space; the difference is
        // at most a fraction of a pixel per cell because the grid is snapped.
        let tex_id = self.texture.as_ref().map(|t| t.id()).unwrap_or_default();
        let image = egui::Image::new((
            tex_id,
            Vec2::new((w * cell_px) as f32, (h * cell_px) as f32),
        ))
        .fit_to_exact_size(area)
        .sense(egui::Sense::click_and_drag());
        let resp = ui.add(image);
        let rect = resp.rect;
        self.rect = rect;
        self.display_cell_px = if w > 0 {
            rect.width() / w as f32
        } else {
            cell_px as f32
        };

        self.handle_input(
            ui,
            sim,
            tool,
            rotation,
            drag,
            rect,
            self.display_cell_px,
            w,
            h,
        );
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
        cell_px: f32,
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
            let x = (local.x / cell_px) as usize;
            let y = (local.y / cell_px) as usize;
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

/// Black border thickness in pixels (В§8.5). Fixed rather than configurable:
/// at the default `cell_px = 4` a single pixel is already a quarter of the cell,
/// and the user has zoom rather than border thickness to spend effort on.
pub const BORDER_PX: usize = 1;

/// Fills the `cell_px` square at `(x, y)` with its fill colour and a black
/// border. Borders are skipped entirely at `cell_px == 1` because a single
/// pixel per cell has no room for them (В§8.5).
fn paint_cell(
    img: &mut ColorImage,
    cell: &crate::sim::Cell,
    sim: &Sim,
    x: usize,
    y: usize,
    cell_px: usize,
) {
    let mut fill = cell_color(cell.state, cell.kind, sim);
    let b = BORDER_PX.min(cell_px.saturating_sub(1));
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
