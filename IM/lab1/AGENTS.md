# Спецификация приложения «Планетарные колонии» (клеточный автомат)

Документ предназначен для передачи в opencode / LLM-агента как ТЗ на генерацию проекта. Формулировки — императивные, требования — проверяемые.

---

## 1. Общее описание

**Название:** Planetary Colonies CA
**Тип:** десктопное приложение — симулятор клеточного автомата с GUI.
**Язык:** Rust (edition 2021).
**GUI:** `eframe` + `egui`.
**Назначение:** симуляция колонии на прямоугольной сетке с ресурсами, конструкциями, стрессом и антиклетками. Все правила — локальные и детерминированные, синхронное обновление, без «интеллекта» клеток.

**Ключевые требования:**
- Детерминизм при одинаковом seed и одинаковых параметрах.
- Полное отделение ядра симуляции от UI (ядро не зависит от egui).
- Синхронное обновление поколения: все клетки читают старое состояние, пишут новое.
- Воспроизводимость: seed ГСЧ, сохранение/загрузка состояния.

---

## 2. Технологический стек

```toml
[dependencies]
eframe = "0.29"
egui = "0.29"
rand = "0.8"
rand_chacha = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"

[dev-dependencies]
proptest = "1"
```

Структура проекта:
```
planetary-colonies/
├── Cargo.toml
├── README.md
├── assets/
│   └── patterns.json
└── src/
    ├── main.rs
    ├── app.rs
    ├── config.rs
    ├── sim/
    │   ├── mod.rs
    │   ├── grid.rs
    │   ├── cell.rs
    │   ├── citizen.rs
    │   ├── pattern.rs
    │   ├── construction.rs
    │   ├── resources.rs
    │   ├── movement.rs
    │   ├── anticell.rs
    │   └── rules.rs
    └── ui/
        ├── mod.rs
        ├── canvas.rs
        ├── panels.rs
        ├── tools.rs
        ├── overlays.rs
        └── palette.rs
```

---

## 3. Модель данных

### 3.1. Состояния клетки

```rust
pub enum CellState {
    Empty,
    Citizen,
    Construction,
    Overlap,
    AntiCell,
}

pub enum ConstructionKind {
    Water,
    Food,
    Energy,
    Population,
    Conflict,
}
```

### 3.2. Данные гражданина

```rust
pub struct CitizenData {
    pub stress: f32,
    pub fatigue: f32,
    pub loyalty: f32,
}
```

### 3.3. Клетка

```rust
pub struct Cell {
    pub state: CellState,
    pub kind: Option<ConstructionKind>,
    pub construction_id: Option<u32>,
    pub citizen: Option<CitizenData>,
    pub frozen: Option<CitizenData>,
}

impl Cell {
    pub fn empty() -> Self;
    pub fn citizen(data: CitizenData) -> Self;
    pub fn construction(kind: ConstructionKind, id: u32, frozen: CitizenData) -> Self;
    pub fn overlap(id: u32, frozen: CitizenData) -> Self;
    pub fn anticell() -> Self;
    pub fn is_regular(&self) -> bool; // Citizen | Construction | Overlap
    pub fn is_occupied(&self) -> bool; // не Empty
}
```

### 3.4. Сетка

```rust
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self;
    pub fn idx(&self, x: usize, y: usize) -> usize;
    pub fn xy(&self, idx: usize) -> (usize, usize);
    pub fn get(&self, x: usize, y: usize) -> &Cell;
    pub fn set(&mut self, x: usize, y: usize, cell: Cell);
    pub fn neighbors4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)>;
    pub fn neighbors8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)>;
    pub fn cells_in_radius(&self, x: usize, y: usize, r: usize) -> impl Iterator<Item = (usize, usize)>;
    pub fn swap(&mut self, a: usize, b: usize);
}
```

Соседи по граням **не заворачиваются** через край (тор не используется).

---

## 4. Конфигурация

