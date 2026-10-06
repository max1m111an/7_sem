use egui::{Color32, ColorImage, Context, TextureHandle, TextureOptions, Ui, Vec2};

use crate::sim::cell::{Cell, Species};
use crate::sim::Life;
use crate::ui::palette;

/// Hard GPU texture side limit; egui/glow rejects anything larger.
pub use crate::sim::MAX_TEXTURE_SIDE;

/// Пределы зума: размер клетки в текстурных пикселях, 1..=32.
pub use crate::sim::{MAX_CELL_PX, MIN_CELL_PX};

/// Размер клетки до первого кручения колеса с Ctrl.
pub const DEFAULT_CELL_PX: usize = 14;

/// Толщина чёрной рамки клетки в пикселях. Фиксированная: при клетке в один
/// пиксель рамки нет совсем — места на неё не остаётся.
pub const BORDER_PX: usize = 1;

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
    /// Размер клетки в текстурных пикселях: 1..=32, крутится Ctrl+колесом.
    pub cell_px: usize,
    /// Дробный остаток прокрутки колеса — колесо даёт дробные дельты, а зум
    /// шагами по 50 px.
    scroll_accum: f32,
    /// Размер клетки, под который мир был подогнан в последний раз. Подгонка
    /// нужна только при смене зума: при каждом ресайзе окна мир бы обрезался
    /// прямо на глазах.
    fitted_zoom: Option<usize>,
    /// Drag in progress: the last cell visited and the species the brush
    /// paints (`None` for the erasing right button).
    drag: Option<(usize, usize, Option<Species>)>,
    /// Screen rect the grid was last painted into.
    pub rect: egui::Rect,
    /// Screen pixels per cell actually in effect. Equal to [`Self::cell_px`]
    /// except for a sub-pixel stretch that hides the fit slack.
    pub display_cell_px: f32,
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
            cell_px: DEFAULT_CELL_PX,
            scroll_accum: 0.0,
            fitted_zoom: None,
            drag: None,
            rect: egui::Rect::NOTHING,
            display_cell_px: DEFAULT_CELL_PX as f32,
        }
    }

    /// Drop the cached colours so the next frame re-uploads everything.
    pub fn invalidate(&mut self) {
        self.full_rebuild = true;
        self.prev.clear();
        self.drag = None;
    }

    pub fn show(&mut self, ui: &mut Ui, life: &mut Life, brush: Species) {
        let steps = steps_from_ctrl_wheel(ui, &mut self.scroll_accum);
        if steps != 0 {
            self.zoom_by(steps);
        }

        let area = ui.available_size();
        // The grid is refitted only when the zoom changes. Doing it on window
        // resizes too would crop the world on every frame of a window drag.
        if self.fitted_zoom != Some(self.cell_px) {
            let (w, h) = crate::sim::grid_dims_for(
                self.cell_px,
                area.x.round().max(0.0) as usize,
                area.y.round().max(0.0) as usize,
            );
            life.resize(w, h);
            self.fitted_zoom = Some(self.cell_px);
            self.invalidate();
        }

        let (w, h) = (life.grid.width, life.grid.height);
        let cell_px = self.cell_px;
        let dirty = self.sync_colors(life);
        if self.full_rebuild || dirty.any {
            self.upload(ui.ctx(), life, dirty);
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
        let display_cell_px = if w > 0 {
            rect.width() / w as f32
        } else {
            cell_px as f32
        };
        self.display_cell_px = display_cell_px;

        self.handle_input(ui, life, rect, display_cell_px, w, h, brush);
    }

    /// Крутит размер клетки на `steps` шагов, не выходя за 1..=32.
    fn zoom_by(&mut self, steps: isize) {
        self.cell_px = (self.cell_px as isize + steps)
            .clamp(MIN_CELL_PX as isize, MAX_CELL_PX as isize) as usize;
    }

    /// Compare the grid against the cached colours, returning the bounding box
    /// of everything that changed. Empty when the grid is untouched, so a
    /// paused game uploads nothing.
    fn sync_colors(&mut self, life: &Life) -> DirtyRect {
        let n = life.grid.cells.len();
        let mut dirty = DirtyRect::default();
        if self.prev.len() != n {
            self.prev = vec![palette::FIELD; n];
            self.full_rebuild = true;
        }
        let w = life.grid.width.max(1);
        for (i, c) in life.grid.cells.iter().enumerate() {
            let want = cell_color(c);
            if self.prev[i] != want {
                self.prev[i] = want;
                dirty.add(i % w, i / w);
            }
        }
        dirty
    }

    /// Repaints the dirty box into the CPU buffer and hands the buffer to the
    /// GPU. egui's `TextureHandle::set` replaces the whole texture, so the
    /// buffer is kept and only the changed cells are redrawn — that keeps the
    /// per-frame cost proportional to the number of changed cells.
    fn upload(&mut self, ctx: &Context, life: &Life, dirty: DirtyRect) {
        let w = life.grid.width;
        let h = life.grid.height;
        let cell_px = self.cell_px;
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
            .get_or_insert_with(|| ColorImage::new(tex_size, palette::FIELD));
        if buf.size != tex_size {
            *buf = ColorImage::new(tex_size, palette::FIELD);
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
                paint_cell(buf, &life.grid.cells[i], x, y, cell_px);
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

    /// ЛКМ рисует выбранным видом, ПКМ стирает; при перетаскивании клетки
    /// ложатся линией без пропусков.
    #[allow(clippy::too_many_arguments)]
    fn handle_input(
        &mut self,
        ui: &mut Ui,
        life: &mut Life,
        rect: egui::Rect,
        cell_px: f32,
        w: usize,
        h: usize,
        brush: Species,
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

        if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary)) {
            if let Some(cell) = hovered.and_then(cell_at) {
                life.set_cell(cell.0, cell.1, Some(brush));
                self.drag = Some((cell.0, cell.1, Some(brush)));
            }
        }
        if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Secondary)) {
            if let Some(cell) = hovered.and_then(cell_at) {
                life.set_cell(cell.0, cell.1, None);
                self.drag = Some((cell.0, cell.1, None));
            }
        }

        match self.drag {
            Some((px, py, painted)) if ui.input(|i| i.pointer.any_down()) => {
                if let Some((x, y)) = hovered.and_then(cell_at) {
                    if (x, y) != (px, py) {
                        draw_line(life, painted, px, py, x, y);
                    }
                    self.drag = Some((x, y, painted));
                }
            }
            _ => self.drag = None,
        }
    }
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

