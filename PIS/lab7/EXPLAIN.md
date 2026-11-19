# EXPLAIN — лабораторная работа №5 (React Router)

Документ состоит из двух частей:

1. Разбор итогового задания — файл `src/task_itog.jsx`, его компоненты, хуки, состояние и структура.
2. Разбор функциональных элементов React Router на основе `src/task1.jsx` — `BrowserRouter`, `Routes`, `Route`, `Link`, `NavLink`, `Outlet`, `useNavigate`, `useParams`, `useLocation`.

Тема проекта — игровой клуб **GameZone**. Стек: React 19, React Router DOM 7, Vite.

---

## 1. Итоговое задание `task_itog`

### 1.1. Назначение и архитектура

`src/task_itog.jsx` — итоговое приложение по **варианту Б**: собственная структура сайта, где тема лабораторной реализована не набором отдельных страниц, а комбинацией страниц и секций главной.

Точка входа:

```jsx
// src/main.jsx
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './task_itog.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>,
)
```

`BrowserRouter` обязателен: `task_itog` использует `useNavigate`, `useLocation`, `Link`, `Routes`.

#### Маршруты приложения

| Маршрут | Компонент | Что показывает |
|---|---|---|
| `/` | `App` → `<main>` с секциями | Главная: Hero, Stats, Features, Catalog, Favorites, Promo, Review, Faq, RequestForm |
| `/about` | `AboutPage` | Отдельная страница «О нас» с историей клуба |
| `*` | `NotFoundPage` | Страница 404 для любого несуществующего адреса |

Дополнительно внутри главной используются хэш-адреса:

| Адрес | Секция |
|---|---|
| `/#features` | Блок услуг `Features` |
| `/#catalog` | Каталог игр `Catalog` |
| `/#contacts` | Форма заявки `RequestForm` |
| `/#favorites` | Блок избранного `Favorites` |

Переход к секциям выполняет `App` через состояние `scrollRequest` и `scrollIntoView`, а не через отдельные маршруты. Поэтому каталог в этой реализации — секция главной, а не страница `/catalog`, как в варианте А из `task1.jsx`.

#### Схема состояния

Всё «общее» состояние поднято в `App` и передаётся дочерним компонентам через props. Обратная связь идёт через колбэки:

```text
App
├── state: activeItem, favorites, selectedFeatureIds, toasts, modal, partyMode ...
├── Header        ← activeItem, favoritesCount, onNavigate
├── Routes
│   ├── /         → Hero, Features, Catalog, Favorites, Promo, Review, Faq, RequestForm
│   └── /about    → AboutPage
├── Footer        ← onSecret
├── Toasts        ← toasts, onDismiss
├── Confetti      ← count
└── Modal         ← isOpen, onClose
```

Принцип: **данные вниз через props, события вверх через callbacks**.

---

### 1.2. Компоненты

