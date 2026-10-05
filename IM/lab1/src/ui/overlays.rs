use egui::{vec2, Color32, Painter, Pos2, Rect, Stroke, Ui};

use crate::sim::{Sim, SimStatus};

/// Vector overlays plus the Game Over / cycle banner. `cell_px` is the
/// on-screen size of a cell, which may differ from `render.cell_px` by the
/// sub-pixel stretch the canvas applies to fill the panel.
pub fn draw(ui: &Ui, sim: &Sim, rect: Rect, cell_px: f32) {
    let painter = ui.painter();

    if sim.render.show_conflict_radius {
        draw_conflict_radii(painter, sim, rect, cell_px);
    }
    if matches!(sim.status, SimStatus::GameOver | SimStatus::CycleDetected) {
        draw_banner(ui, painter, sim);
    }
}

fn draw_conflict_radii(painter: &Painter, sim: &Sim, rect: Rect, cell_px: f32) {
    let r = sim.config.conflict_radius as f32 * cell_px;
    for y in 0..sim.grid.height {
        for x in 0..sim.grid.width {
            let c = &sim.grid.cells[y * sim.grid.width + x];
            if c.kind != Some(crate::sim::cell::ConstructionKind::Conflict) {
                continue;
            }
            if !matches!(c.state, crate::sim::cell::CellState::Construction) {
                continue;
            }
            let center = Pos2::new(
                rect.min.x + (x as f32 + 0.5) * cell_px,
                rect.min.y + (y as f32 + 0.5) * cell_px,
            );
            painter.circle_stroke(
                center,
                r,
                Stroke::new(
                    1.0_f32,
                    Color32::from_rgba_unmultiplied(0x8A, 0x2B, 0xE2, 90),
                ),
            );
        }
    }
}

fn draw_banner(ui: &Ui, painter: &Painter, sim: &Sim) {
    let rect = Rect::from_center_size(ui.max_rect().center(), ui.max_rect().size() * 0.6);
    painter.rect_filled(rect, 12.0, Color32::from_black_alpha(220));
    let (title, body) = match sim.status {
        SimStatus::CycleDetected => (
            "ЦИКЛ ОБНАРУЖЕН",
            match sim.cycle_length {
                Some(len) => format!(
                    "Состояние повторилось через {} тиков.\nКолония сброшена.",
                    len
                ),
                None => "Состояние повторилось.\nКолония сброшена.".to_string(),
            },
        ),
        _ => ("GAME OVER", "Граждан не осталось.".to_string()),
    };
    painter.text(
        rect.center() - vec2(0.0, 14.0),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(28.0),
        Color32::WHITE,
    );
    painter.text(
        rect.center() + vec2(0.0, 16.0),
        egui::Align2::CENTER_CENTER,
        body,
        egui::FontId::proportional(14.0),
        Color32::LIGHT_GRAY,
    );
}
