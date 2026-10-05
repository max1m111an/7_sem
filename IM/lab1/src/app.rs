use std::time::Instant;

use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::sim::{Sim, SimStatus};

/// Upper bound on ticks per frame, re-exported so tests can assert the cap.
pub use crate::sim::MAX_TICKS_PER_FRAME;
use crate::ui::canvas::Canvas;
use crate::ui::panels;
use crate::ui::tools::Tool;

pub const SAVE_PATH: &str = "save.json";
/// Toolbar-only settings, kept beside the simulation save.
pub const UI_SAVE_PATH: &str = "save.json.ui";

#[derive(Serialize, Deserialize)]
struct UiState {
    ticks_per_second: f32,
    colony_size: usize,
    stamp_rotation: usize,
}

/// Ticks due for `elapsed` seconds at `rate` ticks per second, capped at
/// `MAX_TICKS_PER_FRAME`.
///
/// The fractional remainder stays in `accumulator`, otherwise a rate below the
/// frame rate would round down to zero every frame and never advance. When the
/// backlog exceeds the cap the remainder is dropped, so a slow tick cannot
/// cause the simulation to spiral into an ever-growing backlog.
pub fn budget_for(accumulator: &mut f32, elapsed: f32, rate: f32) -> usize {
    *accumulator += elapsed * rate.clamp(1.0, 60.0);
    let budget = accumulator.floor().max(0.0) as usize;
    if budget > MAX_TICKS_PER_FRAME {
        *accumulator = 0.0;
        MAX_TICKS_PER_FRAME
    } else {
        *accumulator -= budget as f32;
        budget
    }
}

