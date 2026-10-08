# HTML-теги и CSS-стили — краткий конспект (лабораторная работа №2)

## HTML-теги — «скелет» страницы

Каждый тег описывает **что это** за элемент:

| Тег | Назначение |
|-----|-----------|
| `<h1>`, `<h2>`, `<h3>` | Заголовки разного уровня (h1 — главный) |
| `<p>` | Абзац текста |
| `<span>` | Кусок текста внутри строки (inline, не переносит строку) |
| `<div>` | Контейнер-блок, занимает всю ширину (block) |
| `<ul>` / `<ol>` + `<li>` | Список: маркированный / нумерованный |
| `<a href="...">` | Ссылка |
| `<img src="..." alt="...">` | Картинка (`alt` — текст, если картинка не загрузилась) |
| `<header> <main> <section> <nav> <footer>` | Семантика: говорят браузеру и поисковику, какая это часть страницы |
| `<form> <input> <fieldset> <label> <button>` | Форма ввода |

Важные правила:
- Теги вкладываются **строго в обратном порядке**: `<p><b>текст</b></p>`, а не `<p><b>текст</p></b>`
- Атрибуты пишутся внутри открывающего тега: `<img src="photo.jpg" alt="Описание">`
- `<!DOCTYPE html>` и `<meta charset="UTF-8">` — обязательная «шапка» каждого документа

Пример структуры (см. `task_itog.html`):

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GameZone — вариант 04</title>
    <link rel="stylesheet" href="task_itog.css">
  </head>
  <body>
    <header>...</header>
    <main>...</main>
    <footer>...</footer>
  </body>
</html>
```

---

## CSS-стили — «одежда» страницы

CSS отвечает на вопрос **как выглядит** элемент. Подключается в `<head>` через
`<link rel="stylesheet" href="файл.css">`.

### Структура правила

```css
.nav a:hover {
    color: #00e5ff;
    background-color: #0d1020;
}
/*  ↑ селектор (кому)   ↑ свойства: значение */
```

### Селекторы

| Селектор | Что выбирает | Пример из lab2 |
|----------|-------------|----------------|
| `p` | Все теги `<p>` | `p { font-size: 17px; }` |
| `.имя` | Элементы с `class="имя"` | `.feature-card { ... }` |
| `#имя` | Элемент с `id="имя"` | — |
| `tag.class` | Тег с указанным классом | `form input[type="text"]` |
| `:hover` | Псевдокласс — при наведении | `.nav a:hover` |
| `::before` / `::after` | Псевдоэлемент — добавляет содержимое | `.hero p::before { content: "▶ "; }` |

### Основные свойства

- **Типографика**: `font-size`, `font-family`, `line-height`, `letter-spacing`, `text-transform`, `color`
- **Цвет и фон**: `background-color`, `background: linear-gradient(...)`
- **Тени и скругления**: `border-radius`, `box-shadow`, `text-shadow`

### Box model (модель коробки)

Каждый элемент — «коробка» из четырёх слоёв:

```
┌────────── margin (внешний отступ) ──────────┐
│  ┌──────── border (рамка) ────────┐         │
│  │  ┌──── padding (внутренний) ─┐ │         │
│  │  │      content (контент)    │ │         │
│  │  └───────────────────────────┘ │         │
│  └────────────────────────────────┘         │
└─────────────────────────────────────────────┘
```

`* { box-sizing: border-box; }` — ширина элемента включает padding и border
(иначе padding «раздувает» элемент). Всегда ставится в начале CSS.

### Вёрстка (раскладка)

- **Flexbox** — линейная раскладка (строка или колонка):
  ```css
  .nav { display: flex; gap: 24px; }
  header { display: flex; justify-content: space-between; align-items: center; }
  ```
- **Grid** — сетка из строк и столбцов:
  ```css
  .features { display: grid; grid-template-columns: repeat(3, 1fr); gap: 24px; }
  ```
- **position**:
  - `fixed` — прилипает к экрану (шапка, task_itog.css:53)
  - `relative` — база для `absolute` внутри него
  - `absolute` — позиционируется от ближайшего родителя (`badge`, task_itog.css:175)

### Модульная шкала размеров (задание 4)

Размер считается по формуле:

```
размер = базовый × коэффициент ^ уровень
```

База 17px, коэффициент 1.333 (см. `task4.css`):

| Уровень | Элемент | Расчёт | Результат |
|---------|---------|--------|-----------|
| 0 | `p` | 17 | 17px |
| 1 | `h4` | 17 × 1.333 | 24px |
| 2 | `h3` | 23 × 1.333 | 28px |
| 3 | `h2` | 27 × 1.333 | 36px |
| 4 | `h1` | 32 × 1.333 | 38px |
| −1 | `.caption` | 17 ÷ 1.333 | 14px |

### Адаптивность (media query)

Стили переопределяются под ширину экрана:

```css
@media (max-width: 560px) {
    header { flex-direction: column; }
    .features { grid-template-columns: 1fr; }
}
```

---

## Как HTML и CSS связаны

1. В `<head>` HTML-страницы подключается CSS: `<link rel="stylesheet" href="task_itog.css">`
2. В HTML элементам задаются `class` и `id`
3. CSS находит их селекторами и применяет свойства

```html
<div class="feature-card">...</div>
```
```css
.feature-card { background-color: #171b31; border-radius: 16px; }
```

**Кратко**: HTML отвечает за **содержание и структуру**, CSS — за **внешний вид и раскладку**.
