use egui::Ui;

use crate::app::App;
use crate::sim::cell::ConstructionKind;
use crate::sim::SimStatus;
use crate::ui::tools::Tool;

pub fn toolbar(ui: &mut Ui, app: &mut App) {
    ui.horizontal_wrapped(|ui| {
        let running = app.sim.status == SimStatus::Running;
        if ui
            .button(if running { "Pause" } else { "Start" })
            .on_hover_text("Запустить или приостановить симуляцию")
            .clicked()
        {
            app.toggle_running();
        }
        if ui.button("Step").on_hover_text("Один тик").clicked() {
            app.step_once();
        }
        if ui
            .button("Reset")
            .on_hover_text("Сброс с текущим seed")
            .clicked()
        {
            app.reset();
        }
        ui.separator();
        let mut count = app.colony_size;
        if ui
            .add(egui::Slider::new(&mut count, 1..=1000).text("Колония"))
            .changed()
        {
            app.colony_size = count;
        }
        if ui.button("Генератор колонии").clicked() {
            app.generate();
        }
        ui.separator();
        if ui.button("Сохранить").clicked() {
            app.save();
        }
        if ui.button("Загрузить").clicked() {
            app.load();
        }
    });
}

pub fn tool_panel(ui: &mut Ui, app: &mut App) {
    ui.vertical(|ui| {
        ui.heading("Инструменты");
        for tool in crate::ui::tools::ALL_TOOLS {
            ui.selectable_value(&mut app.tool, tool, tool.label());
        }
        if app.tool.is_stamp() {
            ui.add(egui::Slider::new(&mut app.stamp_rotation, 0..=3).text("Поворот"));
        }
    });
}

pub fn params_panel(ui: &mut Ui, app: &mut App) {
    let cfg = &mut app.sim.config;
    ui.heading("Параметры");
    egui::ScrollArea::vertical()
        .max_height(320.0)
        .show(ui, |ui| {
            ui.collapsing("Население", |ui| {
                ui.add(egui::Slider::new(&mut cfg.population_max, 1..=500).text("population_max"));
                ui.add(
                    egui::Slider::new(&mut cfg.food_per_citizen, 0.0..=1.0)
                        .text("food_per_citizen"),
                );
            });
            ui.collapsing("Ресурсы", |ui| {
                ui.add(egui::Slider::new(&mut cfg.food_reserve, 0.0..=50.0).text("Запас еды"));
                ui.add(egui::Slider::new(&mut cfg.water_reserve, 0.0..=50.0).text("Запас воды"));
                ui.add(
                    egui::Slider::new(&mut cfg.energy_reserve, 0.0..=50.0).text("Запас энергии"),
                );
            });
            ui.collapsing(
                "Стресс / усталость / лояльность",
                |ui| {
                    ui.add(egui::Slider::new(&mut cfg.c_max, 10.0..=500.0).text("c_max"));
                    ui.add(egui::Slider::new(&mut cfg.f_max, 10.0..=500.0).text("f_max"));
                    ui.add(egui::Slider::new(&mut cfg.l_max, 10.0..=500.0).text("l_max"));
                    ui.add(
                        egui::Slider::new(&mut cfg.stress_conflict_per_tick, 0.0..=5.0)
                            .text("stress_conflict_per_tick"),
                    );
                    ui.add(
                        egui::Slider::new(&mut cfg.conflict_radius, 1..=10).text("conflict_radius"),
                    );
                },
            );
            ui.collapsing("Движение и антиклетки", |ui| {
                ui.checkbox(&mut cfg.citizen_move_enabled, "citizen_move_enabled");
                ui.add(
                    egui::Slider::new(&mut cfg.citizen_move_prob, 0.0..=1.0)
                        .text("citizen_move_prob"),
                );
                ui.checkbox(&mut cfg.anticell_enabled, "anticell_enabled");
            });
            ui.collapsing("Прочее", |ui| {
                let mut seed = cfg.seed;
                if ui
                    .add(egui::DragValue::new(&mut seed).range(0..=u64::MAX))
                    .changed()
                {
                    cfg.seed = seed;
                }
                ui.label("seed");
                ui.add(
                    egui::Slider::new(&mut app.ticks_per_second, 1.0..=60.0)
                        .text("ticks_per_second"),
                );
            });
        });
}

