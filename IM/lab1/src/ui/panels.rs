use egui::Ui;

use crate::app::App;

/// Верхняя панель: управление запуском, очистка, случайное заполнение и
/// скорость. Больше в игре нет ничего, что можно крутить.
pub fn toolbar(ui: &mut Ui, app: &mut App) {
    ui.horizontal_wrapped(|ui| {
        let label = if app.running {
            "Пауза"
        } else {
            "Пуск"
        };
        if ui
            .button(label)
            .on_hover_text("Запустить или остановить игру")
            .clicked()
        {
            app.toggle_running();
        }
        if ui
            .button("Шаг")
            .on_hover_text("Ровно одно поколение")
            .clicked()
        {
            app.step_once();
        }
        if ui
            .button("Очистить")
            .on_hover_text("Убить все клетки и начать заново")
            .clicked()
        {
            app.clear();
        }

        ui.separator();
        ui.label("Заполненность:");
        ui.add(
            egui::DragValue::new(&mut app.density)
                .range(0.0..=1.0)
                .speed(0.01)
                .fixed_decimals(2),
        );
        if ui
            .button("Случайно")
            .on_hover_text("Заполнить поле случайными клетками")
            .clicked()
        {
            app.randomize();
        }

        ui.separator();
        ui.label("Поколений в секунду:");
        ui.add(
            egui::DragValue::new(&mut app.ticks_per_second)
                .range(1.0..=60.0)
                .speed(1.0),
        );
    });
}

/// Нижняя строка: поколение, живые клетки, размер поля, сообщения и Game Over.
pub fn status_bar(ui: &mut Ui, app: &App) {
    ui.horizontal(|ui| {
        ui.label(format!(
            "{} | поколение {} | живых {} | сетка {}×{} | клетка {} px",
            if app.running {
                "играет"
            } else {
                "пауза"
            },
            app.life.generation,
            app.life.alive,
            app.life.grid.width,
            app.life.grid.height,
            app.canvas.cell_px,
        ));
        if let Some(cycle) = app.life.cycle {
            ui.separator();
            let text = if app.life.alive == 0 {
                "Game Over: колония вымерла".to_string()
            } else {
                format!("Game Over: мир повторился, цикл {cycle} поколений")
            };
            ui.colored_label(egui::Color32::RED, text);
        } else if !app.message.is_empty() {
            ui.separator();
            ui.colored_label(egui::Color32::LIGHT_RED, &app.message);
        }
    });
}
