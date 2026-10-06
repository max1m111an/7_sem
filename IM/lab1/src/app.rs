use std::time::Instant;

use eframe::egui;

use crate::sim::{Life, Species};
use crate::ui::canvas::Canvas;

/// Upper bound on ticks per frame, re-exported so tests can assert the cap.
pub use crate::sim::MAX_TICKS_PER_FRAME;

pub const DEFAULT_TICKS_PER_SECOND: f32 = 10.0;
pub const DEFAULT_DENSITY: f32 = 0.2;

/// Ticks due for `elapsed` seconds at `rate` ticks per second, capped at
/// `MAX_TICKS_PER_FRAME`.
///
/// The fractional remainder stays in `accumulator`, otherwise a rate below the
/// frame rate would round down to zero every frame and never advance. When the
/// backlog exceeds the cap the remainder is dropped, so a slow tick cannot
/// cause the game to spiral into an ever-growing backlog.
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

/// Белая тема и чуть крупные элементы управления: панели чисто-белые, шрифты
/// и отступы больше стандартных egui.
fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let font = |size: f32| egui::FontId::new(size, egui::FontFamily::Proportional);
    style.text_styles.insert(egui::TextStyle::Body, font(16.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, font(16.0));
    style.text_styles.insert(egui::TextStyle::Small, font(13.0));
    style
        .text_styles
        .insert(egui::TextStyle::Heading, font(22.0));
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.interact_size = egui::vec2(36.0, 32.0);

    let mut visuals = egui::Visuals::light();
    visuals.panel_fill = egui::Color32::WHITE;
    visuals.window_fill = egui::Color32::WHITE;
    // Фон числовых полей и прочие «крайние» плашки.
    visuals.extreme_bg_color = egui::Color32::from_rgb(0xF2, 0xF2, 0xF2);
    visuals.faint_bg_color = egui::Color32::from_rgb(0xF7, 0xF7, 0xF7);
    style.visuals = visuals;
    ctx.set_style(style);
}

pub struct App {
    pub life: Life,
    pub canvas: Canvas,
    pub running: bool,
    pub ticks_per_second: f32,
    /// Доля живых клеток для кнопки «Случайно».
    pub density: f32,
    /// Вид, который рисует левая кнопка мыши.
    pub brush: Species,
    pub message: String,
    last_tick: Instant,
    /// Fractional ticks carried between frames so a rate below the frame rate
    /// still advances.
    tick_accumulator: f32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info".into()),
            )
            .init();
        apply_theme(&cc.egui_ctx);
        Self {
            // The canvas sizes this to the window on the first frame.
            life: Life::new(64, 64),
            canvas: Canvas::new(),
            running: false,
            ticks_per_second: DEFAULT_TICKS_PER_SECOND,
            density: DEFAULT_DENSITY,
            brush: Species::Prey,
            message: String::new(),
            last_tick: Instant::now(),
            tick_accumulator: 0.0,
        }
    }

    /// Старт/пауза: запуск не зависит от зацикленности мира — игра просто
    /// продолжает шагать, а статус-бар скромно показывает длину цикла.
    pub fn toggle_running(&mut self) {
        self.running = !self.running;
        if self.running {
            self.last_tick = Instant::now();
            self.tick_accumulator = 0.0;
            self.message.clear();
        }
    }

    /// Один шаг вручную: тот же `Life::step`, только ровно одно поколение.
    pub fn step_once(&mut self) {
        self.running = false;
        self.life.step();
    }

    pub fn clear(&mut self) {
        self.running = false;
        self.life.clear();
        self.canvas.invalidate();
        self.message = "Поле очищено.".to_string();
    }

    pub fn randomize(&mut self) {
        self.life.randomize(self.density);
        self.canvas.invalidate();
        self.message = format!("Случайное поле: {:.0}% живых", self.density * 100.0);
    }

    /// Ticks due for `elapsed` seconds at the configured rate.
    fn budget(&mut self, elapsed: f32) -> usize {
        let rate = self.ticks_per_second;
        budget_for(&mut self.tick_accumulator, elapsed, rate)
    }

    /// Runs ticks on a fixed-rate budget and keeps repainting on its own, so the
    /// game animates without any pointer activity.
    ///
    /// The accumulator keeps the fractional part of `elapsed * rate`, otherwise a
    /// rate below the frame rate would round down to zero ticks every frame and
    /// the world would never advance. A new frame is requested whenever the game
    /// is running, including on frames that had no tick to run.
    pub fn advance(&mut self, ctx: &egui::Context) {
        if !self.running {
            return;
        }
        let dt = self.last_tick.elapsed().as_secs_f32();
        self.last_tick = Instant::now();
        let budget = self.budget(dt);

        for _ in 0..budget {
            self.life.step();
        }
        self.stop_on_cycle();
        ctx.request_repaint();
    }

    /// Game Over — просто пауза: мир повторился или вымер, но игра останавливается
    /// только до следующего «Пуск»/«Шаг», которые продолжают шагать по циклу.
    fn stop_on_cycle(&mut self) {
        if self.life.cycle.is_some() {
            self.running = false;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.advance(ctx);

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            crate::ui::panels::toolbar(ui, self);
        });
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            crate::ui::panels::status_bar(ui, self);
        });
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                self.canvas.show(ui, &mut self.life, self.brush);
            });
    }
}
