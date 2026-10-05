use egui::{vec2, Color32, Painter, Pos2, Rect, Stroke, Ui};

use crate::sim::{Sim, SimStatus};

/// Vector overlays plus the Game Over / cycle banner.
pub fn draw(ui: &Ui, sim: &Sim, rect: Rect, cell_px: usize) {
    let painter = ui.painter();
    if sim.render.show_construction_outlines {
        draw_construction_outlines(painter, sim, rect, cell_px);
    }
    if sim.render.show_conflict_radius {
        draw_conflict_radii(painter, sim, rect, cell_px);
    }
    if matches!(sim.status, SimStatus::GameOver | SimStatus::CycleDetected) {
        draw_banner(ui, painter, sim);
    }
}

fn draw_conflict_radii(painter: &Painter, sim: &Sim, rect: Rect, cell_px: usize) {
    let r = sim.config.conflict_radius as f32 * cell_px as f32;
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
                rect.min.x + (x as f32 + 0.5) * cell_px as f32,
                rect.min.y + (y as f32 + 0.5) * cell_px as f32,
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

/// §8.6: outlines each construction complex in the colour of its kind. Only the
/// edges facing outside the complex are stroked, so a 3x3 zone gets one box
/// rather than a grid of nine.
fn draw_construction_outlines(painter: &Painter, sim: &Sim, rect: Rect, cell_px: usize) {
    let w = sim.grid.width;
    let px = cell_px as f32;

    let same_complex = |x: usize, y: usize, id: Option<u32>| -> bool {
        let o = &sim.grid.cells[y * w + x];
        o.construction_id == id
            && matches!(
                o.state,
                crate::sim::cell::CellState::Construction | crate::sim::cell::CellState::Overlap
            )
    };

    for y in 0..sim.grid.height {
        for x in 0..w {
            let c = &sim.grid.cells[y * w + x];
            if !matches!(
                c.state,
                crate::sim::cell::CellState::Construction | crate::sim::cell::CellState::Overlap
            ) {
                continue;
            }
            let id = c.construction_id;
            let stroke = Stroke::new(1.0_f32, kind_color(c.kind));
            let cell = Rect::from_min_max(
                Pos2::new(rect.min.x + x as f32 * px, rect.min.y + y as f32 * px),
                Pos2::new(
                    rect.min.x + (x + 1) as f32 * px,
                    rect.min.y + (y + 1) as f32 * px,
                ),
            );
            if x == 0 || !same_complex(x - 1, y, id) {
                painter.line_segment([cell.left_top(), cell.left_bottom()], stroke);
            }
            if x + 1 >= w || !same_complex(x + 1, y, id) {
                painter.line_segment([cell.right_top(), cell.right_bottom()], stroke);
            }
            if y == 0 || !same_complex(x, y - 1, id) {
                painter.line_segment([cell.left_top(), cell.right_top()], stroke);
            }
            if y + 1 >= sim.grid.height || !same_complex(x, y + 1, id) {
                painter.line_segment([cell.left_bottom(), cell.right_bottom()], stroke);
            }
        }
    }
}

fn kind_color(kind: Option<crate::sim::cell::ConstructionKind>) -> Color32 {
    use crate::sim::cell::ConstructionKind::*;
    match kind {
        Some(Water) => crate::ui::palette::WATER,
        Some(Food) => crate::ui::palette::FOOD,
        Some(Energy) => crate::ui::palette::ENERGY,
        Some(Population) => crate::ui::palette::POPULATION,
        Some(Conflict) => crate::ui::palette::CONFLICT,
        None => crate::ui::palette::OVERLAP,
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