pub fn render_panel(ui: &mut Ui, app: &mut App) {
    let r = &mut app.sim.render;
    ui.heading("Отображение");
    ui.add(egui::Slider::new(&mut r.cell_px, 1..=32).text("cell_px"));
    ui.add(egui::Slider::new(&mut r.border_px, 0..=3).text("border_px"));
    ui.checkbox(&mut r.show_grid_lines, "show_grid_lines");
    ui.checkbox(&mut r.overlay_stress, "Стресс");
    ui.checkbox(&mut r.overlay_fatigue, "Усталость");
    ui.checkbox(&mut r.overlay_loyalty, "Лояльность");
    ui.checkbox(&mut r.show_construction_outlines, "Контуры конструкций");
    ui.checkbox(&mut r.show_conflict_radius, "Радиус конфликта");
}

pub fn stats_panel(ui: &mut Ui, app: &App) {
    let s = &app.sim.stats;
    let cfg = &app.sim.config;
    ui.heading("Статистика");
    egui::Grid::new("stats")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            ui.label("Поколение");
            ui.label(s.generation.to_string());
            ui.end_row();

            ui.label("Граждан (N)");
            ui.label(s.n_citizen.to_string());
            ui.end_row();

            ui.label("Еда");
            ui.label(format!(
                "{:.1} / нужно {:.1}",
                s.food,
                cfg.food_need(s.n_citizen)
            ));
            ui.end_row();

            ui.label("Вода");
            ui.label(format!(
                "{:.1} / нужно {:.1}",
                s.water,
                cfg.water_need(s.n_citizen)
            ));
            ui.end_row();

            ui.label("Энергия");
            ui.label(format!(
                "{:.1} / нужно {:.1}",
                s.energy,
                cfg.energy_need(s.n_citizen)
            ));
            ui.end_row();

            ui.label("Средний стресс");
            ui.label(format!("{:.1} / {:.0}", s.mean_stress, cfg.c_max));
            ui.end_row();

            ui.label("Средняя усталость");
            ui.label(format!("{:.1} / {:.0}", s.mean_fatigue, cfg.f_max));
            ui.end_row();

            ui.label("Средняя лояльность");
            ui.label(format!("{:.1} / {:.0}", s.mean_loyalty, cfg.l_max));
            ui.end_row();

            ui.label("Пустых клеток");
            ui.label(s.n_empty.to_string());
            ui.end_row();

            for (i, (name, _)) in [
                ("Вода", ConstructionKind::Water),
                ("Еда", ConstructionKind::Food),
                ("Энергия", ConstructionKind::Energy),
                ("Население", ConstructionKind::Population),
                ("Конфликт", ConstructionKind::Conflict),
            ]
            .iter()
            .enumerate()
            {
                ui.label(format!("Конструкций: {}", name));
                ui.label(s.n_constructions[i].to_string());
                ui.end_row();
            }

            ui.label("Антиклеток");
            ui.label(s.n_anticell.to_string());
            ui.end_row();

            ui.label("Перекрытий");
            ui.label(s.n_overlap.to_string());
            ui.end_row();

            ui.label("Статус");
            ui.label(app.sim.status.label());
            ui.end_row();
        });
}

pub fn status_bar(ui: &mut Ui, app: &App) {
    ui.horizontal(|ui| {
        ui.label(format!(
            "{} | поколение {} | N = {}",
            app.sim.status.label(),
            app.sim.generation,
            app.sim.stats.n_citizen
        ));
        if let Some(len) = app.sim.cycle_length {
            ui.separator();
            ui.colored_label(egui::Color32::YELLOW, format!("Цикл: {} тиков", len));
        }
        if !app.message.is_empty() {
            ui.separator();
            ui.colored_label(egui::Color32::LIGHT_RED, &app.message);
        }
    });
}

pub fn tool_label(tool: Tool) -> &'static str {
    tool.label()
}