| Компонент | Файл | Назначение | Ключевые props / состояние |
|---|---|---|---|
| `App` | `src/task_itog.jsx` | Корневой компонент: маршруты, общее состояние, обработчики | `useNavigate`, `useLocation`, `favorites`, `toasts`, `isModalOpen`, `partyMode` |
| `NotFoundPage` | `src/task_itog.jsx` | Страница 404 | — |
| `Header` | `components/Header.jsx` | Шапка: логотип, часы, меню, счётчик избранного, бургер | `title`, `navItems`, `activeItem`, `onNavigate`, `favoritesCount`, `onFavoritesClick` |
| `Hero` | `components/Hero.jsx` | Первый экран: печатающийся текст, счётчик онлайн, кнопки | `onShowAbout`, `onOpenModal`, `onExplore` |
| `Features` | `components/Features.jsx` | Сетка услуг | `features`, `selectedIds`, `favorites`, `onSelect`, `onToggleFavorite` |
| `FeatureCard` | `components/FeatureCard.jsx` | Карточка услуги с 3D-наклоном, выбором и избранным | `feature`, `index`, `isSelected`, `isFavorite`, `onSelect`, `onToggleFavorite` |
| `Catalog` | `components/Catalog.jsx` | Каталог игр, сортировка по рейтингу | `favorites`, `onToggleFavorite` |
| `GameCard` | `components/Catalog.jsx` | Карточка игры (внутренний компонент) | `game`, `index`, `isFavorite`, `onToggleFavorite` |
| `Favorites` | `components/Favorites.jsx` | Список избранного с анимацией удаления | `favorites`, `kinds`, `onRemove`, `onClear` |
| `Stats` | `components/Stats.jsx` | Блок «клуб в цифрах» | `stats` (по умолчанию `DEFAULT_STATS`) |
| `CountUp` | `components/Stats.jsx` | Анимированный счётчик (внутренний компонент) | `value`, `start`, `duration` |
| `Promo` | `components/Promo.jsx` | Акция: таймер, купон, копирование, конфетти | `couponGranted`, `onOpenModal`, `onNotify`, `onGoToForm` |
| `Review` | `components/Review.jsx` | Карусель отзывов с загрузкой с сервера | — (внутреннее состояние) |
| `Faq` | `components/Faq.jsx` | Аккордеон частых вопросов | — (`openId`) |
| `RequestForm` | `components/RequestForm.jsx` | Форма заявки с валидацией, маской телефона, паролем | `onRequestSuccess`, `onNotify` |
| `Field` | `components/RequestForm.jsx` | Обёртка поля: подпись и ошибка (внутренний компонент) | `name`, `label`, `error`, `children` |
| `Footer` | `components/Footer.jsx` | Подвал и секретная кнопка Konami | `onSecret` |
| `Modal` | `components/Modal.jsx` | Модальное окно с анимацией закрытия и Escape | `isOpen`, `title`, `onClose`, `children` |
| `AboutPage` | `components/AboutPage.jsx` | Страница «О нас»: факты, статистика, таймлайн | `onBack`, `onOpenModal` |
| `Background` | `components/Background.jsx` | Анимированный фон и свечение за курсором | — (`useRef`) |
| `ScrollUI` | `components/ScrollUI.jsx` | Полоса прогресса прокрутки и кнопка «Наверх» | — (`progress`) |
| `Toasts` | `components/Toasts.jsx` | Всплывающие уведомления | `toasts`, `onDismiss` |
| `Confetti` | `components/Confetti.jsx` | Разовый «взрыв» конфетти | `count` |
| `Reveal` | `components/Reveal.jsx` | Обёртка для анимации появления при прокрутке | `as`, `className`, `delay`, `variant`, `style`, `children` |
| `Icon` | `components/Icon.jsx` | Набор SVG-иконок без внешней библиотеки | `name`, `size`, `className`, `strokeWidth` |

---

### 1.3. Встроенные хуки React

| Хук | Где используется | Что делает |
|---|---|---|
| `useState` | `App`, `Header`, `Hero`, `Favorites`, `Promo`, `Review`, `Faq`, `RequestForm`, `Modal`, `ScrollUI` | Хранит локальное состояние: формы, таймеры, открытые блоки, загрузку |
| `useEffect` | `App`, `Header`, `Hero`, `Favorites`, `Promo`, `Review`, `RequestForm`, `Modal`, `ScrollUI`, `Background` | Побочные эффекты: таймеры, подписки, `fetch`, `IntersectionObserver`, `keydown`, `scroll`, синхронизация с URL |
| `useRef` | `App`, `Header`, `FeatureCard`, `Favorites`, `Promo`, `Modal`, `Background` | Ссылки на DOM-узлы и изменяемые значения без ререндера (id тостов, флаги, таймеры) |
| `useCallback` | `App` | Стабильные функции `notify`, `dismissToast`, `triggerSecret`, `openModal`, `closeModal` |
| `useMemo` | `App`, `Confetti` | Кэширование вычислений: `favoriteKinds`, массив частиц конфетти |
| `useLayoutEffect` | `Header` | Измерение позиции активного пункта меню до отрисовки (ползунок подсветки) |

Примеры:

```jsx
// useState + useEffect: счётчик онлайн в Hero
const [online, setOnline] = useState(start)

useEffect(() => {
  const intervalId = setInterval(() => {
    setOnline((prev) => Math.min(Math.max(prev + delta, 96), 214))
  }, 2200)
  return () => clearInterval(intervalId)
}, [])
```

```jsx
// useCallback: стабильный notify, чтобы не пересоздавать подписки
const notify = useCallback((message, type = "info") => { ... }, [dismissToast])
```

```jsx
// useRef: id тоста не вызывает ререндер
const toastId = useRef(0)
function notify(message) {
  toastId.current += 1
  ...
}
```

---

### 1.4. Хуки React Router в `task_itog`