```rust
pub struct SimConfig {
    // Население
    pub population_max: u32,          // M
    pub food_per_citizen: f32,        // K

    // Ресурсы
    pub water_capacity: f32,
    pub food_capacity: f32,
    pub energy_capacity: f32,
    pub water_need_per_citizen: f32,
    pub energy_need_per_citizen: f32,

    // Стресс / усталость / лояльность
    pub c_max: f32,
    pub f_max: f32,
    pub l_max: f32,
    pub stress_hunger: f32,
    pub stress_thirst: f32,
    pub stress_dark: f32,
    pub stress_over: f32,
    pub stress_conflict_per_tick: f32,
    pub conflict_radius: usize,
    pub fatigue_per_tick: f32,
    pub fatigue_rest: f32,
    pub rest_radius: usize,
    pub loyalty_surplus: f32,
    pub loyalty_deficit: f32,
    pub loyalty_stress: f32,

    // Движение граждан (вариант 2 — градиент плотности)
    pub citizen_move_enabled: bool,
    pub citizen_move_prob: f32,       // p_move

    // Антиклетки (вариант 5 — волна)
    pub anticell_enabled: bool,

    // Общие
    pub seed: u64,
}

pub struct RenderConfig {
    pub cell_px: usize,               // 1..32
    pub border_px: usize,             // 0..3
    pub show_grid_lines: bool,
    pub overlay_stress: bool,
    pub overlay_fatigue: bool,
    pub overlay_loyalty: bool,
    pub show_construction_outlines: bool,
    pub show_conflict_radius: bool,
}
```

Значения по умолчанию:
```
population_max = 100
food_per_citizen = 0.1
water_capacity = 500.0
food_capacity = 500.0
energy_capacity = 500.0
water_need_per_citizen = 0.05
energy_need_per_citizen = 0.05
c_max = 100.0
f_max = 100.0
l_max = 100.0
stress_hunger = 2.0
stress_thirst = 1.5
stress_dark = 1.5
stress_over = 3.0
stress_conflict_per_tick = 0.5
conflict_radius = 3
fatigue_per_tick = 0.2
fatigue_rest = 0.5
rest_radius = 2
loyalty_surplus = 0.3
loyalty_deficit = 1.0
loyalty_stress = 0.5
citizen_move_enabled = true
citizen_move_prob = 0.15
anticell_enabled = true
seed = 42
```

---

## 5. Паттерны конструкций

### 5.1. Формат

```rust
pub struct Pattern {
    pub name: String,
    pub kind: ConstructionKind,
    pub cells: Vec<(i8, i8)>,         // относительные координаты
    pub produce: Resources,            // +ресурсы за тик
    pub consume: Resources,            // -ресурсы за тик
    pub stress_radius: u8,             // только для Conflict
    pub stress_per_tick: f32,          // только для Conflict
}
```

### 5.2. Загрузка

Паттерны хранятся в `assets/patterns.json` в ASCII-виде и парсятся при старте.

Пример:
```json
[
  {
    "name": "water_well",
    "kind": "Water",
    "ascii": [".W.", "WWW", ".W."],
    "produce": { "water": 5.0 }
  },
  {
    "name": "farm",
    "kind": "Food",
    "ascii": ["FF", "FF"],
    "produce": { "food": 4.0 }
  },
  {
    "name": "solar",
    "kind": "Energy",
    "ascii": [".E.", "EEE", ".E."],
    "produce": { "energy": 5.0 }
  },
  {
    "name": "habitat",
    "kind": "Population",
    "ascii": [".P.", "PPP", ".P."],
    "produce": { "population": 1.0 }
  },
  {
    "name": "conflict_zone",
    "kind": "Conflict",
    "ascii": ["CCC", "CCC", "CCC"],
    "produce": {},
    "stress_radius": 3,
    "stress_per_tick": 0.5
  }
]
```

### 5.3. Вращения

Паттерн автоматически генерирует 4 варианта (0°, 90°, 180°, 270°). Отражений нет.

Функция:
```rust
pub fn rotations(&self) -> [Pattern; 4];
```

### 5.4. Детект

Для каждой клетки-якоря на поле пробуем наложить все паттерны и их вращения. Паттерн **применяется**, если все его клетки сейчас находятся в состоянии `Citizen` (или уже в `Construction` этого же паттерна).

Если клетка попадает в два паттерна — она помечается как `Overlap`.

---

## 6. Правила автомата

Все правила применяются **синхронно**: изменения пишутся в буфер `next`, затем применяются атомарно. Порядок применения — фиксированный.

### 6.1. Глобальные счётчики поколения

До применения правил вычислить:

