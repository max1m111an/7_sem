//! Игра «Жизнь»: правило «Хищник и добыча» на замкнутом поле (торе).
//!
//! Два вида живых клеток ([`Species`]) с явными правилами рождения, гибели и
//! межвидового взаимодействия. Поколение считается синхронно: все клетки
//! читают старое состояние и пишут в буфер `next`.
//!
//! ```text
//! Пусто:      добычи в N8 == 3 и хищников в N4 == 0        → Добыча (B3)
//!             иначе хищников в N8 >= 2 и добычи в N4 >= 1   → Хищник
//!             иначе                                          → Пусто
//! Добыча:     хищников в N4 >= 1                            → Пусто  (съедена)
//!             иначе 2 <= добычи в N8 <= 3                    → Добыча (S23)
//!             иначе                                          → Пусто
//! Хищник:     добычи в N4 >= 1                               → Хищник (сыт)
//!             иначе хищников в N8 >= 2                        → Хищник (стая)
//!             иначе                                          → Пусто  (голод)
//! ```
//!
//! Колония из `rules.rs` и соседних модулей осталась в репозитории как
//! справочный код и в игру не вызывается.

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;

use rand::Rng;

use crate::sim::cell::{Cell, Species};
use crate::sim::grid::Grid;

/// Доля хищников среди живых клеток при случайном заполнении поля.
pub const PREDATOR_SHARE: f64 = 0.2;

/// Рождение добычи: ровно столько соседей-добычи среди N8.
const PREY_BORN: usize = 3;
/// Выживание добычи: число соседей-добычи среди N8 лежит в `MIN..=MAX`.
const PREY_MIN: usize = 2;
const PREY_MAX: usize = 3;
/// Рождение хищника и выживание стаи: столько хищников среди N8.
const PREDATOR_GROUP: usize = 2;