| Элемент | Где | Что делает |
|---|---|---|
| `Routes` / `Route` | `App` | Описывают маршруты `/`, `/about`, `*` |
| `Link` | `Header`, `NotFoundPage` | Клиентские переходы без перезагрузки |
| `useNavigate` | `App` | `navigate("/about")`, `navigate("/")`, `navigate("/#features")` |
| `useLocation` | `App` | Читает `pathname` и `hash`, синхронизирует меню и прокрутку |

```jsx
const navigate = useNavigate()
const location = useLocation()

function goToSection(target) {
  if (location.pathname !== "/") {
    navigate(`/#${target}`)
    return
  }
  setScrollRequest({ target })
}
```

---

### 1.5. Пользовательские хуки

| Хук | Файл | Возвращает | Назначение |
|---|---|---|---|
| `useLocalStorage(key, initialValue)` | `hooks/useLocalStorage.js` | `[value, setValue]` | Состояние, сохраняемое в `localStorage`; используется для избранного (`gamezone-favorites`) |
| `useReveal({ threshold, once })` | `hooks/useReveal.js` | `[ref, isVisible]` | Отслеживает появление элемента через `IntersectionObserver` |
| `useTypewriter(phrases, options)` | `components/Hero.jsx` | строку | Эффект печатающегося текста |
| `useOnlineCounter(start)` | `components/Hero.jsx` | число | Имитация «живого» счётчика игроков |

```jsx
// useLocalStorage: ленивая инициализация + запись при изменении
const [value, setValue] = useState(() => {
  try {
    const raw = localStorage.getItem(key)
    return raw ? JSON.parse(raw) : initialValue
  } catch {
    return initialValue
  }
})

useEffect(() => {
  try {
    localStorage.setItem(key, JSON.stringify(value))
  } catch {
    // приватный режим — просто работаем без сохранения
  }
}, [key, value])
```

---

### 1.6. Данные и константы

| Константа | Файл | Содержимое |
|---|---|---|
| `NAV_ITEMS` | `task_itog.jsx` | Пункты меню: Главная, О нас, Услуги, Игры, Контакты |
| `SCROLL_TOP`, `SPY_TARGETS` | `task_itog.jsx` | Верх страницы и секции для scroll spy |
| `FEATURES` | `task_itog.jsx` | Три услуги клуба |
| `KONAMI` | `task_itog.jsx` | Последовательность клавиш пасхалки |
| `GAMES`, `GENRES` | `components/Catalog.jsx` | 11 игр и жанры |
| `QUESTIONS` | `components/Faq.jsx` | Вопросы и ответы FAQ |
| `ABOUT_FACTS`, `TIMELINE` | `components/AboutPage.jsx` | Факты и история клуба |
| `DEFAULT_STATS` | `components/Stats.jsx` | Цифры для блока статистики |
| `PATHS` | `components/Icon.jsx` | SVG-пути иконок |

---

### 1.7. Соответствие чек-листу итогового задания

`task1.jsx` реализует **вариант А** (все пункты буквально), `task_itog.jsx` — **вариант Б** (своя структура).

| № | Требование чек-листа | `task1.jsx` (вариант А) | `task_itog.jsx` (вариант Б) |
|---|---|---|---|
| 1 | Главная без переделки макета | `HomePage`, `/` | Главная с секциями, `/` |
| 2 | Каталог со списком карточек | `CatalogPage`, `/catalog` | Секция `Catalog`, `/#catalog` |
| 3 | Страница товара `/product/:id` + `useParams` | `ProductPage` | **Нет** отдельной страницы товара |
| 4 | Вкладки «Описание» / «Отзывы» через `Outlet` | `ProductDetails`, `ProductReviews`, `/product/:id` | **Нет** |
| 5 | Страница «О сайте» с формой и `navigate("/")` | `AboutPage` | `AboutPage` есть, но **без формы** |
| 6 | Меню через `NavLink` с подсветкой | `Header` на `NavLink` | `Header` на `Link` + ручная подсветка |
| 7 | Кнопка «Назад» через `navigate(-1)` | `ProductPage` | **Нет** (`handleBackHome` использует `navigate("/")`) |
| 8 | Страница 404 | `path="*"` | `path="*"` |
| 9 | Модальное окно через state | — | `Modal` |
| 10 | Стили из ЛР4 | `task1.css` | `task_itog.css` |

**Вывод для защиты:** в проекте есть оба варианта. `task1.jsx` закрывает весь чек-лист по пунктам 2–8, а `task_itog.jsx` показывает расширенную версию сайта по варианту Б. Если требуется сдавать строго один вариант, нужно либо запускать `task1.jsx`, либо добавить в `task_itog` недостающие пункты 3, 4, 5 (форма), 6 (`NavLink`), 7 (`navigate(-1)`).

