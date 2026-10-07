use super::{
    cell::{Cell, CellState},
    grid::Grid,
};
use crate::config::SimConfig;
use rand::Rng;
use std::collections::HashMap;

pub fn tick<R: Rng>(current: &Grid, next: &mut Grid, config: &SimConfig, rng: &mut R) -> bool {
    let w = current.width;
    let h = current.height;
    let len = w * h;

    // 0. Очищаем next, переносим существующие Phyto и рождаем новые
    for i in 0..len {
        next.cells[i] = Cell::empty();

        if current.cells[i].state == CellState::Phyto {
            // Фитопланктон просто переносится (не умирает от одиночества)
            next.cells[i] = Cell::phyto();
        } else if current.cells[i].state == CellState::Empty {
            // Проверяем соседей для пустой клетки (N8)
            let n8 = Grid::n8(i, w, h);
            let phyto_neighbors = n8
                .iter()
                .filter(|&&idx| current.cells[idx].state == CellState::Phyto)
                .count();

            // Если рядом хотя бы 3 фито, рождается новый
            if phyto_neighbors >= 3 {
                next.cells[i] = Cell::phyto();
            }
        }
    }

    let mut intentions: HashMap<usize, usize> = HashMap::new(); // куда -> откуда (минимальный)

    // 1 & 2. Сбор намерений и разрешение конфликтов
    for i in 0..len {
        if current.cells[i].state == CellState::Zoo {
            let n4 = Grid::n4(i, w, h);

            let target = if let Some(&phyto_idx) = n4
                .iter()
                .find(|&&idx| current.cells[idx].state == CellState::Phyto)
            {
                phyto_idx // Приоритет 1: Идём на фито
            } else {
                let empties: Vec<usize> = n4
                    .into_iter()
                    .filter(|&idx| current.cells[idx].state == CellState::Empty)
                    .collect();
                if !empties.is_empty() {
                    empties[rng.gen_range(0..empties.len())] // Приоритет 2: Случайная пустая
                } else {
                    i // Приоритет 3: Стоим
                }
            };

            let entry = intentions.entry(target).or_insert(i);
            if i < *entry {
                *entry = i; // Побеждает меньший индекс
            }
        }
    }

    // Определяем проигравших, они остаются на месте
    let mut actual_moves = HashMap::new();
    for i in 0..len {
        if current.cells[i].state == CellState::Zoo {
            let mut moved_to = i;
            for (&to, &from) in &intentions {
                if from == i {
                    moved_to = to;
                    break;
                }
            }
            actual_moves.insert(moved_to, i);
        }
    }

    // 3, 4, 5. Применение движения, поедание, трата энергии и смерть
    let mut zoo_count = 0;
    for (&to, &from) in &actual_moves {
        let mut energy = current.cells[from].energy - 1.0; // Трата на жизнь

        // Если шагнул на Phyto (проверяем по текущему кадру, чтобы не съесть "недоросший" фито)
        if to != from && current.cells[to].state == CellState::Phyto {
            energy += config.food_value;
        }

        if energy > 0.0 {
            next.cells[to] = Cell::zoo(energy);
            zoo_count += 1;
        } else {
            next.cells[to] = Cell::empty(); // Смерть
        }
    }

    // 6. Размножение (почкование)
    for i in 0..len {
        if next.cells[i].state == CellState::Zoo && next.cells[i].energy >= config.reproduction_cost
        {
            let n8 = Grid::n8(i, w, h);
            // Проверяем пустоту по новому кадру (next), чтобы не родить зоопланктон поверх только что сходившего товарища
            let empties: Vec<usize> = n8
                .into_iter()
                .filter(|&idx| next.cells[idx].state == CellState::Empty)
                .collect();

            if !empties.is_empty() {
                let child_idx = empties[rng.gen_range(0..empties.len())];
                next.cells[i].energy -= config.reproduction_cost;
                next.cells[child_idx] = Cell::zoo(config.child_start_energy);
                zoo_count += 1;
            }
        }
    }

    // 7. Проверка конца
    zoo_count > 0
}
