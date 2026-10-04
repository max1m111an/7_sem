use crate::config::RenderConfig;
use crate::sim::{Brush, CellState, ConstructionKind, Sim};
use egui::{ TextureHandle, TextureOptions, Ui};

#[derive(Clone, Default)] pub struct Canvas {
    pub tex: Option<TextureHandle>,
    pub last_img: Option<egui::ColorImage>,
    pub drag: Option<(usize, usize)>,
}

impl Canvas {
    pub const MAX_TEXTURE_SIDE: usize = 2048;
    pub fn new() -> Self {
        Self { tex: None, last_img: None, drag: None }
    }
    pub fn show(&mut self, ui: &mut Ui, app: &mut crate::app::App) {
        let grid = &app.sim.grid;
        let (w, h) = (grid.width, grid.height);
        let mut cell_px = app.render_cfg.cell_px;
        if cell_px == 0 { cell_px = 1; }
        if cell_px > 32 { cell_px = 32; }
        let max_c = Self::max_cell_px(w, h);
        if cell_px > max_c { cell_px = max_c; }
        app.render_cfg.cell_px = cell_px;
        let img = self.render(&app.sim, &app.render_cfg);
        let tex = self.get_or_update_texture(ui.ctx(), &img);
        let size = [w * cell_px, h * cell_px];
        let resp = ui.image((tex.id(), egui::Vec2::new(size[0] as f32, size[1] as f32)));
        self.handle_input(ui, app, &resp);
    }
    fn max_cell_px(w: usize, h: usize) -> usize {
        let max_side = Self::MAX_TEXTURE_SIDE;
        let s = if w == 0 || h == 0 { 1 } else { ((max_side as f32) / (w.max(h) as f32)).floor() as usize };
        if s == 0 { 1 } else { s.min(32) }
    }
    fn render(&self, sim: &Sim, cfg: &RenderConfig) -> egui::ColorImage {
        let w = sim.grid.width;
        let h = sim.grid.height;
        let mut img = egui::ColorImage::new([w, h], crate::ui::palette::Palette::empty());
        for y in 0..h {
            for x in 0..w {
                let c = sim.grid.get(x, y);
                let col = match c.state {
                    CellState::Empty => crate::ui::palette::Palette::empty(),
                    CellState::Citizen => crate::ui::palette::Palette::citizen(),
                    CellState::AntiCell => crate::ui::palette::Palette::anticell(),
                    CellState::Construction | CellState::Overlap => match c.kind {
                        Some(ConstructionKind::Water) => crate::ui::palette::Palette::water(),
                        Some(ConstructionKind::Food) => crate::ui::palette::Palette::food(),
                        Some(ConstructionKind::Energy) => crate::ui::palette::Palette::energy(),
                        Some(ConstructionKind::Population) => crate::ui::palette::Palette::population(),
                        Some(ConstructionKind::Conflict) => crate::ui::palette::Palette::conflict(),
                        None => crate::ui::palette::Palette::citizen(),
                    },
                };
                img[(x, y)] = col;
            }
        }
        img
    }
    fn get_or_update_texture(&mut self, ctx: &egui::Context, img: &egui::ColorImage) -> &TextureHandle {
        if self.last_img.as_ref().map_or(true, |l| l.size != img.size || l.pixels != img.pixels) {
            let tex = ctx.load_texture("grid", egui::ImageData::Color(img.clone().into()), TextureOptions::default());
            self.tex = Some(tex);
            self.last_img = Some(img.clone());
        }
        self.tex.as_ref().unwrap()
    }
    fn handle_input(&mut self, ui: &mut Ui, app: &mut crate::app::App, resp: &egui::Response) {
        let rect = resp.rect;
        if rect.width() <= 0.0 || rect.height() <= 0.0 { return; }
        let cell_px = app.render_cfg.cell_px as f32;
        if let Some(pos) = resp.interact_pointer_pos() {
            let p = pos - rect.min;
            let x = (p.x / cell_px) as usize;
            let y = (p.y / cell_px) as usize;
            if x < app.sim.grid.width && y < app.sim.grid.height {
                if resp.drag_started() { self.drag = Some((x, y)); }
                if resp.dragged() {
                    if let Some(_d) = self.drag {
                        crate::ui::tools::apply_tool_at(&mut app.sim, &app.tool, x, y);
                        app.sim.update_stats();
                    }
                }
                if resp.clicked() {
                    crate::ui::tools::apply_tool_at(&mut app.sim, &app.tool, x, y);
                    app.sim.update_stats();
                }
            }
        }
        if resp.drag_stopped() { self.drag = None; }
    }
}
















