use eframe::egui;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::time::{Duration, Instant};

use crate::config::{RenderConfig, SimConfig};
use crate::sim::{
    cell::{Cell, CellState},
    grid::Grid,
    rules::tick,
};
use crate::ui::canvas::{bresenham_line, draw_grid};

#[derive(PartialEq, Clone, Copy)]
enum Tool {
    Phyto,
    Zoo,
}

pub struct PhytoZooApp {
    grid_current: Grid,
    grid_next: Grid,
    sim_config: SimConfig,
    render_config: RenderConfig,
    rng: ChaCha8Rng,

    running: bool,
    game_over: bool,
    ticks: usize,
    tps: usize,
    last_update: Instant,

    active_tool: Tool,
    gen_zoo: usize,
    gen_phyto: usize,
    last_pointer_pos: Option<egui::Pos2>,
    cluster_radius_phyto: i32,
    cluster_radius_zoo: i32,
}

impl PhytoZooApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Устанавливаем белую тему для интерфейса
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        let w = 128;
        let h = 128;
        Self {
            grid_current: Grid::new(w, h),
            grid_next: Grid::new(w, h),
            sim_config: SimConfig::default(),
            render_config: RenderConfig::default(),
            rng: ChaCha8Rng::seed_from_u64(42),
            running: false,
            game_over: false,
            ticks: 0,
            tps: 10,
            last_update: Instant::now(),
            active_tool: Tool::Phyto, // По умолчанию рисуем фито
            gen_zoo: 500,
            gen_phyto: 1000,
            last_pointer_pos: None,
            cluster_radius_phyto: 6,
            cluster_radius_zoo: 4,
        }
    }

    fn apply_tool(&mut self, x: usize, y: usize, erase: bool) {
        if x >= self.grid_current.width || y >= self.grid_current.height {
            return;
        }
        let idx = y * self.grid_current.width + x;

        if erase {
            self.grid_current.cells[idx] = Cell::empty();
        } else {
            self.grid_current.cells[idx] = match self.active_tool {
                Tool::Phyto => Cell::phyto(),
                Tool::Zoo => Cell::zoo(self.sim_config.child_start_energy * 2.0),
            };
        }
    }

    fn reset(&mut self) {
        self.grid_current = Grid::new(self.grid_current.width, self.grid_current.height);
        self.ticks = 0;
        self.running = false;
        self.game_over = false;
    }

    fn generate_colony(&mut self) {
        // Плотность кластеров: чем больше клеток, тем больше пятен,
        // но каждое пятно остаётся компактным.
        let phyto_clusters = (self.gen_phyto as f32 / 60.0).ceil().max(1.0) as usize;
        let zoo_clusters = (self.gen_zoo as f32 / 25.0).ceil().max(1.0) as usize;

        // Радиус пятна растёт медленно (корень), чтобы пятна не расплывались.
        let phyto_radius = ((self.gen_phyto as f32).sqrt() / 2.5).max(2.0).round() as i32;
        let zoo_radius = ((self.gen_zoo as f32).sqrt() / 2.5).max(1.0).round() as i32;

        self.scatter_clusters(
            CellState::Phyto,
            self.gen_phyto,
            phyto_clusters,
            phyto_radius,
        );
        self.scatter_clusters(CellState::Zoo, self.gen_zoo, zoo_clusters, zoo_radius);

        self.game_over = false;
    }

    fn scatter_clusters(
        &mut self,
        kind: CellState,
        total: usize,
        cluster_count: usize,
        radius: i32,
    ) {
        if total == 0 || cluster_count == 0 {
            return;
        }

        let w = self.grid_current.width as i32;
        let h = self.grid_current.height as i32;

        // Сколько клеток на кластер. Остаток раздаём первым кластерам.
        let base = total / cluster_count;
        let remainder = total % cluster_count;

        for c in 0..cluster_count {
            // Случайный центр кластера
            let cx = self.rng.gen_range(0..w);
            let cy = self.rng.gen_range(0..h);

            // Количество клеток в этом кластере (с учётом остатка)
            let per_cluster = base + if c < remainder { 1 } else { 0 };
            if per_cluster == 0 {
                continue;
            }

            // Собираем координаты-кандидаты вокруг центра,
            // сортируем по удалённости и берём ближайшие.
            // Это даёт компактное пятно вместо размытого облака.
            let mut candidates: Vec<(i32, i32, i32)> = Vec::new();
            // (dx, dy, dist2)

            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let dist2 = dx * dx + dy * dy;
                    // Ограничиваем пятно кругом радиуса radius
                    if dist2 > radius * radius {
                        continue;
                    }
                    // Небольшая случайность в приоритете: чуть перемешиваем
                    // "стоимость" клетки, чтобы пятно не было идеальным кругом.
                    let jitter = self.rng.gen_range(0..=radius * radius);
                    candidates.push((dx, dy, dist2 * 2 + jitter));
                }
            }

            candidates.sort_by_key(|&(_, _, cost)| cost);

            let mut placed = 0usize;
            for (dx, dy, _) in candidates {
                if placed >= per_cluster {
                    break;
                }

                let x = (cx + dx).rem_euclid(w);
                let y = (cy + dy).rem_euclid(h);
                let idx = (y * w + x) as usize;

                // Не перезаписываем уже занятую клетку
                if !matches!(self.grid_current.cells[idx].state, CellState::Empty) {
                    continue;
                }

                self.grid_current.cells[idx] = match kind {
                    CellState::Phyto => Cell::phyto(),
                    CellState::Zoo => Cell::zoo(self.sim_config.child_start_energy * 2.0),
                    CellState::Empty => unreachable!(),
                };
                placed += 1;
            }
        }
    }

    fn step(&mut self) {
        let alive = tick(
            &self.grid_current,
            &mut self.grid_next,
            &self.sim_config,
            &mut self.rng,
        );
        std::mem::swap(&mut self.grid_current, &mut self.grid_next);
        self.ticks += 1;
        if !alive {
            self.running = false;
            self.game_over = true;
        }
    }
}