- `N` — количество клеток в состоянии `Citizen`.
- `N_pop` — количество клеток `Construction` / `Overlap` с подтипом `Population` (для лимита M).
- `Food`, `Water`, `Energy` — суммарное производство построек с учётом множителей.
- `Food_need = N × K`.
- `Water_need = N × water_need_per_citizen`.
- `Energy_need = N × energy_need_per_citizen`.
- `hungry = Food < Food_need`.
- `thirsty = Water < Water_need`.
- `dark = Energy < Energy_need`.
- `over = N > population_max`.

### 6.2. Правило 1. Голод → AntiCell
```
if cell.state == Citizen && cell.citizen.loyalty <= 0.0 {
    next[i] = Cell::anticell();
}
```

### 6.3. Правило 2. Стресс → AntiCell
```
if cell.state == Citizen && cell.citizen.stress >= c_max {
    next[i] = Cell::anticell();
}
```

### 6.4. Правило 3. Обновление CitizenData
Только для `Citizen`, не попавших в правила 1–2.

```
Δstress = stress_hunger * hungry
        + stress_thirst * thirsty
        + stress_dark   * dark
        + stress_over   * over
        + conflict_sum(i)

Δfatigue = fatigue_per_tick
         - fatigue_rest * has_rest(i)

Δloyalty = loyalty_surplus * (все ресурсы в профиците)
         - loyalty_deficit * (хотя бы один в дефиците)
         - loyalty_stress  * (stress > c_max / 2)

stress  = clamp(stress  + Δstress,  0, c_max)
fatigue = clamp(fatigue + Δfatigue, 0, f_max)
loyalty = clamp(loyalty + Δloyalty, 0, l_max)
```

`conflict_sum(i)` — сумма вкладов всех `Conflict`-конструкций в радиусе `conflict_radius`.
`has_rest(i)` — есть ли `Population`-конструкция в радиусе `rest_radius`.

### 6.5. Правило 4. Вход в конструкцию
Применить детект паттернов (§5.4). Для каждой найденной фигуры:
- все её клетки становятся `Construction` с соответствующим `kind`;
- `citizen` переносится в `frozen`, `citizen` становится `None`.

### 6.6. Правило 5. Overlap
Если клетка попала в ≥2 паттерна одновременно:
- `state = Overlap`;
- весь связный комплекс конструкций получает `multiplier = 1.5`.

Комплекс — компонента связности графа, где вершины — конструкции, а рёбра — соседство по грани между их клетками.

### 6.7. Правило 6. Выход из конструкции
Если `state == Construction || state == Overlap` и паттерн, в который входила клетка, больше не существует (какая-то клетка паттерна стала `Empty` или `AntiCell`):
- `state = Citizen`;
- `citizen = frozen`, `frozen = None`;
- `kind = None`, `construction_id = None`.

### 6.8. Правило 7. Взрыв антиклетки
```
if cell.state == AntiCell
   && exists j in N4(i) with state(j) in {Citizen, Construction, Overlap}
{
    for j in N4(i) with is_regular(j): next[j] = Cell::empty();
    next[i] = Cell::empty();
}
```

### 6.9. Правило 8. Шаг антиклетки (вариант 5 — волна)
Реализуется **не как движение**, а как передача состояния. В формулировке чистого CA:

```
if cell.state == AntiCell
   && !правило_7_сработало
   && exists j in N4(i) with state(j) == Empty
   && следующий за j в том же направлении — не Empty
{
    next[j] = Cell::anticell();
    next[i] = Cell::empty();
}
```

Направление выбирается по фиксированному приоритету: **вверх → влево → вправо → вниз**. Первый пустой сосед в этом порядке — цель. Если сосед пуст, но за ним пусто, — шага нет (волна «рассасывается»).

### 6.10. Правило 9. Движение граждан (вариант 2 — градиент плотности)

Реализуется как **смена состояний**: старая клетка → `Empty`, новая → `Citizen` с теми же числами.

```
if config.citizen_move_enabled
   && cell.state == Citizen
   && rng.gen::<f32>() < citizen_move_prob
{
    candidates = пустые соседи по граням
    if candidates.is_empty() { не двигаемся }
    else {
        target = candidates.min_by_key(|j| density(j))
        // density(j) = число занятых клеток в N8(j)
        // при равенстве — по фиксированному приоритету направлений
        намерения.push((i, target))
    }
}
```