---

## 2. Функциональные элементы React Router (разбор `task1.jsx`)

`task1.jsx` — учебный файл со всеми TODO по темам 1–7. Ниже — таблицы по каждому элементу.

### 2.1. Компоненты роутера

| Элемент | Где в `task1.jsx` | Назначение | Пример |
|---|---|---|---|
| `BrowserRouter` | TODO 1, `main.jsx` | Оборачивает приложение и даёт роутеру контекст. Без него хуки и `<Link>` не работают | `<BrowserRouter><App /></BrowserRouter>` |
| `Routes` | TODO 2–4 | Контейнер, который выбирает **один** подходящий `Route` по текущему URL | `<Routes>...</Routes>` |
| `Route` | TODO 2–4 | Сопоставляет `path` и `element` | `<Route path="/about" element={<AboutPage />} />` |
| `Route index` | TODO 26 | Дочерний маршрут по умолчанию для вложенного пути | `<Route index element={<ProductDetails />} />` |
| `Route path="*"` | TODO 31 | Ловушка для всех несуществующих адресов; ставится последней | `<Route path="*" element={<NotFoundPage />} />` |
| `Link` | TODO 6–9 | Клиентская ссылка без перезагрузки документа | `<Link to="/about">О сайте</Link>` |
| `NavLink` | TODO 11–14 | Ссылка, которая знает, активна ли она, и добавляет класс `active` | `<NavLink to="/" end>...</NavLink>` |
| `Outlet` | TODO 27 | Место в родительском компоненте, куда рендерится вложенный маршрут | `<Outlet />` |

#### Ошибка из TODO 4

```jsx
// Неправильно:
function App() {
  return (
    <Route path="/" element={<HomePage />} />
    <Route path="/about" element={<AboutPage />} />
  )
}
```

Две проблемы:

1. В JSX должен быть **один корневой элемент** — два соседних `<Route>` недопустимы.
2. `<Route>` не работает без общего `<Routes>`: роутер не знает, какой маршрут выбрать.

```jsx
// Правильно:
function App() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/about" element={<AboutPage />} />
    </Routes>
  )
}
```

---

### 2.2. Хуки роутера

| Хук | Где | Возвращает / делает | Пример |
|---|---|---|---|
| `useNavigate` | TODO 21–24 | Функцию программного перехода | `const navigate = useNavigate()` |
| `useParams` | TODO 17, 19, 20 | Объект параметров маршрута; значения всегда строки | `const { id } = useParams()` |
| `useLocation` | в `task_itog` | Объект текущего адреса: `pathname`, `search`, `hash`, `state` | `const { pathname, hash } = useLocation()` |

#### `useParams`: почему всегда строка

Параметр `:id` — это часть URL, то есть текст. Роутер не знает, что в проекте `id` означает число.

```jsx
const { id } = useParams()          // id === "1", а не 1
const product = PRODUCTS.find(
  (item) => item.id === String(id)  // приведение типа обязательно
)
```

Если забыть приведение:

```jsx
PRODUCTS.find((item) => item.id === id) // "1" === 1 → false
```

Результат — страница товара с существующим id покажет «Товар не найден».

---

### 2.3. `useNavigate` — переход по коду

```jsx
import { useNavigate } from 'react-router-dom'

function ProductPage() {
  const navigate = useNavigate()

  return (
    <button type="button" onClick={() => navigate(-1)}>
      Назад к каталогу
    </button>
  )
}
```

Хук вызывается **один раз в начале компонента**, как и другие хуки. Возвращённая функция `navigate` используется в обработчиках событий и эффектах.

#### Формы вызова

| Вызов | Поведение | Уместный пример |
|---|---|---|
| `navigate("/")` | Добавляет новую запись в историю и переходит на `/` | После отправки формы (TODO 22) |
| `navigate("/about")` | Переход на конкретный маршрут | Кнопка «О нас» |
| `navigate("/product/3")` | Переход на динамический маршрут | Переход к товару из кода |
| `navigate(-1)` | Возврат на предыдущую запись истории, как кнопка «назад» браузера | Кнопка «Назад к каталогу» (TODO 21) |
| `navigate(1)` | Переход вперёд по истории | Кнопка «вперёд» |
| `navigate("/", { replace: true })` | Заменяет текущую запись вместо добавления новой | Редирект, чтобы не засорять историю |
| `navigate("/about", { state: { from: "home" } })` | Передаёт данные без изменения URL | Вернуться туда, откуда пришли |