impl eframe::App for PhytoZooApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running
            && self.last_update.elapsed() >= Duration::from_secs_f32(1.0 / self.tps as f32)
        {
            self.step();
            self.last_update = Instant::now();
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tool, Tool::Phyto, "Кисть: Фито");
                ui.selectable_value(&mut self.active_tool, Tool::Zoo, "Кисть: Зоо");
                // Кисть Empty убрана из тулбара

                ui.separator();
                if ui
                    .button(if self.running {
                        "Пауза"
                    } else {
                        "Старт"
                    })
                    .clicked()
                {
                    self.running = !self.running;
                }
                if ui.button("Шаг").clicked() {
                    self.step();
                }
                if ui.button("Очистить").clicked() {
                    self.reset();
                }
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Параметры");
            ui.add(
                egui::Slider::new(&mut self.sim_config.food_value, 0.0..=100.0)
                    .step_by(0.5)
                    .text("Питательность фито"),
            );
            ui.add(
                egui::Slider::new(&mut self.sim_config.reproduction_cost, 0.0..=200.0)
                    .step_by(1.0)
                    .text("Цена размножения"),
            );
            ui.add(
                egui::Slider::new(&mut self.sim_config.child_start_energy, 0.0..=100.0)
                    .step_by(0.5)
                    .text("Энергия потомка"),
            );
            ui.add(egui::Slider::new(&mut self.tps, 1..=60).text("Тиков в секунду"));

            ui.separator();
            ui.add(egui::Slider::new(&mut self.gen_zoo, 0..=1000).text("Количество зоо"));
            ui.add(egui::Slider::new(&mut self.gen_phyto, 0..=2000).text("Количество фито"));
            if ui.button("Сгенерировать колонию").clicked() {
                self.generate_colony();
            }

            ui.separator();
            ui.heading("Статистика");
            let phyto_c = self
                .grid_current
                .cells
                .iter()
                .filter(|c| c.state == CellState::Phyto)
                .count();
            let zoo_c = self
                .grid_current
                .cells
                .iter()
                .filter(|c| c.state == CellState::Zoo)
                .count();
            let total_e: f32 = self
                .grid_current
                .cells
                .iter()
                .filter(|c| c.state == CellState::Zoo)
                .map(|c| c.energy)
                .sum();
            let avg_e = if zoo_c > 0 {
                total_e / zoo_c as f32
            } else {
                0.0
            };

            ui.label(format!("Поколение: {}", self.ticks));
            ui.label(format!("Фито: {}", phyto_c));
            ui.label(format!("Зоо: {}", zoo_c));
            ui.label(format!("Ср. энергия: {:.1}", avg_e));

            let status = if self.game_over {
                "Игра окончена"
            } else if self.running {
                "Работает"
            } else {
                "Пауза"
            };
            ui.label(format!("Статус: {}", status));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let response = draw_grid(
                    ui,
                    &self.grid_current,
                    self.render_config.cell_px,
                    self.render_config.border_px,
                );

                // Колесо зума
                if response.hovered() {
                    let scroll = ctx.input(|i| i.raw_scroll_delta.y);
                    if scroll > 0.0 && self.render_config.cell_px < 32 {
                        self.render_config.cell_px += 1;
                    }
                    if scroll < 0.0 && self.render_config.cell_px > 1 {
                        self.render_config.cell_px -= 1;
                    }
                }

                // Взаимодействие (ЛКМ / ПКМ / Drag)
                if response.dragged() || response.clicked() || response.secondary_clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let x = ((pos.x - response.rect.min.x) / self.render_config.cell_px as f32)
                            as isize;
                        let y = ((pos.y - response.rect.min.y) / self.render_config.cell_px as f32)
                            as isize;

                        // Если нажата ПКМ, включаем режим ластика (стирания)
                        let erase = response.secondary_clicked()
                            || ctx.input(|i| i.pointer.secondary_down());

                        if let Some(last_pos) = self.last_pointer_pos {
                            let lx = ((last_pos.x - response.rect.min.x)
                                / self.render_config.cell_px as f32)
                                as isize;
                            let ly = ((last_pos.y - response.rect.min.y)
                                / self.render_config.cell_px as f32)
                                as isize;
                            for (px, py) in bresenham_line(lx, ly, x, y) {
                                self.apply_tool(px, py, erase);
                            }
                        } else {
                            self.apply_tool(x as usize, y as usize, erase);
                        }
                        self.last_pointer_pos = Some(pos);
                    }
                } else {
                    self.last_pointer_pos = None;
                }
            });
        });

        if self.running {
            ctx.request_repaint();
        }
    }
}