/// Цвет клетки по её виду: добыча чёрная, хищник красный, пусто — поле.
fn cell_color(cell: &Cell) -> Color32 {
    match cell.species {
        Some(Species::Prey) => palette::PREY,
        Some(Species::Predator) => palette::PREDATOR,
        None => palette::FIELD,
    }
}

/// Fills the `cell_px` square at `(x, y)` with its colour and a black border.
/// Borders are skipped entirely at `cell_px == 1`: one pixel per cell has no
/// room for them.
fn paint_cell(img: &mut ColorImage, cell: &Cell, x: usize, y: usize, cell_px: usize) {
    let fill = cell_color(cell);
    let b = BORDER_PX.min(cell_px.saturating_sub(1));
    for py in 0..cell_px {
        for px in 0..cell_px {
            let idx = (y * cell_px + py) * img.size[0] + (x * cell_px + px);
            let on_border = b > 0 && (px < b || py < b || px >= cell_px - b || py >= cell_px - b);
            img.pixels[idx] = if on_border { palette::BORDER } else { fill };
        }
    }
}

/// Bresenham line so dragging does not leave gaps between sampled cells.
fn draw_line(
    life: &mut Life,
    painted: Option<Species>,
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
        if x >= 0 && y >= 0 && (x as usize) < life.grid.width && (y as usize) < life.grid.height {
            life.set_cell(x as usize, y as usize, painted);
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