#### Правила использования

| Правило | Почему |
|---|---|
| Не вызывать `navigate` в теле компонента | Вызов происходит на каждом рендере → бесконечный цикл переходов (TODO 23) |
| Вызывать в обработчике события | Переход — реакция на действие пользователя |
| Или в `useEffect` | Если переход нужен как побочный эффект после рендера |
| Указывать `navigate` в зависимостях `useEffect` | Функция должна быть актуальной |
| Не забывать `event.preventDefault()` в форме | Иначе браузер выполнит обычную отправку и перезагрузит страницу |

#### Ошибка из TODO 23

```jsx
// Неправильно: navigate вызывается при каждом рендере
function BrokenRedirect() {
  const navigate = useNavigate()
  navigate("/")
  return <p>Секунду...</p>
}
```

```jsx
// Правильно: переход — побочный эффект
import { useEffect } from 'react'

function BrokenRedirect() {
  const navigate = useNavigate()

  useEffect(() => {
    navigate("/")
  }, [navigate])

  return <p>Секунду...</p>
}
```

#### `navigate("/путь")` против `navigate(-1)`

| Критерий | `navigate("/")` | `navigate(-1)` |
|---|---|---|
| Что делает | Переходит на конкретный адрес | Возвращает на предыдущую запись истории |
| История | Добавляет новую запись | Двигается по существующей истории |
| Аналог в браузере | Ввод адреса / клик по ссылке | Кнопка «назад» |
| Когда использовать | После отправки формы, при редиректе | Кнопка «Назад», возврат туда, откуда пришёл пользователь |
| Пример в ЛР | `AboutPage` → `navigate("/")` | `ProductPage` → `navigate(-1)` |

#### `useNavigate` против `Link`

| Критерий | `Link` | `useNavigate` |
|---|---|---|
| Где используется | В JSX | В коде компонента |
| Стиль | Декларативный | Императивный |
| Когда | Обычная навигация в разметке | Навигация после события, таймера, запроса, проверки условия |
| Пример | `<Link to="/about">О сайте</Link>` | `navigate("/")` после `onSubmit` |

---

### 2.4. Сравнения и ответы на вопросы «уровня собеседования»

| Вопрос | Краткий ответ |
|---|---|
| Чем `BrowserRouter` отличается от переключения экранов через `useState` (ЛР4)? | `BrowserRouter` меняет реальный URL через History API, сохраняет историю, работают кнопки «назад/вперёд», прямые ссылки и закладки. `useState` меняет только отрисованный компонент, URL остаётся прежним |
| `Link` против `<a href>` | `<a href>` делает полный запрос к серверу и перезагружает документ: React-приложение размонтируется, весь state теряется. `<Link to>` перехватывает клик, меняет URL через History API и рендерит маршрут без перезагрузки |
| Почему `NavLink` без CSS для `.active` «не работает»? | Класс `active` добавляется, но без CSS-правила он ни на что не влияет визуально. Нужно правило `.menu a.active { ... }` |
| Что если забыть `<Outlet />`? | Вложенные маршруты считаются активными и URL меняется, но их элементы не отрисовываются: родителю некуда вставить дочерний маршрут |
| Почему `useParams` возвращает строку? | Параметр — часть URL, то есть текст. Роутер не знает тип данных проекта. Без приведения `"1" === 1` даёт `false` |
| Что видно до добавления `path="*"`? | Ни один маршрут не совпадает, `<Routes>` ничего не рендерит — пустая область (возможно, только `Header`, вынесенный наружу) |
| Чем `end` полезен у `NavLink`? | Без `end` ссылка на `/` считается активной на всех вложенных адресах; `end` включает совпадение только с точным `/` |
| Почему путь вложенного маршрута пишут без `/`? | `path="reviews"` — относительный путь от родителя `/product/:id`; `path="/reviews"` был бы абсолютным и не вложился бы |
| Что делает функция в `className` у `NavLink`? | `className={({ isActive }) => ...}` позволяет собрать классы в зависимости от активности; аналогично children-функция управляет текстом |
| Почему `navigate` нельзя вызывать в рендере? | Это побочный эффект: вызов на каждом рендере приводит к циклу переходов и ошибке React |

---

### 2.5. Все TODO из `task1.jsx`

