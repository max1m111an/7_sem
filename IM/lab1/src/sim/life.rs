//! Игра «Жизнь»: единственное правило B3/S23 на замкнутом поле (торе).
//!
//! Колония из `rules.rs` и соседних модулей осталась в репозитории как
//! справочный код и в игру не вызывается. Весь движок игры — вот этот модуль:
//! сетка, счётчик поколений и число живых клеток.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;

use rand::Rng;

use crate::sim::cell::Cell;
use crate::sim::grid::Grid;

/// Мир «Жизни».
///
/// Поле замкнуто: соседи берутся через край (`Grid::neighbors8`), поэтому
/// клетка, уходящая за правый нижний угол, появляется в левом верхнем.
///
/// За миром ведётся история отпечатков (см. [`Life::cycle`]): конечный мир
/// обязан повториться, и повтор — это Game Over.
#[derive(Clone, Debug)]
pub struct Life {
    pub grid: Grid,
    pub generation: u64,
    pub alive: usize,
    /// Отпечаток мира → поколение, в котором он встретился впервые. История
    /// начинается заново при любой правке мира.
    seen: HashMap<u64, u64>,
    /// `Some(длина цикла)` — мир повторился дальше не развивается; снимается
    /// [`Life::forget_cycles`].
    pub cycle: Option<u64>,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        let mut life = Self {
            grid: Grid::new(width, height),
            generation: 0,
            alive: 0,
            seen: HashMap::new(),
            cycle: None,
        };
        life.remember();
        life
    }

    /// Отпечаток мира: два поколения с одинаковым отпечатком — один и тот же
    /// набор живых клеток, то есть дальше всё повторится.
    fn hash_state(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for cell in &self.grid.cells {
            hasher.write_u8(cell.is_alive() as u8);
        }
        hasher.finish()
    }

    /// Запоминает текущий мир как точку отсчёта истории.
    fn remember(&mut self) {
        let hash = self.hash_state();
        self.seen.insert(hash, self.generation);
    }

    /// Начинает следить за циклами заново с текущего мира: история пуста,
    /// `cycle` снят. Вызывается при любой правке мира и когда игрок после
    /// Game Over просмотрит цикл ещё раз.
    pub fn forget_cycles(&mut self) {
        self.seen.clear();
        self.cycle = None;
        self.remember();
    }

    /// Одно поколение по правилу B3/S23: живая клетка выживает при 2 или 3
    /// живых соседях, мёртвая рождается ровно при 3. Соседство — N8, с
    /// заворотом через край. Поколение увеличивается всегда.
    ///
    /// Если новое состояние уже встречалось в истории, мир зациклился:
    /// `cycle` получает длину цикла в поколениях.
    pub fn step(&mut self) {
        let old = &self.grid;
        let mut next = old.cells.clone();
        let mut alive = 0usize;
        for y in 0..old.height {
            for x in 0..old.width {
                let neighbours = old
                    .neighbors8(x, y)
                    .filter(|(nx, ny)| old.get(*nx, *ny).is_alive())
                    .count();
                let live = neighbours == 3 || (neighbours == 2 && old.get(x, y).is_alive());
                next[old.idx(x, y)] = if live { Cell::alive() } else { Cell::empty() };
                alive += live as usize;
            }
        }
        self.grid.cells = next;
        self.generation += 1;
        self.alive = alive;

        if self.cycle.is_none() {
            let hash = self.hash_state();
            if let Some(first) = self.seen.insert(hash, self.generation) {
                self.cycle = Some(self.generation - first);
            }
        }
    }

    /// Убивает всё и обнуляет поколение.
    pub fn clear(&mut self) {
        for cell in &mut self.grid.cells {
            *cell = Cell::empty();
        }
        self.generation = 0;
        self.alive = 0;
        self.forget_cycles();
    }

    /// Заполняет поле случайным образом: каждая клетка живёт с вероятностью
    /// `density`. Поколение обнуляется — это новый мир.
    pub fn randomize(&mut self, density: f32) {
        let mut rng = rand::thread_rng();
        self.fill(density, &mut rng);
    }

    /// То же с явным генератором, чтобы тесты были воспроизводимыми.
    pub fn fill(&mut self, density: f32, rng: &mut impl Rng) {
        let density = density.clamp(0.0, 1.0);
        let mut alive = 0usize;
        for cell in &mut self.grid.cells {
            let live = rng.gen_bool(f64::from(density));
            *cell = if live { Cell::alive() } else { Cell::empty() };
            alive += live as usize;
        }
        self.alive = alive;
        self.generation = 0;
        self.forget_cycles();
    }

    /// Ставит клетку `(x, y)` в живое/мёртвое состояние и поддерживает счётчик
    /// живых в согласии с сеткой.
    pub fn set_alive(&mut self, x: usize, y: usize, live: bool) {
        let idx = self.grid.idx(x, y);
        let cell = &mut self.grid.cells[idx];
        if cell.is_alive() == live {
            return;
        }
        *cell = if live { Cell::alive() } else { Cell::empty() };
        if live {
            self.alive += 1;
        } else {
            self.alive = self.alive.saturating_sub(1);
        }
        self.forget_cycles();
    }

    /// Центрирует старое содержимое в сетке `width x` `height`, отбрасывая то,
    /// что не помещается. Используется один раз при старте, чтобы подогнать
    /// пустой мир под окно.
    pub fn resize(&mut self, width: usize, height: usize) {
        let (width, height) = (width.max(1), height.max(1));
        if width == self.grid.width && height == self.grid.height {
            return;
        }
        let old = std::mem::replace(&mut self.grid, Grid::new(width, height));
        let (dx, dy) = (
            old.width as isize / 2 - width as isize / 2,
            old.height as isize / 2 - height as isize / 2,
        );
        for y in 0..height {
            for x in 0..width {
                let (ox, oy) = (x as isize + dx, y as isize + dy);
                if ox < 0 || oy < 0 || ox >= old.width as isize || oy >= old.height as isize {
                    continue;
                }
                let src = oy as usize * old.width + ox as usize;
                self.grid.cells[y * width + x] = old.cells[src].clone();
            }
        }
        self.alive = self.grid.cells.iter().filter(|c| c.is_alive()).count();
        self.forget_cycles();
    }
}
