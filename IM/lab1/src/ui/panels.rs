use crate::app::{App, Tool};
use crate::sim::{Sim, SimStatus, Stats};
use egui::Ui;

pub fn toolbar(ui: &mut Ui, app: &mut App) {
    ui.horizontal(|ui| {
        if ui.button(if app.sim.status == SimStatus::Running { "Pause" } else { "Start" }).clicked() {
            if app.sim.status == SimStatus::Running { app.sim.status = SimStatus::Paused; } else { app.sim.status = SimStatus::Running; app.last_tick = std::time::Instant::now(); app.accumulated = 0.0; }
        }
        if ui.button("Step").clicked() { let _ = app.sim.step(); app.message.clear(); }
        if ui.button("Reset").clicked() { app.sim.reset(); app.message.clear(); }
        ui.label(format!("Gen: {}", app.sim.generation));
    });
}

pub fn tool_panel(ui: &mut Ui, app: &mut App) {
    ui.vertical(|ui| {
        ui.radio_value(&mut app.tool, Tool::Empty, "Empty");
        ui.radio_value(&mut app.tool, Tool::Citizen, "Citizen");
        ui.radio_value(&mut app.tool, Tool::Water, "Water");
        ui.radio_value(&mut app.tool, Tool::Food, "Food");
        ui.radio_value(&mut app.tool, Tool::Energy, "Energy");
        ui.radio_value(&mut app.tool, Tool::Population, "Population");
        ui.radio_value(&mut app.tool, Tool::Conflict, "Conflict");
        ui.radio_value(&mut app.tool, Tool::AntiCell, "AntiCell");
        ui.radio_value(&mut app.tool, Tool::Stamp, "Stamp");
    });
}

pub fn params_panel(ui: &mut Ui, app: &mut App) {
    ui.vertical(|ui| {
        ui.label("Params");
        ui.add(egui::Slider::new(&mut app.ticks_per_second, 1.0..=60.0).text("TPS"));
        ui.add(egui::Slider::new(&mut app.render_cfg.cell_px, 1..=32).text("Cell px"));
    });
}

pub fn stats_panel(ui: &mut Ui, app: &App) {
    let s = &app.sim.stats;
    ui.vertical(|ui| {
        ui.label(format!("Status: {:?}", app.sim.status));
        ui.label(format!("Citizens: {}", s.n_citizen));
        ui.label(format!("Water: {}", s.n_water));
        ui.label(format!("Food: {}", s.n_food));
        ui.label(format!("Energy: {}", s.n_energy));
        ui.label(format!("Population: {}", s.n_population));
        ui.label(format!("Conflict: {}", s.n_conflict));
        ui.label(format!("AntiCell: {}", s.n_anticell));
        if !app.message.is_empty() { ui.colored_label(egui::Color32::RED, &app.message); }
    });
}