После сбора намерений:
1. Сортируем по индексу источника `i`.
2. Применяем последовательно: если `target` уже занят другим намерением — пропускаем.
3. Для применённого намерения: `next[i] = Empty`, `next[target] = Citizen(data)`, где `data` — числа исходной клетки.

Движение не может произойти в клетку, которая занята, или в клетку, которая в этом же поколении уже приняла другого гражданина.

### 6.11. Правило 10. Конец игры
Если после применения всех правил `N == 0` — симуляция останавливается. Состояния не меняются.

---

## 7. Порядок хода

```
fn tick(grid: &mut Grid, config: &SimConfig, rng: &mut ChaCha8Rng) {
    let ctx = compute_global_counters(grid, config);
    let mut next = grid.clone();

    apply_rule1_hunger(grid, &mut next, config);
    apply_rule2_stress(grid, &mut next, config);
    apply_rule3_citizen_update(grid, &mut next, config, &ctx);
    apply_rule4_construction_entry(grid, &mut next, config);
    apply_rule5_overlap(grid, &mut next, config);
    apply_rule6_construction_exit(grid, &mut next, config);
    apply_rule7_anticell_explosion(grid, &mut next);
    apply_rule8_anticell_wave(grid, &mut next, config);
    apply_rule9_citizen_move(grid, &mut next, config, rng);

    *grid = next;

    if ctx.n_citizen == 0 {
        stop_simulation();
    }
}
```

---

## 8. UI

### 8.1. Окно

- Заголовок: `Planetary Colonies CA`.
- Размер по умолчанию: 1280×800.
- Панели:
  - **Сверху** — тулбар инструментов.
  - **Слева** — canvas.
  - **Справа** — параметры и статистика.
  - **Снизу** — статус-бар.

### 8.2. Тулбар (инструменты)

Кнопки-переключатели (radio):
- `Кисть: Пусто`
- `Кисть: Гражданин`
- `Кисть: Антиклетка`
- `Штамп: Вода`
- `Штамп: Еда`
- `Штамп: Энергия`
- `Штамп: Население`
- `Штамп: Конфликт`
- `Генератор колонии` — размещает `N` граждан в случайных пустых клетках.

Плюс:
- `Start / Pause`
- `Step` — один тик вручную.
- `Reset` — сброс с текущим seed.
- `Сохранить` / `Загрузить`.

### 8.3. Правая панель

**Параметры:**
- Слайдер `population_max` (1..500).
- Слайдер `food_per_citizen` (0.0..1.0).
- Слайдер `c_max` (10..500).
- Слайдер `f_max` (10..500).
- Слайдер `l_max` (10..500).
- Слайдер `conflict_radius` (1..10).
- Слайдер `stress_conflict_per_tick` (0.0..5.0).
- Чекбокс `citizen_move_enabled`.
- Слайдер `citizen_move_prob` (0.0..1.0).
- Чекбокс `anticell_enabled`.
- Поле `seed` (u64).
- Слайдер `ticks_per_second` (1..60).

**Статистика:**
- Поколение.
- `N` (граждане).
- Вода / Еда / Энергия (текущее / ёмкость).
- Средний стресс / усталость / лояльность.
- Число конструкций по подтипам.
- Число антиклеток.
- Статус: `Running` / `Paused` / `Game Over`.

### 8.4. Canvas

- Отрисовка через одну текстуру `ColorImage` размером `W*cell_px × H*cell_px`.
- Перерисовка только изменённых клеток (dirty region) для производительности.
- Мышь:
  - ЛКМ — применить активный инструмент.
  - ПКМ — стереть (`Empty`).
  - Drag — рисовать линией (Bresenham).
  - Колесо — зум `cell_px` (1..32).

### 8.5. Палитра

| Состояние | Заливка | RGB |
|---|---|---|
| Empty | фон | `#1E1E1E` |
| Citizen | чёрный | `#000000` |
| Water | синий | `#1E6FFF` |
| Food | зелёный | `#2ECC40` |
| Energy | жёлтый | `#FFD400` |
| Population | светло-серый | `#D9D9D9` |
| Conflict | фиолетовый | `#8A2BE2` |
| Overlap | оранжевый | `#FF7A00` |
| AntiCell | красный | `#E01B24` |

