use std::time::Instant;

use crate::config::RenderConfig;
use crate::sim::{Brush, Sim, SimStatus};
use crate::ui::canvas::Canvas;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tool {
    Empty,
    Citizen,
    Water,
    Food,
    Energy,
    Population,
    Conflict,
    AntiCell,
    Stamp,
}

#[derive(Clone, Debug)]
pub struct App {
    pub sim: Sim,
    pub canvas: Canvas,
    pub tool: Tool,
    pub brush: Brush,
    pub render_cfg: RenderConfig,
    pub message: String,
    pub ticks_per_second: f32,
    pub last_tick: Instant,
    pub accumulated: f32,
    pub stamp_rot: usize,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "default".to_string(),
            egui::FontData::from_static(b"").into(),
        );
        fonts.families.insert(egui::FontFamily::Proportional, vec!["default".to_string()]);
        cc.egui_ctx.set_fonts(fonts);
        let mut sim = Sim::with_defaults(crate::sim::DEFAULT_WIDTH, crate::sim::DEFAULT_HEIGHT);
        sim.status = SimStatus::Paused;
        Self {
            sim,
            canvas: Canvas::new(),
            tool: Tool::Citizen,
            brush: Brush::Citizen,
            render_cfg: RenderConfig::default(),
            message: String::new(),
            ticks_per_second: 10.0,
            last_tick: Instant::now(),
            accumulated: 0.0,
            stamp_rot: 0,
        }
    }

    pub fn advance(&mut self, ctx: &egui::Context) {
        if self.sim.status == SimStatus::Running {
            let dt = self.last_tick.elapsed().as_secs_f32();
            self.last_tick = Instant::now();
            let rate = self.ticks_per_second.max(0.1).min(60.0);
            self.accumulated += dt * rate;
            let mut ticks = self.accumulated.floor() as i32;
            if ticks > 6 { ticks = 6; }
            if ticks < 0 { ticks = 0; }
            self.accumulated -= ticks as f32;
            let mut did_step = false;
            for _ in 0..ticks {
                if self.sim.step() {
                    did_step = true;
                } else {
                    break;
                }
            }
            if did_step || self.sim.status == SimStatus::Running {
                ctx.request_repaint();
            }
            if self.sim.status == SimStatus::GameOver {
                self.message = "GAME OVER: ���?����?���? �� ���?�?�?�?�?�?�?".to_string();
                ctx.request_repaint();
            }
        }
    }
}

pub fn brush_for_tool(tool: &Tool) -> Brush {
    match tool {
        Tool::Empty => Brush::Empty,
        Tool::Citizen => Brush::Citizen,
        Tool::Water => Brush::Water,
        Tool::Food => Brush::Food,
        Tool::Energy => Brush::Energy,
        Tool::Population => Brush::Population,
        Tool::Conflict => Brush::Conflict,
        Tool::AntiCell => Brush::AntiCell,
        Tool::Stamp => Brush::Citizen,
    }
}






impl eframe::App for crate::app::App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.advance(ctx);
        let mut canvas = std::mem::take(&mut self.canvas);
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            crate::ui::panels::toolbar(ui, self);
        });
        egui::SidePanel::left("tools").show(ctx, |ui| {
            crate::ui::panels::tool_panel(ui, self);
        });
        egui::SidePanel::right("params").show(ctx, |ui| {
            crate::ui::panels::params_panel(ui, self);
            ui.separator();
            crate::ui::panels::stats_panel(ui, self);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            canvas.show(ui, self);
        });
        self.canvas = canvas;
    }
}