| TODO | Тема | Реализация |
|---|---|---|
| 1 | `BrowserRouter` | Обёртка в `main.jsx` |
| 2 | Первый маршрут | `HomePage`, `path="/"` |
| 3 | Второй маршрут | `AboutPage`, `path="/about"` |
| 4 | Найдите ошибку | Два `<Route>` без `<Routes>` и без общего корня |
| 5 | `BrowserRouter` vs `useState` | Ответ в разделе 2.4 |
| 6 | Ссылка вперёд | `<Link to="/about">О сайте</Link>` |
| 7 | Ссылка назад | `<Link to="/">На главную</Link>` |
| 8 | `<a href>` vs `Link` | Полная перезагрузка vs клиентский переход |
| 9 | Меню сайта | `Header` снаружи `<Routes>` |
| 10 | `Link` vs `<a>` | Ответ в разделе 2.4 |
| 11 | `Link` → `NavLink` | Все три ссылки в `Header` |
| 12 | Стиль `.active` | Правило в `task1.css` (`App.css` в комментарии) |
| 13 | Проверка в браузере | Подсветка переезжает на активный пункт |
| 14 | Функция в `className` | Класс + стрелка «→» у «Цены» |
| 15 | `NavLink` без CSS | Класс есть, визуального эффекта нет |
| 16 | Динамический маршрут | `path="/product/:id"` |
| 17 | Чтение параметра | `const { id } = useParams()` |
| 18 | Список товаров | `PRODUCTS.map(...)` + `key` |
| 19 | Реальный товар | `find` по id, иначе «Товар не найден» |
| 20 | Параметр — строка | `String(id)` / `Number(id)` |
| 21 | Кнопка «Назад» | `navigate(-1)` |
| 22 | Переход после формы | Controlled-форма + `navigate("/")` |
| 23 | `navigate` в рендере | Перенос в `useEffect` |
| 24 | `navigate("/path")` vs `navigate(-1)` | Ответ в разделе 2.3 |
| 25 | Компоненты вкладок | `ProductDetails`, `ProductReviews` |
| 26 | Вложенные маршруты | `index` и `path="reviews"` |
| 27 | `Outlet` и вкладки | `<Outlet />` + две `<Link>` |
| 28 | Отзывы из данных | `reviews.map(...)` с `key` |
| 29 | Забытый `Outlet` | Вложенный маршрут не отрисуется |
| 30 | Страница 404 | `NotFoundPage` |
| 31 | Маршрут-ловушка | `path="*"` |
| 32 | Проверка 404 | До `path="*"` — пустая область |

---

### 2.6. Итоговый маршрут приложения из `task1.jsx`

```jsx
function App() {
  return (
    <>
      <Header />

      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/catalog" element={<CatalogPage />} />

          <Route path="/product/:id" element={<ProductPage />}>
            <Route index element={<ProductDetails />} />
            <Route path="reviews" element={<ProductReviews />} />
          </Route>

          <Route path="*" element={<NotFoundPage />} />
        </Routes>
      </main>
    </>
  )
}
```

Ключевые решения:

- `Header` вынесен **наружу** `<Routes>` — меню видно на любой странице.
- `ProductPage` содержит `<Outlet />` — вкладки «Описание» и «Отзывы» рендерятся внутри страницы товара.
- `index` — вкладка по умолчанию, `path="reviews"` — относительный вложенный путь.
- `path="*"` стоит последним и ловит все неизвестные адреса.

---

## 3. Исправленная ошибка запуска

До исправления `main.jsx` импортировал `BrowserRouter` из `react`:

```jsx
// Было — неправильно:
import { StrictMode, BrowserRouter } from 'react'
```

`react` не экспортирует `BrowserRouter`, поэтому в JSX оказывался `undefined`. Отсюда ошибка:

```text
Element type is invalid: expected a string ... but got: undefined.
```

Исправление:

```jsx
// Стало — правильно:
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import App from './task_itog.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>,
)
```

Проверка: `npm run build` завершается успешно.

---

## 4. Файлы проекта

| Файл | Роль |
|---|---|
| `src/main.jsx` | Точка входа, `BrowserRouter` |
| `src/task_itog.jsx` | Итоговое приложение (вариант Б) |
| `src/task1.jsx` | Учебные TODO по всем темам React Router (вариант А) |
| `src/task_itog.css` | Стили итогового приложения |
| `src/task1.css` | Стили учебного файла, правило `.active` |
| `src/index.css` | Базовые стили |
| `src/components/*.jsx` | Компоненты итогового приложения |
| `src/hooks/*.js` | Пользовательские хуки |