pub struct App {
    pub sim: Sim,
    pub canvas: Canvas,
    pub tool: Tool,
    pub ticks_per_second: f32,
    pub colony_size: usize,
    pub stamp_rotation: usize,
    pub message: String,
    drag: Option<(usize, usize)>,
    last_tick: Instant,
    /// Fractional ticks carried between frames so a rate below the frame rate
    /// still advances.
    tick_accumulator: f32,
    canvas_rect: egui::Rect,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .init();
        let sim = Sim::with_defaults(crate::sim::DEFAULT_WIDTH, crate::sim::DEFAULT_HEIGHT);
        Self {
            sim,
            canvas: Canvas::new(),
            tool: Tool::Citizen,
            ticks_per_second: 10.0,
            colony_size: 40,
            stamp_rotation: 0,
            message: String::new(),
            drag: None,
            last_tick: Instant::now(),
            tick_accumulator: 0.0,
            canvas_rect: egui::Rect::NOTHING,
        }
        .with_saved_ui(cc)
    }

    fn with_saved_ui(mut self, cc: &eframe::CreationContext<'_>) -> Self {
        // The UI state lives in a sidecar file so it cannot be confused with the
        // simulation save, which is a `Sim` payload.
        if let Ok(text) = std::fs::read_to_string(UI_SAVE_PATH) {
            match serde_json::from_str::<UiState>(&text) {
                Ok(state) => {
                    self.ticks_per_second = state.ticks_per_second;
                    self.colony_size = state.colony_size;
                    self.stamp_rotation = state.stamp_rotation;
                }
                Err(err) => self.message = format!("Не удалось прочитать UI: {err}"),
            }
        }
        if let Ok(text) = std::fs::read_to_string(SAVE_PATH) {
            match self.sim.load_json(&text) {
                Ok(()) => {
                    self.canvas.invalidate();
                    self.message = "Сохранение загружено.".to_string();
                }
                Err(err) => self.message = format!("Не удалось загрузить: {err}"),
            }
        }
        cc.egui_ctx.set_fonts(egui::FontDefinitions::default());
        self
    }

    fn ui_state(&self) -> UiState {
        UiState {
            ticks_per_second: self.ticks_per_second,
            colony_size: self.colony_size,
            stamp_rotation: self.stamp_rotation,
        }
    }

    fn write_ui_state(&mut self) {
        let path = format!("{SAVE_PATH}.ui");
        match std::fs::write(
            &path,
            serde_json::to_string_pretty(&self.ui_state()).unwrap_or_default(),
        ) {
            Ok(()) => self.message = format!("Сохранено в {SAVE_PATH} и {path}."),
            Err(err) => self.message = format!("Ошибка сохранения: {err}"),
        }
    }

    pub fn toggle_running(&mut self) {
        match self.sim.status {
            SimStatus::Running => self.sim.status = SimStatus::Paused,
            SimStatus::Paused => {
                self.sim.status = SimStatus::Running;
                self.last_tick = Instant::now();
                self.message.clear();
            }
            _ => self.message = "Симуляция остановлена. Нажмите Reset.".to_string(),
        }
    }

    pub fn step_once(&mut self) {
        self.sim.status = SimStatus::Paused;
        self.message.clear();
        if !self.sim.step() {
            self.report_stop();
        }
    }

    pub fn reset(&mut self) {
        self.sim.reset();
        self.canvas.invalidate();
        self.message = "Колония сброшена.".to_string();
        self.drag = None;
    }

    pub fn generate(&mut self) {
        self.sim.generate_colony(self.colony_size);
        self.canvas.invalidate();
        self.message = format!("Размещено граждан: {}", self.colony_size);
    }

    pub fn save(&mut self) {
        match self.sim.save() {
            Ok(json) => match std::fs::write(SAVE_PATH, json) {
                Ok(()) => self.write_ui_state(),
                Err(err) => self.message = format!("Ошибка сохранения: {err}"),
            },
            Err(err) => self.message = format!("Ошибка сериализации: {err}"),
        }
    }

    pub fn load(&mut self) {
        match std::fs::read_to_string(SAVE_PATH) {
            Ok(text) => match self.sim.load_json(&text) {
                Ok(()) => {
                    self.canvas.invalidate();
                    if let Some(state) = std::fs::read_to_string(UI_SAVE_PATH)
                        .ok()
                        .and_then(|t| serde_json::from_str::<UiState>(&t).ok())
                    {
                        self.ticks_per_second = state.ticks_per_second;
                        self.colony_size = state.colony_size;
                        self.stamp_rotation = state.stamp_rotation;
                    }
                    self.message = format!("Загружено из {SAVE_PATH}.");
                }
                Err(err) => self.message = format!("Ошибка загрузки: {err}"),
            },
            Err(err) => self.message = format!("Файл не найден: {err}"),
        }
    }

    fn report_stop(&mut self) {
        match self.sim.status {
            SimStatus::GameOver => {
                self.message = "GAME OVER: граждан не осталось.".to_string();
            }
            SimStatus::CycleDetected => {
                let len = self.sim.cycle_length.unwrap_or(0);
                self.message = format!("Цикл обнаружен ({len} тиков) — колония сброшена.");
                self.sim.reset();
                self.canvas.invalidate();
            }
            _ => {}
        }
    }

    /// Ticks due for `elapsed` seconds at the configured rate.
    fn budget(&mut self, elapsed: f32) -> usize {
        let rate = self.ticks_per_second;
        budget_for(&mut self.tick_accumulator, elapsed, rate)
    }

    /// Runs ticks on a fixed-rate budget and keeps repainting on its own, so the
    /// simulation animates without any pointer activity.
    ///
    /// The accumulator keeps the fractional part of `elapsed * rate`, otherwise a
    /// rate below the frame rate would round down to zero ticks every frame and the
    /// colony would never advance. A new frame is requested whenever the sim is
    /// running, including on frames that had no tick to run.
    pub fn advance(&mut self, ctx: &egui::Context) {
        if self.sim.status != SimStatus::Running {
            return;
        }
        let dt = self.last_tick.elapsed().as_secs_f32();
        self.last_tick = Instant::now();
        let budget = self.budget(dt);

        for _ in 0..budget {
            if !self.sim.step() {
                self.report_stop();
                ctx.request_repaint();
                return;
            }
        }
        ctx.request_repaint();
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.advance(ctx);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            panels::toolbar(ui, self);
        });
        egui::SidePanel::left("tools")
            .exact_width(220.0)
            .show(ctx, |ui| {
                panels::tool_panel(ui, self);
            });
        egui::SidePanel::right("params")
            .exact_width(320.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    panels::params_panel(ui, self);
                    ui.separator();
                    panels::render_panel(ui, self);
                    ui.separator();
                    panels::stats_panel(ui, self);
                });
            });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            panels::status_bar(ui, self);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect_before = ui.max_rect();
            self.canvas_rect = rect_before;
            let mut canvas = std::mem::take(&mut self.canvas);
            let mut drag = self.drag.take();
            canvas.show(ui, &mut self.sim, self.tool, self.stamp_rotation, &mut drag);
            self.canvas = canvas;
            self.drag = drag;
            crate::ui::overlays::draw(ui, &self.sim, self.canvas_rect, self.sim.render.cell_px);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        let path = format!("{SAVE_PATH}.ui");
        if let Ok(text) = serde_json::to_string_pretty(&self.ui_state()) {
            let _ = std::fs::write(path, text);
        }
    }
}