/// Мир «Жизни».
///
/// Поле замкнуто: соседи берутся через край (`Grid::neighbors4`/`neighbors8`),
/// поэтому клетка, уходящая за правый нижний угол, появляется в левом верхнем.
///
/// За миром ведётся история отпечатков (см. [`Life::cycle`]): конечный мир
/// обязан повториться; приложение ставит игру на паузу, но её можно продолжить.
#[derive(Clone, Debug)]
pub struct Life {
    pub grid: Grid,
    pub generation: u64,
    /// Число живых клеток обоих видов; всегда равно `prey + predator`.
    pub alive: usize,
    /// Число живых клеток вида «Добыча».
    pub prey: usize,
    /// Число живых клеток вида «Хищник».
    pub predator: usize,
    /// Отпечаток мира → поколение, в котором он встретился впервые. История
    /// начинается заново при любой правке мира.
    seen: HashMap<u64, u64>,
    /// `Some(длина цикла)` — мир повторился (приложение ставит паузу);
    /// снимается [`Life::forget_cycles`].
    pub cycle: Option<u64>,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        let mut life = Self {
            grid: Grid::new(width, height),
            generation: 0,
            alive: 0,
            prey: 0,
            predator: 0,
            seen: HashMap::new(),
            cycle: None,
        };
        life.remember();
        life
    }

    /// Отпечаток мира: два поколения с одинаковым отпечатком — один и тот же
    /// набор клеток с теми же видами, то есть дальше всё повторится.
    fn hash_state(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for cell in &self.grid.cells {
            hasher.write_u8(cell.is_alive() as u8);
            hasher.write_u8(species_tag(cell.species));
        }
        hasher.finish()
    }

    /// Запоминает текущий мир как точку отсчёта истории.
    fn remember(&mut self) {
        let hash = self.hash_state();
        self.seen.insert(hash, self.generation);
    }

    /// Начинает следить за циклами заново с текущего мира: история пуста,
    /// `cycle` снят. Вызывается при любой правке мира.
    pub fn forget_cycles(&mut self) {
        self.seen.clear();
        self.cycle = None;
        self.remember();
    }

    /// Одно поколение по правилу «Хищник и добыча»: см. описание модуля.
    /// Соседство по граням и диагоналям — N8, с заворотом через край;
    /// ближний контакт (N4) решает, кого съесть и чем сыт хищник; стая из
    /// [`PREDATOR_GROUP`] и более хищников в N8 держится вместе даже натощак.
    /// Поколение увеличивается всегда.
    ///
    /// Если новое состояние уже встречалось в истории, мир зациклился:
    /// `cycle` получает длину цикла в поколениях. Приложение ставит паузу.
    pub fn step(&mut self) {
        let old = &self.grid;
        let mut next = old.cells.clone();
        let mut prey = 0usize;
        let mut predator = 0usize;
        for y in 0..old.height {
            for x in 0..old.width {
                let (mut prey8, mut predator8) = (0usize, 0usize);
                let (mut prey4, mut predator4) = (0usize, 0usize);
                for (nx, ny) in old.neighbors8(x, y) {
                    match old.get(nx, ny).species {
                        Some(Species::Prey) => prey8 += 1,
                        Some(Species::Predator) => predator8 += 1,
                        None => {}
                    }
                }
                for (nx, ny) in old.neighbors4(x, y) {
                    match old.get(nx, ny).species {
                        Some(Species::Prey) => prey4 += 1,
                        Some(Species::Predator) => predator4 += 1,
                        None => {}
                    }
                }
                let born = next_species(old.get(x, y).species, prey8, predator8, prey4, predator4);
                next[old.idx(x, y)] = match born {
                    Some(species) => Cell::with_species(species),
                    None => Cell::empty(),
                };
                match born {
                    Some(Species::Prey) => prey += 1,
                    Some(Species::Predator) => predator += 1,
                    None => {}
                }
            }
        }
        self.grid.cells = next;
        self.generation += 1;
        self.alive = prey + predator;
        self.prey = prey;
        self.predator = predator;

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
        self.prey = 0;
        self.predator = 0;
        self.forget_cycles();
    }

    /// Заполняет поле случайным образом: каждая клетка живёт с вероятностью
    /// `density`, из живых примерно [`PREDATOR_SHARE`] — хищники. Поколение
    /// обнуляется — это новый мир.
    pub fn randomize(&mut self, density: f32) {
        let mut rng = rand::thread_rng();
        self.fill(density, &mut rng);
    }

    /// То же с явным генератором, чтобы тесты были воспроизводимыми.
    pub fn fill(&mut self, density: f32, rng: &mut impl Rng) {
        let density = density.clamp(0.0, 1.0);
        let (mut prey, mut predator) = (0usize, 0usize);
        for cell in &mut self.grid.cells {
            if !rng.gen_bool(f64::from(density)) {
                *cell = Cell::empty();
                continue;
            }
            let species = if rng.gen_bool(PREDATOR_SHARE) {
                Species::Predator
            } else {
                Species::Prey
            };
            *cell = Cell::with_species(species);
            match species {
                Species::Prey => prey += 1,
                Species::Predator => predator += 1,
            }
        }
        self.alive = prey + predator;
        self.prey = prey;
        self.predator = predator;
        self.generation = 0;
        self.forget_cycles();
    }

    /// Ставит клетку `(x, y)` в выбранный вид (или убивает, если `None`) и
    /// поддерживает счётчики видов в согласии с сеткой.
    pub fn set_cell(&mut self, x: usize, y: usize, species: Option<Species>) {
        let idx = self.grid.idx(x, y);
        let old_species = self.grid.cells[idx].species;
        if old_species == species {
            return;
        }
        self.grid.cells[idx] = match species {
            Some(species) => Cell::with_species(species),
            None => Cell::empty(),
        };
        self.bump(old_species, species);
        self.forget_cycles();
    }

    /// Ставит клетку `(x, y)` в живое/мёртвое состояние: живая клетка —
    /// «Добыча» (вид выбирается через [`Life::set_cell`]).
    pub fn set_alive(&mut self, x: usize, y: usize, live: bool) {
        self.set_cell(x, y, if live { Some(Species::Prey) } else { None });
    }

    /// Пересчитывает счётчики видов после замены одной клетки.
    fn bump(&mut self, from: Option<Species>, to: Option<Species>) {
        match from {
            Some(Species::Prey) => self.prey = self.prey.saturating_sub(1),
            Some(Species::Predator) => self.predator = self.predator.saturating_sub(1),
            None => {}
        }
        match to {
            Some(Species::Prey) => self.prey += 1,
            Some(Species::Predator) => self.predator += 1,
            None => {}
        }
        self.alive = self.prey + self.predator;
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
        self.recount();
        self.forget_cycles();
    }

    /// Пересчитывает все три счётчика по фактическому содержимому сетки.
    fn recount(&mut self) {
        let (mut prey, mut predator) = (0usize, 0usize);
        for cell in &self.grid.cells {
            match cell.species {
                Some(Species::Prey) => prey += 1,
                Some(Species::Predator) => predator += 1,
                None => {}
            }
        }
        self.prey = prey;
        self.predator = predator;
        self.alive = prey + predator;
    }
}

/// Следующее состояние клетки по правилу «Хищник и добыча».
///
/// `prey8`/`predator8` — соседи обоих видов среди N8, `prey4`/`predator4` —
/// среди ближнего кольца N4. Считается по старому поколению, поколение
/// синхронное.
fn next_species(
    cell: Option<Species>,
    prey8: usize,
    predator8: usize,
    prey4: usize,
    predator4: usize,
) -> Option<Species> {
    match cell {
        None if prey8 == PREY_BORN && predator4 == 0 => Some(Species::Prey),
        None if predator8 >= PREDATOR_GROUP && prey4 >= 1 => Some(Species::Predator),
        None => None,
        Some(Species::Prey) if predator4 >= 1 => None,
        Some(Species::Prey) if !(PREY_MIN..=PREY_MAX).contains(&prey8) => None,
        Some(Species::Prey) => Some(Species::Prey),
        Some(Species::Predator) if prey4 >= 1 || predator8 >= PREDATOR_GROUP => {
            Some(Species::Predator)
        }
        Some(Species::Predator) => None,
    }
}

/// Тег вида для отпечатка мира: мёртвая клетка — 0, добыча — 1, хищник — 2.
fn species_tag(species: Option<Species>) -> u8 {
    match species {
        None => 0,
        Some(Species::Prey) => 1,
        Some(Species::Predator) => 2,
    }
}
