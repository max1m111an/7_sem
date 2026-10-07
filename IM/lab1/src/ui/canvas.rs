use crate::sim::{cell::CellState, grid::Grid};
use eframe::egui;

pub fn draw_grid(
    ui: &mut egui::Ui,
    grid: &Grid,
    cell_px: usize,
    border_px: usize,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            (grid.width * cell_px) as f32,
            (grid.height * cell_px) as f32,
        ),
        egui::Sense::click_and_drag(),
    );

    if ui.is_rect_visible(rect) {
        let painter = ui.painter_at(rect);

        for y in 0..grid.height {
            for x in 0..grid.width {
                let idx = y * grid.width + x;
                let color = match grid.cells[idx].state {
                    CellState::Empty => egui::Color32::from_rgb(255, 255, 255),
                    CellState::Phyto => egui::Color32::from_rgb(46, 204, 64),
                    CellState::Zoo => egui::Color32::from_rgb(0, 0, 0),
                };

                let cell_rect = egui::Rect::from_min_size(
                    rect.min + egui::vec2((x * cell_px) as f32, (y * cell_px) as f32),
                    egui::vec2(cell_px as f32, cell_px as f32),
                );

                if cell_px > 1 && border_px > 0 {
                    painter.rect(
                        cell_rect,
                        0.0,
                        color,
                        egui::Stroke::new(border_px as f32, egui::Color32::BLACK),
                    );
                } else {
                    painter.rect_filled(cell_rect, 0.0, color);
                }
            }
        }
    }
    response
}

// Алгоритм Брезенхэма для драга
pub fn bresenham_line(mut x0: isize, mut y0: isize, x1: isize, y1: isize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 {
            points.push((x0 as usize, y0 as usize));
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    points
}