**Грани всех клеток — чёрные** (`#000000`), толщина `border_px` (0..3). При `cell_px == 1` грани не рисуются.

### 8.6. Оверлеи

Тумблеры в правой панели:
- Стресс — тепловая карта от прозрачного к красному.
- Усталость — от прозрачного к синему.
- Лояльность — от прозрачного к зелёному.
- Контуры конструкций — обводка комплекса цветом доминирующего типа.
- Радиус конфликта — полупрозрачный фиолетовый круг.

---

## 9. Сериализация

Сохранять в JSON:
- `SimConfig` целиком.
- `Grid` (все клетки).
- `RenderConfig`.
- `seed` и текущее состояние ГСЧ.

Файл: `save.json` в корне проекта (или диалог выбора пути).

---

## 10. Тесты

### 10.1. Модульные
- `Grid::neighbors4` возвращает корректных соседей на краях.
- `Pattern::rotations` даёт 4 уникальных вращения.
- `Cell::is_regular` возвращает true для Citizen/Construction/Overlap.

### 10.2. Интеграционные
- **Детерминизм:** два запуска с одинаковым seed и одинаковой начальной сеткой дают одинаковый результат после 100 тиков.
- **Голод:** при отсутствии еды граждане через N тиков становятся антиклетками.
- **Стресс:** при наличии Conflict-конструкции рядом стресс растёт.
- **Взрыв антиклетки:** антиклетка рядом с гражданином уничтожает соседей и исчезает.
- **Вход в конструкцию:** правильно собранный паттерн из граждан становится конструкцией.
- **Overlap:** две пересекающиеся конструкции дают клетку `Overlap` и множитель ×1.5.
- **Game Over:** если `N == 0`, симуляция останавливается.

### 10.3. Property-тесты (proptest)
- Количество клеток сохраняется (никакая клетка не появляется из ниоткуда, кроме явных переходов).
- Все числа CitizenData в границах `[0, max]`.
- Применение правил не паникует на пустой сетке.

---

## 11. Производительность

- Сетка — плоский `Vec<Cell>`.
- `next` — второй `Vec<Cell>`, переиспользуется между тиками (`std::mem::swap`).
- `neighbors4` — без аллокаций, через `arrayvec` или фиксированный массив.
- Паттерн-детект — по локальным окнам, кэшировать хэш окна (опционально).
- Рендер — `ColorImage` + `TextureHandle`, обновление только dirty-региона.
- Целевая производительность: 60 FPS на сетке 256×256 при `cell_px = 4`.

---

## 12. Критерии готовности

- [ ] Проект собирается `cargo build --release`.
- [ ] `cargo test` проходит все тесты.
- [ ] При запуске открывается окно с пустой сеткой 128×128.
- [ ] Все инструменты работают.
- [ ] Все параметры из правой панели влияют на симуляцию.
- [ ] Start/Pause/Step/Reset работают.
- [ ] Сохранение/загрузка работают.
- [ ] Game Over корректно отображается.
- [ ] Правила 1–10 реализованы и покрыты тестами.
- [ ] Граждане двигаются по градиенту плотности (правило 9).
- [ ] Антиклетки распространяются как волна (правило 8).
- [ ] Overlap даёт множитель ×1.5.
- [ ] Палитра и грани соответствуют §8.5.
- [ ] README с описанием запуска и правил.

---

## 13. Что запрещено

- Использовать `unsafe` без обоснования.
- Смешивать логику симуляции и UI в одном модуле.
- Делать движение клеток через «поиск цели» (BFS, A*) — только локальные правила.
- Использовать `f64` — только `f32` для совместимости с egui.
- Хардкодить параметры в правилах — только через `SimConfig`.
- Использовать глобальное состояние (`static mut`, `lazy_static` для симуляции).

---

## 14. README

Сгенерировать `README.md` с:
- кратким описанием,
- скриншотом-заглушкой,
- инструкцией `cargo run --release`,
- списком правил автомата (§6),
- описанием палитры и инструментов,
- списком параметров конфига.

---

**Конец спецификации.** При реализации соблюдать порядок правил (§7) и синхронность обновления. Все спорные случаи решать в пользу детерминизма и локальности.
