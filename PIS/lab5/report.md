# Лабораторная работа №5

React Router. Вариант 04. Отчёт по проделанной работе.

В работе изучена маршрутизация React-приложения средствами `react-router-dom`: `BrowserRouter`, `Routes`, `Route`, `Link`, `NavLink`, `useParams`, `useNavigate`, `useLocation` и `Outlet`. Проект собран на React 19 и Vite; тема варианта – спортивный клуб «Атлет», итоговое приложение оформлено в стиле игрового клуба GameZone.

Итоговое задание выполнено в двух вариантах. Файл `src/task1.jsx` реализует вариант А – структуру и названия страниц строго по чек-листу. Файл `src/task_itog.jsx` реализует вариант Б – собственную структуру сайта GameZone, где часть разделов вынесена в отдельные маршруты, а часть работает как секции главной страницы. Основным запускаемым вариантом является Б: именно `task_itog.jsx` импортируется в `main.jsx`.

## Задание 1. Установка и базовая настройка (BrowserRouter, Routes, Route)

Изучена одноразовая настройка роутера и базовое сопоставление адресов. В `main.jsx` приложение обёрнуто в `BrowserRouter` из `react-router-dom`, поэтому все дочерние компоненты получают контекст роутера. В `task1.jsx` созданы компоненты `HomePage` с заголовком «Спортивный клуб «Атлет»» и `AboutPage` с заголовком «О сайте». Они подключены через `<Routes>` и `<Route>` с путями `/` и `/about`.

В TODO 4 разобран сломанный `App`. В нём два соседних `<Route>` нарушают правило JSX об одном корневом элементе, а отсутствие `<Routes>` не даёт роутеру выбрать подходящий маршрут. Исправленный `AppTask4` оборачивает оба `<Route>` в `<Routes>`.

На листинге 1 показано: подключение `BrowserRouter` в точке входа приложения.

**Листинг 1 - Задание №1 (main.jsx)**

```jsx
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import './index.css'
import App from './task_itog.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>,
)
```

На листинге 2 показано: импорты `react-router-dom`, данные каталога, компоненты `HomePage` и `AboutPage`, учебные варианты `AppTask2` и `AppTask3`, маршруты `/` и `/about`, а также исправленный вариант `AppTask4`.

**Листинг 2 - Задание №1 (task1.jsx)**

```jsx
import { useEffect, useState } from 'react'
import {
  Link,
  NavLink,
  Outlet,
  Route,
  Routes,
  useNavigate,
  useParams,
} from 'react-router-dom'
import './task1.css'

// Данные каталога (тема варианта — спортивный клуб «Атлет»).
const PRODUCTS = [
  { id: '1', title: 'Абонемент на месяц', price: 2500 },
  { id: '2', title: 'Разовое занятие', price: 500 },
  { id: '3', title: 'Персональная тренировка', price: 1500 },
]

// ...

// TODO 2: Первый маршрут
// В App.jsx создайте компонент HomePage, возвращающий
// <h1>Спортивный клуб «Атлет»</h1>, и подключите его через
// <Routes><Route path="/" element={<HomePage />} /></Routes>.

// Подсказка: не забудьте import { Routes, Route } from "react-router-dom".
function HomePage() {
  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>
      {/* TODO 6: ссылка вперёд на страницу «О сайте» */}
      <Link to="/about">О сайте</Link>
    </div>
  )
}

// Маршрут для главной страницы добавлен в итоговый <App /> ниже.
// Отдельный минимальный пример из TODO 2:
function AppTask2() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
    </Routes>
  )
}

// ...

// --- Средний уровень ---

// TODO 3: Второй маршрут
// Создайте компонент AboutPage с <h1>О сайте</h1> и подключите его на
// path="/about" вторым <Route> внутри того же <Routes>.

// Подсказка: несколько <Route> могут лежать внутри одного <Routes> —
// он сам выбирает, какой из них показать, по текущему адресу.
function AboutPage() {
  // TODO 22: controlled-форма + navigate("/") после отправки.
  const [email, setEmail] = useState('')
  const navigate = useNavigate()

  function handleSubmit(event) {
    event.preventDefault()
    navigate('/')
  }

  return (
    <div>
      <h1>О сайте</h1>
      {/* TODO 7: ссылка назад на главную */}
      <Link to="/">На главную</Link>

      <form onSubmit={handleSubmit}>
        <label>
          Ваш email
          <input
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </label>
        <button type="submit">Отправить</button>
      </form>
    </div>
  )
}

// Пример из TODO 3: второй <Route> внутри того же <Routes>.
function AppTask3() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/about" element={<AboutPage />} />
    </Routes>
  )
}

// ...

// --- Сложный уровень ---

// TODO 4: Найдите ошибку
// Ниже сломанный набросок App — объясните в отчёте по лабораторной
// работе, почему он не заработает (запустится с ошибкой), и
// перепишите его правильно в коде:
//
//   function App() {
//     return (
//       <Route path="/" element={<HomePage />} />
//       <Route path="/about" element={<AboutPage />} />
//     )
//   }
//
// Подсказка: сразу две проблемы — вспомните правило JSX «один корневой
// элемент» из ЛР4, и то, что <Route> не работает сам по себе, без
// общего <Routes> вокруг.

// Ошибки: (1) два соседних <Route> без общего корневого элемента —
// JSX требует ровно один корень; (2) <Route> должен находиться внутри
// <Routes>, иначе React Router не знает, какой маршрут показывать.
// Исправленный вариант:
function AppTask4() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/about" element={<AboutPage />} />
    </Routes>
  )
}
```

Рисунок 1 - Результат выполнения программы main.jsx, task1.jsx

На рис. 1 показан результат запуска приложения с двумя базовыми маршрутами. Переход через `BrowserRouter` меняет реальный URL в адресной строке через History API и сохраняет историю браузера, поэтому работают кнопки «назад/вперёд», прямые ссылки и закладки. Переключение экранов через `useState`, которое применялось в ЛР4, меняло только отрисованный компонент: URL оставался прежним, а история и закладки на конкретный экран не появлялись. Именно поэтому в ЛР5 экраны заменены на маршруты.

## Задание 2. Link – навигация без перезагрузки

Изучены клиентские ссылки `Link`. На `HomePage` добавлена ссылка `<Link to="/about">О сайте</Link>`, на `AboutPage` – `<Link to="/">На главную</Link>`. Компонент `Header` содержит три ссылки на `/`, `/catalog` и `/about` с текстами «Расписание», «Тренеры» и «Цены». Для пути `/catalog` создана заглушка `CatalogPage`. Компонент `<Header />` размещён в `App` снаружи `<Routes>`, поэтому меню видно на любой странице и не пропадает при переходе.

В TODO 8 показана ошибка: обычный `<a href="/about">` выполняет полноценный запрос к серверу, перезагружает документ и теряет состояние React. Исправление – заменить его на `<Link to="/about">`.

На листинге 3 показано: ссылки `Link` на главной и странице «О сайте», меню `Header` и каталог `CatalogPage` со списком товаров.

**Листинг 3 - Задание №2 (task1.jsx)**

```jsx
function HomePage() {
  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>
      {/* TODO 6: ссылка вперёд на страницу «О сайте» */}
      <Link to="/about">О сайте</Link>
    </div>
  )
}

// ...

function AboutPage() {
  // TODO 22: controlled-форма + navigate("/") после отправки.
  const [email, setEmail] = useState('')
  const navigate = useNavigate()

  function handleSubmit(event) {
    event.preventDefault()
    navigate('/')
  }

  return (
    <div>
      <h1>О сайте</h1>
      {/* TODO 7: ссылка назад на главную */}
      <Link to="/">На главную</Link>

      <form onSubmit={handleSubmit}>
        <label>
          Ваш email
          <input
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </label>
        <button type="submit">Отправить</button>
      </form>
    </div>
  )
}

// ...

// TODO 9: Меню сайта
// Соберите компонент Header с тремя ссылками Link на "/", "/catalog" и
// "/about", с текстами "Расписание", "Тренеры", "Цены" — для
// "/catalog" пока создайте компонент-заглушку CatalogPage с любым
// текстом, маршрут для него тоже нужно добавить в Routes. Разместите
// <Header /> в App — снаружи <Routes>, а не внутри одного из <Route>
// (меню должно быть видно на любой странице).

// Подсказка: если поставить <Header /> внутрь одного конкретного
// <Route>, оно будет пропадать при переходе на другие страницы —
// проверьте это на практике и учтите в решении.

// Header сразу сделан на NavLink (TODO 11) и с функцией className
// для ссылки «Цены» (TODO 14).
function Header() {
  return (
    <header className="site-header">
      <nav className="menu">
        <NavLink to="/" end>
          Расписание
        </NavLink>
        <NavLink to="/catalog">Тренеры</NavLink>

        {/* TODO 14: функция className + стрелка «→ » у активной ссылки */}
        <NavLink
          to="/about"
          className={({ isActive }) => (isActive ? 'active about-link' : 'about-link')}
        >
          {({ isActive }) => (isActive ? '→ Цены' : 'Цены')}
        </NavLink>
      </nav>
    </header>
  )
}

// Заглушка каталога (TODO 9), позже дополнена списком товаров (TODO 18).
function CatalogPage() {
  return (
    <div>
      <h1>Каталог</h1>

      {/* TODO 18: список товаров через .map() с key */}
      <ul className="product-list">
        {PRODUCTS.map((product) => (
          <li key={product.id}>
            <Link to={`/product/${product.id}`}>
              {product.title} — {product.price} ₽
            </Link>
          </li>
        ))}
      </ul>
    </div>
  )
}
```

Рисунок 2 - Результат выполнения программы task1.jsx

На рис. 2 показано меню, которое остаётся на месте при переходе между страницами. Ссылка `<a href>` перезагружает документ: React-приложение размонтируется, всё состояние и контекст теряются. `Link` перехватывает клик, меняет URL через History API и просит роутер отрисовать нужный маршрут без перезагрузки, поэтому состояние приложения сохраняется. Вынос `Header` за пределы `<Routes>` обеспечивает постоянное меню на всех страницах.

## Задание 3. NavLink – подсветка активной ссылки

Изучен `NavLink`, который автоматически определяет активность ссылки. В `Header` все три ссылки заменены на `NavLink`; для главной добавлен проп `end`, чтобы она не считалась активной на вложенных адресах. Для ссылки на `/about` использована функция в `className`: `({ isActive }) => (isActive ? 'active about-link' : 'about-link')`. Дополнительно применена children-функция, которая при активности выводит перед текстом «Цены» дополнительную стрелку.

В `task1.css` добавлено правило `.menu a.active`, которое задаёт цвет, жирность и подчёркивание активного пункта.

На листинге 4 показано: меню `Header` на `NavLink` с функцией `className` и children-функцией для стрелки.

**Листинг 4 - Задание №3 (task1.jsx)**

```jsx
// Header сразу сделан на NavLink (TODO 11) и с функцией className
// для ссылки «Цены» (TODO 14).
function Header() {
  return (
    <header className="site-header">
      <nav className="menu">
        <NavLink to="/" end>
          Расписание
        </NavLink>
        <NavLink to="/catalog">Тренеры</NavLink>

        {/* TODO 14: функция className + стрелка «→ » у активной ссылки */}
        <NavLink
          to="/about"
          className={({ isActive }) => (isActive ? 'active about-link' : 'about-link')}
        >
          {({ isActive }) => (isActive ? '→ Цены' : 'Цены')}
        </NavLink>
      </nav>
    </header>
  )
}
```

На листинге 5 показано: стили меню и правило `.menu a.active` для подсветки активной ссылки.

**Листинг 5 - Задание №3 (task1.css)**

```css
/* Стили для лабораторной работы №5 (React Router).
   Основное требование TODO 12 — правило для класса .active. */

.site-header {
  padding: 1rem 0;
  border-bottom: 1px solid #ddd;
}

.menu {
  display: flex;
  gap: 1.5rem;
  align-items: center;
}

.menu a {
  color: inherit;
  text-decoration: none;
}

/* TODO 12: подсветка активной ссылки NavLink */
.menu a.active {
  color: #e63946;
  font-weight: 700;
  text-decoration: underline;
}

.tabs {
  display: flex;
  gap: 1rem;
  margin: 1rem 0;
}

.product-list {
  display: grid;
  gap: 0.75rem;
  padding-left: 1.25rem;
}
```

Рисунок 3 - Результат выполнения программы task1.jsx, task1.css

На рис. 3 показана подсветка активного пункта меню. При переходе подсветка автоматически переезжает на выбранный пункт, а остальные пункты возвращаются к обычному виду. У ссылки «Цены» в активном состоянии дополнительно появляется стрелка. `NavLink` действительно добавляет класс `active`, но без CSS-правила для `.active` этот класс не влияет на внешний вид, поэтому кажется, что подсветка не работает. Правило в `task1.css` делает состояние видимым.

## Задание 4. useParams – динамические маршруты

Изучены динамические маршруты и хук `useParams`. В `Routes` добавлен маршрут `path="/product/:id"` с компонентом `ProductPage`. В `CatalogPage` выведен список из трёх товаров через `PRODUCTS.map()` с `key`; каждая карточка является ссылкой `<Link to={`/product/${product.id}`}>`. В `ProductPage` параметр `id` получен через `useParams()`, товар найден через `PRODUCTS.find((item) => item.id === String(id))`. Если товара нет, выводится текст «Товар не найден».

На листинге 6 показано: данные товаров, список ссылок в `CatalogPage` и страница `ProductPage` с `useParams`.

**Листинг 6 - Задание №4 (task1.jsx)**

```jsx
// Данные каталога (тема варианта — спортивный клуб «Атлет»).
const PRODUCTS = [
  { id: '1', title: 'Абонемент на месяц', price: 2500 },
  { id: '2', title: 'Разовое занятие', price: 500 },
  { id: '3', title: 'Персональная тренировка', price: 1500 },
]

// ...

// Заглушка каталога (TODO 9), позже дополнена списком товаров (TODO 18).
function CatalogPage() {
  return (
    <div>
      <h1>Каталог</h1>

      {/* TODO 18: список товаров через .map() с key */}
      <ul className="product-list">
        {PRODUCTS.map((product) => (
          <li key={product.id}>
            <Link to={`/product/${product.id}`}>
              {product.title} — {product.price} ₽
            </Link>
          </li>
        ))}
      </ul>
    </div>
  )
}

// ...

function ProductPage() {
  // TODO 17: чтение параметра из адреса.
  const { id } = useParams()
  const navigate = useNavigate()

  // TODO 19: id — строка, поэтому приводим id товара к строке.
  const product = PRODUCTS.find((item) => item.id === String(id))

  // TODO 19: если товара нет — показываем сообщение.
  if (!product) {
    return <h1>Товар не найден</h1>
  }

  return (
    <div>
      {/* TODO 17/19: вместо номера показываем реальное название и цену */}
      <h1>{product.title}</h1>
      <p>{product.price} ₽</p>

      {/* TODO 21: кнопка «Назад к каталогу» через navigate(-1) */}
      <button type="button" onClick={() => navigate(-1)}>
        Назад к каталогу
      </button>

      {/* TODO 27: вкладки «Описание» / «Отзывы» */}
      <nav className="tabs">
        <Link to="">Описание</Link>
        <Link to="reviews">Отзывы</Link>
      </nav>

      {/* TODO 27: сюда рендерятся вложенные маршруты */}
      <Outlet />
    </div>
  )
}
```

Рисунок 4 - Результат выполнения программы task1.jsx

На рис. 4 показана страница конкретного товара, открытая по адресу `/product/1`. `useParams()` возвращает значения строками, потому что параметр – это часть URL, то есть текст; роутер не знает, что в конкретном проекте `:id` означает число. Если забыть приведение типа, сравнение `"1" === 1` даст `false`, и страница товара с существующим `id` покажет «Товар не найден». Поэтому в коде используется `String(id)`.

## Задание 5. useNavigate – переход по коду

Изучен программный переход через `useNavigate`. В `ProductPage` добавлена кнопка «Назад к каталогу» с `onClick={() => navigate(-1)}`. На `AboutPage` создана controlled-форма с полем «Ваш email»: значение хранится в `useState`, изменяется через `onChange`, а в `handleSubmit` вызываются `event.preventDefault()` и `navigate("/")`. В TODO 23 исправлен `BrokenRedirect`: вызов `navigate("/")` перенесён из тела компонента в `useEffect`, чтобы переход выполнялся как побочный эффект, а не при каждом рендере.

На листинге 7 показано: форма на `AboutPage`, кнопка возврата в `ProductPage` и исправленный `BrokenRedirect`.

**Листинг 7 - Задание №5 (task1.jsx)**

```jsx
function AboutPage() {
  // TODO 22: controlled-форма + navigate("/") после отправки.
  const [email, setEmail] = useState('')
  const navigate = useNavigate()

  function handleSubmit(event) {
    event.preventDefault()
    navigate('/')
  }

  return (
    <div>
      <h1>О сайте</h1>
      {/* TODO 7: ссылка назад на главную */}
      <Link to="/">На главную</Link>

      <form onSubmit={handleSubmit}>
        <label>
          Ваш email
          <input
            type="email"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
        </label>
        <button type="submit">Отправить</button>
      </form>
    </div>
  )
}

// ...

      {/* TODO 21: кнопка «Назад к каталогу» через navigate(-1) */}
      <button type="button" onClick={() => navigate(-1)}>
        Назад к каталогу
      </button>

// ...

// TODO 23: Найдите ошибку
// Ниже компонент вызывает navigate прямо в теле функции, а не в ответ
// на действие пользователя — объясните, к чему это приведёт, и
// исправьте, перенеся вызов в подходящее место:
//
//   function BrokenRedirect() {
//     const navigate = useNavigate()
//     navigate("/")
//     return <p>Секунду...</p>
//   }
//
// Подсказка: вызов прямо в теле компонента срабатывает при КАЖДОМ
// рендере — подумайте, что произойдёт с адресной строкой браузера.

// Ошибка: navigate вызывается во время рендера, то есть при каждом
// рендере компонента. Это может привести к бесконечному циклу
// перерисовок/переходов и ошибке React о побочном эффекте в рендере.
// Исправление — перенести вызов в useEffect (или в обработчик события):
function BrokenRedirect() {
  const navigate = useNavigate()

  useEffect(() => {
    navigate('/')
  }, [navigate])

  return <p>Секунду...</p>
}
```

Рисунок 5 - Результат выполнения программы task1.jsx

На рис. 5 показан переход на главную страницу после отправки формы. `navigate("/path")` добавляет новую запись в историю и ведёт на конкретную страницу; это уместно после отправки формы, например `navigate("/")`. `navigate(-1)` возвращает на предыдущую запись истории, как кнопка «назад» браузера; это уместно для кнопки «Назад к каталогу», чтобы вернуть пользователя туда, откуда он пришёл. Вызывать `navigate` в теле компонента нельзя: это побочный эффект, который срабатывает на каждом рендере и приводит к циклу переходов.

## Задание 6. Вложенные маршруты и Outlet

Изучены вложенные маршруты и компонент `Outlet`. Созданы `ProductDetails` с текстом «Описание товара» и `ProductReviews`, который выводит список отзывов-заглушек через `.map()` с `key`. Оба компонента вложены в маршрут `/product/:id`: `ProductDetails` подключён как `index`-маршрут, `ProductReviews` – как `path="reviews"` без слэша в начале. В `ProductPage` добавлены `<Outlet />` и две ссылки-вкладки: `<Link to="">Описание</Link>` и `<Link to="reviews">Отзывы</Link>`.

На листинге 8 показано: страница `ProductPage` с `Outlet` и вкладками, компоненты `ProductDetails` и `ProductReviews`, а также вложенные маршруты.

**Листинг 8 - Задание №6 (task1.jsx)**

```jsx
function ProductPage() {
  // TODO 17: чтение параметра из адреса.
  const { id } = useParams()
  const navigate = useNavigate()

  // TODO 19: id — строка, поэтому приводим id товара к строке.
  const product = PRODUCTS.find((item) => item.id === String(id))

  // TODO 19: если товара нет — показываем сообщение.
  if (!product) {
    return <h1>Товар не найден</h1>
  }

  return (
    <div>
      {/* TODO 17/19: вместо номера показываем реальное название и цену */}
      <h1>{product.title}</h1>
      <p>{product.price} ₽</p>

      {/* TODO 21: кнопка «Назад к каталогу» через navigate(-1) */}
      <button type="button" onClick={() => navigate(-1)}>
        Назад к каталогу
      </button>

      {/* TODO 27: вкладки «Описание» / «Отзывы» */}
      <nav className="tabs">
        <Link to="">Описание</Link>
        <Link to="reviews">Отзывы</Link>
      </nav>

      {/* TODO 27: сюда рендерятся вложенные маршруты */}
      <Outlet />
    </div>
  )
}

// ...

// TODO 25: Компоненты вкладок
// Создайте два простых компонента: ProductDetails (текст "Описание
// товара") и ProductReviews (текст "Отзывов пока нет").

function ProductDetails() {
  return <p>Описание товара</p>
}

function ProductReviews() {
  // TODO 28: «отзывы»-заглушки через .map() с key.
  const reviews = ['Полотенце', 'Вода']

  return (
    <ul>
      {reviews.map((review) => (
        <li key={review}>{review}</li>
      ))}
    </ul>
  )
}

// ...

// TODO 26: Вложенные маршруты
// Вложите оба компонента в маршрут "/product/:id": ProductDetails —
// как index-маршрут, ProductReviews — как path="reviews" (без слэша в
// начале).

// Подсказка: <Route path="/product/:id" element={<ProductPage />}>
// ...вложенные Route внутри... </Route>.

// Настроено в итоговом <App /> ниже:
//   <Route path="/product/:id" element={<ProductPage />}>
//     <Route index element={<ProductDetails />} />
//     <Route path="reviews" element={<ProductReviews />} />
//   </Route>

// ...

          <Route path="/product/:id" element={<ProductPage />}>
            <Route index element={<ProductDetails />} />
            <Route path="reviews" element={<ProductReviews />} />
          </Route>
```

Рисунок 6 - Результат выполнения программы task1.jsx

На рис. 6 показаны вкладки «Описание» и «Отзывы», которые переключаются без перезагрузки страницы. Если вложенные маршруты настроены, но `<Outlet />` в родительском компоненте забыт, роутер будет считать их активными и менять URL, однако элементы не отрисуются: родителю некуда вставить дочерний маршрут. Пользователь увидит страницу товара без вкладки «Описание» или «Отзывы», хотя адрес будет меняться. `Outlet` – обязательная точка вставки вложенного содержимого.

## Задание 7. Страница 404

Изучена обработка несуществующих адресов. Создан `NotFoundPage` с текстом «404 – страница не найдена» и ссылкой `<Link to="/">Вернуться на главную</Link>`. В конец `<Routes>` добавлен маршрут `path="*"` с элементом `<NotFoundPage />`; звёздочка ловит любой адрес, который не подошёл маршрутам выше.

На листинге 9 показано: компонент `NotFoundPage` и маршрут-ловушка `path="*"`.

**Листинг 9 - Задание №7 (task1.jsx)**

```jsx
// TODO 30: Страница "не найдено"
// Создайте компонент NotFoundPage с текстом "404 — страница не
// найдена" и добавьте на неё <Link to="/">Вернуться на главную</Link>.

function NotFoundPage() {
  return (
    <div>
      <h1>404 — страница не найдена</h1>
      <Link to="/">Вернуться на главную</Link>
    </div>
  )
}

// ...

// TODO 31: Маршрут-ловушка
// Добавьте в самый конец Routes маршрут path="*" с элементом
// <NotFoundPage />.

// Подсказка: звёздочка означает «любой адрес, не подошедший ни одному
// маршруту выше» — по смыслу его всегда ставят последним в списке.

// Добавлено в итоговый <App /> ниже:
//   <Route path="*" element={<NotFoundPage />} />

// ...

          <Route path="*" element={<NotFoundPage />} />
```

Рисунок 7 - Результат выполнения программы task1.jsx

На рис. 7 показана страница 404 для несуществующего адреса. До добавления `path="*"` ни один маршрут не совпадал с таким адресом, и `<Routes>` ничего не рендерил: на экране оставалась пустая область, а снаружи мог отображаться только `Header`. После добавления `path="*"` на адрес вроде `/qwerty` показывается `NotFoundPage`, а не пустая страница.

## Итоговое задание

Итоговое приложение собрано в `task_itog.jsx` по варианту Б. Сайт GameZone использует маршруты `/` и `/about`, а также ловушку `*`; разделы «Услуги», «Игры» и «Контакты» открываются как секции главной через хэш-адреса `/#features`, `/#catalog` и `/#contacts`. Меню `Header` построено на `Link` и получает активный пункт из `App`; переходы к секциям выполняются через состояние `scrollRequest` и `scrollIntoView`. Хук `useLocation` синхронизирует подсветку меню с `pathname` и `hash`, а `useNavigate` используется в кнопках «О нас» и «На главную». Компоненты итогового приложения вынесены в `src/components/*.jsx`, пользовательские хуки – в `src/hooks/*.js`.

Вариант А полностью реализован в `task1.jsx`: маршруты `/`, `/about`, `/catalog`, `/product/:id` с вложенными вкладками и `path="*"`, форма на `/about`, кнопка `navigate(-1)` и `NavLink` в меню. В `task_itog.jsx` сохранены те же принципы маршрутизации, но структура сайта собственная: каталог и форма являются секциями главной, а отдельной страницей вынесена «О нас».

На листинге 10 показано: итоговое приложение `task_itog.jsx` с маршрутами, меню, обработчиками навигации и страницей 404.

**Листинг 10 - Итоговое задание (task_itog.jsx)**

```jsx
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Link, Route, Routes, useLocation, useNavigate } from "react-router-dom";
import Header from "./components/Header.jsx";
import Hero from "./components/Hero.jsx";
import Features from "./components/Features.jsx";
import Catalog, { GAMES } from "./components/Catalog.jsx";
import Favorites from "./components/Favorites.jsx";
import Stats from "./components/Stats.jsx";
import Promo from "./components/Promo.jsx";
import Review from "./components/Review.jsx";
import Faq from "./components/Faq.jsx";
import RequestForm from "./components/RequestForm.jsx";
import Footer from "./components/Footer.jsx";
import Modal from "./components/Modal.jsx";
import AboutPage from "./components/AboutPage.jsx";
import Background from "./components/Background.jsx";
import ScrollUI from "./components/ScrollUI.jsx";
import Toasts from "./components/Toasts.jsx";
import Confetti from "./components/Confetti.jsx";
import useLocalStorage from "./hooks/useLocalStorage.js";
import "./task_itog.css";

const NAV_ITEMS = [
  { label: "Главная", target: "home", to: "/" },
  { label: "О нас", target: "about", to: "/about" },
  { label: "Услуги", target: "features", to: "/#features" },
  { label: "Игры", target: "catalog", to: "/#catalog" },
  { label: "Контакты", target: "contacts", to: "/#contacts" },
];

const SCROLL_TOP = "top";

// Секции главной страницы, за которыми следит «scroll spy»
const SPY_TARGETS = ["home", "features", "catalog", "contacts"];

const FEATURES = [
  {
    id: "quality",
    icon: "star",
    title: "Качество",
    text: "Мы тщательно следим за каждой деталью.",
  },
  {
    id: "speed",
    icon: "bolt",
    title: "Скорость",
    text: "Быстро решаем любые вопросы клиентов.",
  },
  {
    id: "care",
    icon: "heart",
    title: "Забота о клиенте",
    text: "Индивидуальный подход к каждому.",
  },
];

const KONAMI = [
  "ArrowUp", "ArrowUp", "ArrowDown", "ArrowDown",
  "ArrowLeft", "ArrowRight", "ArrowLeft", "ArrowRight", "KeyB", "KeyA",
];

const TOAST_MS = 3200;

function NotFoundPage() {
  return (
    <main className="page-enter">
      <section className="hero hero--compact">
        <div className="hero-glow" aria-hidden="true" />
        <h2 className="hero-title">
          404 — <span className="gradient-text">страница не найдена</span>
        </h2>
        <p>Похоже, такой страницы в GameZone нет.</p>
        <div className="hero-actions">
          <Link to="/" className="btn">
            Вернуться на главную
          </Link>
        </div>
      </section>
    </main>
  );
}

function App() {
  const navigate = useNavigate();
  const location = useLocation();

  const [activeItem, setActiveItem] = useState(NAV_ITEMS[0].label);

  const [selectedFeatureIds, setSelectedFeatureIds] = useState([]);
  const [favorites, setFavorites] = useLocalStorage("gamezone-favorites", []);

  const [couponGranted, setCouponGranted] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const [scrollRequest, setScrollRequest] = useState(null);
  const [toasts, setToasts] = useState([]);
  const [partyMode, setPartyMode] = useState(false);
  const [konamiBurst, setKonamiBurst] = useState(0);

  const toastId = useRef(0);
  const isAutoScrolling = useRef(false);
  const partyRef = useRef(false);

  // ---------- уведомления ----------
  const dismissToast = useCallback((id) => {
    setToasts((prev) =>
      prev.map((toast) => (toast.id === id ? { ...toast, leaving: true } : toast))
    );
    setTimeout(() => {
      setToasts((prev) => prev.filter((toast) => toast.id !== id));
    }, 300);
  }, []);

  const notify = useCallback(
    (message, type = "info") => {
      toastId.current += 1;
      const id = toastId.current;
      setToasts((prev) => [...prev.slice(-3), { id, message, type }]);
      setTimeout(() => dismissToast(id), TOAST_MS);
    },
    [dismissToast]
  );

  // ---------- прокрутка ----------
  useEffect(() => {
    if (!scrollRequest) return undefined;

    isAutoScrolling.current = true;
    const timeoutId = setTimeout(() => {
      isAutoScrolling.current = false;
    }, 900);

    if (scrollRequest.target === SCROLL_TOP) {
      window.scrollTo({ top: 0, behavior: "smooth" });
    } else {
      document
        .getElementById(scrollRequest.target)
        ?.scrollIntoView({ behavior: "smooth", block: "start" });
    }

    return () => clearTimeout(timeoutId);
  }, [scrollRequest]);

  // Подсвечиваем пункт меню той секции, которая сейчас на экране
  useEffect(() => {
    if (location.pathname !== "/" || !("IntersectionObserver" in window)) return undefined;

    const observer = new IntersectionObserver(
      (entries) => {
        if (isAutoScrolling.current) return;
        const visible = entries
          .filter((entry) => entry.isIntersecting)
          .sort((a, b) => b.intersectionRatio - a.intersectionRatio)[0];
        if (!visible) return;
        const item = NAV_ITEMS.find((nav) => nav.target === visible.target.id);
        if (item) setActiveItem(item.label);
      },
      { rootMargin: "-35% 0px -55% 0px", threshold: [0, 0.25, 0.5] }
    );

    SPY_TARGETS.forEach((id) => {
      const node = document.getElementById(id);
      if (node) observer.observe(node);
    });

    return () => observer.disconnect();
  }, [location.pathname]);

  // Синхронизируем активный пункт меню с адресом (прямой заход,
  // кнопки браузера «назад/вперёд», переход по хэшу).
  useEffect(() => {
    if (location.pathname === "/about") {
      setActiveItem("О нас");
      return;
    }

    if (location.pathname === "/") {
      const hash = location.hash.replace("#", "");
      const item = NAV_ITEMS.find((nav) => nav.target === hash);
      setActiveItem(item ? item.label : "Главная");
    }
  }, [location.pathname, location.hash]);

  // Если открыли адрес с хэшем (например, /#catalog), прокручиваем к секции.
  useEffect(() => {
    if (location.pathname !== "/" || !location.hash) return;
    const target = location.hash.replace("#", "");
    if (document.getElementById(target)) {
      setScrollRequest({ target });
    }
  }, [location.pathname, location.hash]);

  // ---------- пасхалка: код Konami ----------
  // Общая функция: вызывается и с клавиатуры, и по клику на стрелки в подвале
  const triggerSecret = useCallback(() => {
    setKonamiBurst((prev) => prev + 1);
    partyRef.current = !partyRef.current;
    setPartyMode(partyRef.current);
    notify(
      partyRef.current ? "🎉 Секретный режим вечеринки!" : "Режим вечеринки выключен",
      "info"
    );
  }, [notify]);

  useEffect(() => {
    // Последние нажатые клавиши — сравниваем их «хвост» с кодом
    let history = [];

    function handleKey(event) {
      // event.code — физическая клавиша, поэтому код работает и на русской раскладке
      if (event.repeat) return;
      history = [...history, event.code].slice(-KONAMI.length);

      if (history.join() === KONAMI.join()) {
        history = [];
        triggerSecret();
      }
    }

    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [triggerSecret]);

  useEffect(() => {
    document.body.classList.toggle("party-mode", partyMode);
  }, [partyMode]);

  // Конфетти на весь экран после ввода кода — само исчезает через 4 секунды
  useEffect(() => {
    if (!konamiBurst) return undefined;
    const timeoutId = setTimeout(() => setKonamiBurst(0), 4000);
    return () => clearTimeout(timeoutId);
  }, [konamiBurst]);

  function requestScroll(target) {
    setScrollRequest({ target });
  }

  // Ссылки в Header сами меняют маршрут через <Link>; здесь только
  // обновляем подсветку и прокручиваем к нужной секции.
  function handleNavigate(item) {
    setActiveItem(item.label);

    if (item.target === "about") {
      requestScroll(SCROLL_TOP);
      return;
    }

    requestScroll(item.target === "home" ? SCROLL_TOP : item.target);
  }

  function handleShowAbout() {
    setActiveItem("О нас");
    requestScroll(SCROLL_TOP);
    navigate("/about");
  }

  function handleBackHome() {
    setActiveItem("Главная");
    requestScroll(SCROLL_TOP);
    navigate("/");
  }

  function goToSection(target) {
    const item = NAV_ITEMS.find((nav) => nav.target === target);
    if (item) setActiveItem(item.label);

    if (location.pathname !== "/") {
      navigate(`/#${target}`);
      return;
    }

    requestScroll(target);
  }

  // ---------- услуги и избранное ----------
  function handleSelectFeature(id) {
    setSelectedFeatureIds((prev) =>
      prev.includes(id) ? prev.filter((item) => item !== id) : [...prev, id]
    );
  }

  function handleToggleFavorite(title) {
    if (favorites.includes(title)) {
      setFavorites((prev) => prev.filter((item) => item !== title));
      notify(`«${title}» убрано из избранного`, "info");
    } else {
      setFavorites((prev) => (prev.includes(title) ? prev : [...prev, title]));
      notify(`«${title}» добавлено в избранное`, "fav");
    }
  }

  function handleRemoveFavorite(title) {
    setFavorites((prev) => prev.filter((item) => item !== title));
  }

  function handleClearFavorites() {
    setFavorites([]);
    notify("Избранное очищено", "info");
  }

  // Подпись типа элемента в списке избранного
  const favoriteKinds = useMemo(() => {
    const kinds = {};
    FEATURES.forEach((feature) => {
      kinds[feature.title] = "услуга";
    });
    GAMES.forEach((game) => {
      kinds[game.title] = "игра";
    });
    return kinds;
  }, []);

  function handleRequestSuccess() {
    setCouponGranted(true);
  }

  const openModal = useCallback(() => setIsModalOpen(true), []);
  const closeModal = useCallback(() => setIsModalOpen(false), []);

  return (
    <>
      <Background />
      <ScrollUI />

      <Header
        title="GameZone"
        navItems={NAV_ITEMS}
        activeItem={activeItem}
        onNavigate={handleNavigate}
        favoritesCount={favorites.length}
        onFavoritesClick={() => goToSection("favorites")}
      />

      <Routes>
        <Route
          path="/"
          element={
            <main className="page-enter" key="home">
              <Hero
                onShowAbout={handleShowAbout}
                onOpenModal={openModal}
                onExplore={() => goToSection("catalog")}
              />

              <Stats />

              <Features
                features={FEATURES}
                selectedIds={selectedFeatureIds}
                favorites={favorites}
                onSelect={handleSelectFeature}
                onToggleFavorite={handleToggleFavorite}
              />

              <Catalog
                favorites={favorites}
                onToggleFavorite={handleToggleFavorite}
              />

              <Favorites
                favorites={favorites}
                kinds={favoriteKinds}
                onRemove={handleRemoveFavorite}
                onClear={handleClearFavorites}
              />

              <Promo
                couponGranted={couponGranted}
                onOpenModal={openModal}
                onNotify={notify}
                onGoToForm={() => goToSection("contacts")}
              />

              <Review />

              <Faq />

              <RequestForm
                onRequestSuccess={handleRequestSuccess}
                onNotify={notify}
              />
            </main>
          }
        />

        <Route
          path="/about"
          element={
            <AboutPage
              key="about"
              onBack={handleBackHome}
              onOpenModal={openModal}
            />
          }
        />

        <Route path="*" element={<NotFoundPage />} />
      </Routes>

      <Footer onSecret={triggerSecret} />

      <Toasts toasts={toasts} onDismiss={dismissToast} />

      {konamiBurst > 0 && (
        <div className="confetti-screen" aria-hidden="true">
          <Confetti key={konamiBurst} count={140} />
        </div>
      )}

      <Modal
        isOpen={isModalOpen}
        title="Правила акции"
        onClose={closeModal}
      >
        <p>
          Оставьте заявку на главной странице, пока идёт таймер, и получите
          купон.
        </p>
        <ul className="modal-steps">
          <li>Заполните все поля формы заявки.</li>
          <li>Дождитесь сообщения об успешной отправке.</li>
          <li>Назовите купон при оформлении заказа.</li>
        </ul>
        <p>Одно нажатие на «Понятно» — и можно играть дальше.</p>
        <button type="button" className="btn" onClick={closeModal}>
          Понятно
        </button>
      </Modal>
    </>
  );
}

export default App;
```

На листинге 11 показано: стили итогового приложения `task_itog.css`, включая оформление меню, секций и адаптивные правила.

**Листинг 11 - Итоговое задание (task_itog.css)**

```css
/*
  ЛАБОРАТОРНАЯ РАБОТА №2
  ИТОГОВОЕ ЗАДАНИЕ
  Вариант 04
  Стиль: «Игра» — GameZone
*/

* {
    box-sizing: border-box;
}


body {
    margin: 0;
    padding-top: 76px;
    font-family: var(--font-body);
    font-size: 17px;
    line-height: 1.5;
    color: #e8ecff;
    background-color: var(--bg);
    overflow-x: hidden;
}

h1 {
    font-size: 40px;
    margin: 0;
    letter-spacing: 2px;
    text-transform: uppercase;
    text-shadow: 0 0 18px rgba(0, 229, 255, 0.55);
}

h2 {
    font-size: 30px;
    margin: 0 0 16px;
}

h3 {
    font-size: 23px;
    margin: 0 0 8px;
    color: #00e5ff;
}

p {
    margin: 0 0 16px;
}

a {
    color: #00e5ff;
    text-decoration: none;
    transition: color 0.25s ease;
}

header {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    padding: 14px 32px;
    background-color: rgba(23, 27, 49, 0.62);
    border-bottom: 1px solid rgba(0, 229, 255, 0.35);
    backdrop-filter: blur(14px) saturate(140%);
    -webkit-backdrop-filter: blur(14px) saturate(140%);
    transition: padding 0.3s ease, background-color 0.3s ease,
                box-shadow 0.3s ease;
    animation: header-in 0.7s var(--ease-out) both;
}

.nav {
    display: flex;
    position: relative;
    gap: 6px;
}

.nav a {
    position: relative;
    z-index: 1;
    color: #e8ecff;
    padding: 8px 14px;
    border: none;
    border-radius: 8px;
    font-weight: 500;
    transition: color 0.25s ease, background-color 0.25s ease,
                border-color 0.25s ease, transform 0.25s ease;
}

main {
    max-width: 1100px;
    margin: 0 auto;
    padding: 0 32px 64px;
}

.hero {
    position: relative;
    overflow: hidden;
    margin: 40px 0 32px;
    padding: 64px 32px 72px;
    text-align: center;
    background: linear-gradient(135deg, rgba(23, 27, 49, 0.82), rgba(36, 26, 77, 0.82));
    border: 1px solid #2c3566;
    border-radius: 16px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
    isolation: isolate;
}

.hero:hover {
    border-color: #00e5ff;
    box-shadow: 0 0 40px rgba(0, 229, 255, 0.18);
}

.hero p {
    font-size: 30px;
    margin: 0;
    color: #cfd6ff;
    min-height: 1.5em;
    animation: reveal-up 0.9s var(--ease-out) 0.35s backwards;
}

.hero p::before {
    content: "▶ ";
    color: #ff2e88;
}

.hero p::after {
    content: " ◀";
    color: #ff2e88;
}

.features {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
    margin-bottom: 48px;
}

.feature-card {
    position: relative;
    overflow: hidden;
    padding: 24px;
    background-color: var(--panel);
    border: 1px solid #2c3566;
    border-top: 4px solid #00e5ff;
    border-radius: 16px;
    cursor: pointer;
    transition: border-color 0.3s ease, transform 0.3s ease,
                box-shadow 0.3s ease;
}

.feature-card p {
    margin: 0;
    color: #a9b1d9;
}

.feature-card:nth-child(2) {
    border-top-color: #ff2e88;
}

.feature-card:hover {
    border-color: #00e5ff;
    transform: translateY(-6px);
    box-shadow: 0 14px 34px rgba(0, 229, 255, 0.25);
}

.promo {
    margin-bottom: 48px;
}

.promo-card {
    position: relative;
    overflow: visible;
    isolation: isolate;
    padding: 40px 32px;
    background: linear-gradient(120deg, #3a1060, #171b31);
    border: 1px solid #ff2e88;
    border-radius: 16px;
    transition: box-shadow 0.3s ease, border-color 0.4s ease;
}

.promo-card:hover {
    box-shadow: 0 14px 34px rgba(255, 46, 136, 0.3);
}

.badge {
    position: absolute;
    top: -14px;
    right: -14px;
    padding: 8px 16px;
    font-size: 13px;
    color: #ffffff;
    background-color: #ff2e88;
    border-radius: 999px;
    letter-spacing: 1px;
    text-transform: uppercase;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
    animation: badge-float 3s ease-in-out infinite;
}

.promo-card:hover .badge {
    transform: rotate(6deg) scale(1.06);
    box-shadow: 0 0 18px rgba(255, 46, 136, 0.7);
}

.form-section {
    position: relative;
    overflow: hidden;
    padding: 32px;
    margin-bottom: 24px;
    background-color: #171b31;
    border: 1px solid #2c3566;
    border-radius: 16px;
}

.form-section h2 {
    text-align: center;
}

.form-section h2::after {
    content: " — поля обязательны";
    display: block;
    margin-top: 6px;
    font-size: 13px;
    color: #ff2e88;
    letter-spacing: 1px;
    text-transform: uppercase;
}

form {
    display: flex;
    flex-direction: column;
    gap: 14px;
    width: 100%;
    max-width: 420px;
    margin: 0 auto;
}

form input[type="text"],
form input[type="email"],
form input[type="password"],
form input[type="tel"] {
    width: 100%;
    padding: 14px 16px;
    font-family: inherit;
    font-size: 17px;
    color: #e8ecff;
    background-color: #0d1020;
    border: 2px solid #2c3566;
    border-radius: 10px;
    transition: border-color 0.25s ease, box-shadow 0.25s ease,
                background-color 0.25s ease;
}

form input::placeholder {
    color: #6b74a8;
}

form input:hover {
    border-color: #4b5690;
}

form input:focus {
    border-color: #00e5ff;
    box-shadow: 0 0 0 3px rgba(0, 229, 255, 0.25);
    outline: none;
}

.radio-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: 6px 0 0;
    padding: 14px 18px 18px;
    border: 1px solid #2c3566;
    border-radius: 10px;
    transition: border-color 0.25s ease, box-shadow 0.25s ease;
}

.radio-group:hover {
    border-color: #ff2e88;
    box-shadow: 0 0 0 3px rgba(255, 46, 136, 0.12);
}

.radio-group legend {
    padding: 0 8px;
    font-size: 13px;
    color: #a9b1d9;
    letter-spacing: 1px;
    text-transform: uppercase;
}

.radio-group label {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    transition: color 0.25s ease;
}

.radio-group label:hover {
    color: #00e5ff;
}

form input[type="radio"] {
    width: 18px;
    height: 18px;
    accent-color: #ff2e88;
}

.btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    width: 100%;
    margin-top: 6px;
    padding: 16px 24px;
    overflow: hidden;
    font-family: inherit;
    font-size: 17px;
    font-weight: 600;
    color: #0d1020;
    background-color: #00e5ff;
    border: none;
    border-radius: 10px;
    letter-spacing: 1px;
    text-transform: uppercase;
    cursor: pointer;
    isolation: isolate;
    transition: color 0.25s ease, background-color 0.25s ease,
                transform 0.25s ease, box-shadow 0.25s ease;
}

.btn:hover {
    color: #ffffff;
    background-color: #ff2e88;
    transform: translateY(-2px);
    box-shadow: 0 10px 26px rgba(255, 46, 136, 0.45);
}

#form-status {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin-top: 16px;
    padding: 12px 16px;
    font-size: 14px;
    font-weight: 600;
    text-align: center;
    border-radius: 10px;
    animation: pop-in 0.4s var(--ease-spring);
}

#form-status.status-success {
    color: #4ade80;
    background: rgba(74, 222, 128, 0.1);
    border: 1px solid rgba(74, 222, 128, 0.4);
}

#form-status.status-error {
    color: #f87171;
    background: rgba(248, 113, 113, 0.08);
    border: 1px solid rgba(248, 113, 113, 0.35);
}

#live-clock {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
    font-size: 15px;
    font-weight: 600;
    color: #00e5ff;
    letter-spacing: 1px;
    font-variant-numeric: tabular-nums;
    background: rgba(0, 229, 255, 0.07);
    border: 1px solid rgba(0, 229, 255, 0.25);
    border-radius: 999px;
}

.feature-card.selected {
    border-color: #ff2e88;
    box-shadow: 0 0 30px rgba(255, 46, 136, 0.35);
    transform: translateY(-6px);
}

.add-fav-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    margin-top: 16px;
    padding: 8px 16px;
    font-family: inherit;
    font-size: 14px;
    font-weight: 600;
    color: #e8ecff;
    background-color: transparent;
    border: 1px solid #00e5ff;
    border-radius: 8px;
    cursor: pointer;
    transition: color 0.25s ease, background-color 0.25s ease,
                transform 0.25s ease;
}

.add-fav-btn:hover {
    color: #0d1020;
    background-color: #00e5ff;
    transform: translateY(-2px);
}

.favorites,
.review {
    margin-bottom: 48px;
    padding: 32px;
    background-color: #171b31;
    border: 1px solid #2c3566;
    border-radius: 16px;
}

#favorites-empty {
    display: flex;
    align-items: center;
    gap: 10px;
    margin: 0;
    color: #a9b1d9;
}

#favorites-list {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 12px;
}

#favorites-list li {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    max-height: 90px;
    padding: 12px 16px;
    overflow: hidden;
    background-color: #0d1020;
    border: 1px solid #2c3566;
    border-radius: 8px;
    animation: fav-in 0.45s var(--ease-spring);
    transition: border-color 0.25s ease, transform 0.25s ease;
}

.remove-fav-btn {
    padding: 6px 12px;
    font-size: 13px;
    font-weight: 600;
    color: #f87171;
    background-color: transparent;
    border: 1px solid #f87171;
    border-radius: 8px;
    cursor: pointer;
    transition: color 0.25s ease, background-color 0.25s ease;
}

.remove-fav-btn:hover {
    color: #ffffff;
    background-color: #f87171;
}

#promo-timer {
    display: inline-block;
    margin-bottom: 16px;
    font-family: var(--font-display);
    font-size: 32px;
    font-weight: 700;
    color: #ff2e88;
    letter-spacing: 2px;
    font-variant-numeric: tabular-nums;
    text-shadow: 0 0 18px rgba(255, 46, 136, 0.6);
}

#promo-coupon {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin: 0;
    margin-top: 8px;
    padding: 14px 18px;
    font-size: 16px;
    font-weight: 700;
    color: #0d1020;
    background: linear-gradient(120deg, #00e5ff, #4ade80);
    background-size: 200% 100%;
    border-radius: 8px;
    letter-spacing: 0.5px;
    animation: coupon-in 0.6s var(--ease-spring), gradient-shift 4s linear infinite;
}

#review-content {
    position: relative;
    min-height: 168px;
    margin-bottom: 16px;
    padding: 20px;
    overflow: hidden;
    background-color: rgba(13, 16, 32, 0.75);
    border: 1px solid #2c3566;
    border-radius: 8px;
    color: #cfd6ff;
}

#review-content p {
    margin: 0 0 8px;
}

#review-content p:last-child {
    margin: 0;
}

.review-author {
    font-size: 14px;
    color: #00e5ff;
}

footer {
    position: relative;
    padding: 32px;
    font-size: 13px;
    text-align: center;
    color: #6b74a8;
    background-color: rgba(10, 12, 24, 0.85);
    border-top: 1px solid #2c3566;
}

/* ============================================================
   ЛАБОРАТОРНАЯ РАБОТА №4
   Дополнения к стилям ЛР2: корневой контейнер React, кнопки
   главной/второй страницы, прогресс таймера и модальное окно.
   ============================================================ */

#root {
    width: auto;
    max-width: none;
    min-height: auto;
    margin: 0;
    text-align: initial;
    border-inline: none;
    display: block;
}

/* Сброс стилей шаблона Vite, чтобы заголовки выглядели как в ЛР2 */
h1,
h2 {
    font-family: var(--font-display);
    font-weight: 400;
    letter-spacing: 0.5px;
    color: #e8ecff;
}

.hero-actions {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 16px;
    margin-top: 24px;
    animation: reveal-up 0.9s var(--ease-out) 0.5s backwards;
}

.hero-actions .btn {
    width: auto;
    margin-top: 0;
    padding: 14px 28px;
}

.btn.btn-outline {
    color: #00e5ff;
    background-color: transparent;
    border: 2px solid #00e5ff;
}

.btn.btn-outline:hover {
    color: #0d1020;
    background-color: #00e5ff;
    box-shadow: 0 10px 26px rgba(0, 229, 255, 0.35);
}

.btn:disabled {
    cursor: wait;
    opacity: 0.6;
    transform: none;
}

.promo-btn {
    width: auto;
    margin-bottom: 16px;
    padding: 12px 24px;
}

.promo-progress {
    height: 8px;
    margin-bottom: 16px;
    background-color: #0d1020;
    border: 1px solid #2c3566;
    border-radius: 999px;
    overflow: hidden;
}

.promo-progress span {
    display: block;
    height: 100%;
    background: linear-gradient(90deg, #00e5ff, #ff2e88);
    border-radius: 999px;
    transition: width 1s linear;
}

.modal-overlay {
    position: fixed;
    inset: 0;
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    background-color: rgba(13, 16, 32, 0.72);
    backdrop-filter: blur(8px);
    -webkit-backdrop-filter: blur(8px);
    animation: overlay-fade 0.25s ease;
}

.modal {
    position: relative;
    width: 100%;
    max-width: 480px;
    max-height: 85vh;
    overflow-y: auto;
    padding: 32px;
    background: linear-gradient(135deg, #171b31, #241a4d);
    border: 1px solid #00e5ff;
    border-radius: 16px;
    box-shadow: 0 24px 60px rgba(0, 229, 255, 0.28);
    animation: modal-pop 0.4s var(--ease-spring);
}

.modal h2 {
    padding-right: 36px;
}

.modal ul {
    margin: 0 0 16px;
    padding-left: 20px;
    color: #cfd6ff;
}

.modal .btn {
    width: auto;
    padding: 12px 24px;
}

.modal-close {
    position: absolute;
    top: 12px;
    right: 12px;
    width: 36px;
    height: 36px;
    font-family: inherit;
    font-size: 22px;
    line-height: 1;
    color: #e8ecff;
    background-color: transparent;
    border: 1px solid #2c3566;
    border-radius: 8px;
    cursor: pointer;
    transition: color 0.25s ease, background-color 0.25s ease, transform 0.3s var(--ease-spring);
}

.modal-close:hover {
    color: #0d1020;
    background-color: #ff2e88;
    transform: rotate(90deg);
}

.about-cta p {
    color: #a9b1d9;
}

.about-cta .hero-actions {
    justify-content: flex-start;
}

.status-error {
    color: #f87171;
}

#review-content .status-error {
    margin: 0;
}

@keyframes overlay-fade {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

@keyframes modal-pop {
    from {
        opacity: 0;
        transform: translateY(18px) scale(0.96);
    }
    to {
        opacity: 1;
        transform: translateY(0) scale(1);
    }
}

/* ============================================================
   ЛАБОРАТОРНАЯ РАБОТА №5
   Редизайн: анимированный фон, появление при прокрутке, каталог,
   FAQ, уведомления, улучшенная форма. Правила ниже дополняют и
   переопределяют стили ЛР2/ЛР4.
   ============================================================ */

:root {
    --cyan: #00e5ff;
    --pink: #ff2e88;
    --violet: #8b5cf6;
    --green: #4ade80;
    --yellow: #facc15;
    --red: #f87171;
    --bg: #0d1020;
    --panel: rgba(23, 27, 49, 0.72);
    --panel-solid: #171b31;
    --line: #2c3566;
    --text: #e8ecff;
    --muted: #a9b1d9;
    --ease-out: cubic-bezier(0.2, 0.7, 0.2, 1);
    --ease-spring: cubic-bezier(0.34, 1.56, 0.64, 1);
    --font-display: 'Russo One', 'Segoe UI', Tahoma, sans-serif;
    --font-body: 'Rubik', 'Segoe UI', Tahoma, sans-serif;
}

html {
    scroll-padding-top: 90px;
}

::selection {
    color: var(--bg);
    background: var(--cyan);
}

:focus-visible {
    outline: 2px solid var(--cyan);
    outline-offset: 3px;
}

/* ---------- Анимированный фон ---------- */

.bg-fx {
    position: fixed;
    inset: 0;
    z-index: -1;
    overflow: hidden;
    pointer-events: none;
}

.bg-orb {
    position: absolute;
    width: 46vmax;
    height: 46vmax;
    border-radius: 50%;
    filter: blur(90px);
    opacity: 0.32;
    animation: orb-float 22s ease-in-out infinite alternate;
}

.bg-orb--cyan {
    top: -18vmax;
    left: -12vmax;
    background: radial-gradient(circle, var(--cyan), transparent 65%);
}

.bg-orb--pink {
    right: -16vmax;
    top: 20vh;
    background: radial-gradient(circle, var(--pink), transparent 65%);
    animation-duration: 27s;
    animation-delay: -8s;
}

.bg-orb--violet {
    bottom: -22vmax;
    left: 25vw;
    background: radial-gradient(circle, var(--violet), transparent 65%);
    animation-duration: 31s;
    animation-delay: -14s;
}

.bg-grid {
    position: absolute;
    left: -50%;
    right: -50%;
    bottom: -10vh;
    height: 70vh;
    background-image:
        linear-gradient(rgba(0, 229, 255, 0.18) 1px, transparent 1px),
        linear-gradient(90deg, rgba(0, 229, 255, 0.18) 1px, transparent 1px);
    background-size: 64px 64px;
    transform: perspective(420px) rotateX(62deg);
    transform-origin: bottom;
    mask-image: linear-gradient(to top, #000 10%, transparent 85%);
    -webkit-mask-image: linear-gradient(to top, #000 10%, transparent 85%);
    animation: grid-move 6s linear infinite;
    opacity: 0.55;
}

.bg-spotlight {
    position: absolute;
    inset: 0;
    background: radial-gradient(
        520px circle at var(--cursor-x, 50%) var(--cursor-y, 30%),
        rgba(0, 229, 255, 0.09),
        transparent 60%
    );
}

.bg-scanlines {
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        to bottom,
        rgba(255, 255, 255, 0.025) 0 1px,
        transparent 1px 3px
    );
}

/* ---------- Полоса прокрутки и «Наверх» ---------- */

.scroll-progress {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 60;
    width: 100%;
    height: 3px;
    background: linear-gradient(90deg, var(--cyan), var(--violet), var(--pink));
    box-shadow: 0 0 12px rgba(0, 229, 255, 0.7);
    transform-origin: left;
    transform: scaleX(0);
}

.to-top {
    position: fixed;
    right: 24px;
    bottom: 24px;
    z-index: 40;
    display: grid;
    place-items: center;
    width: 52px;
    height: 52px;
    color: var(--cyan);
    background: rgba(23, 27, 49, 0.85);
    border: 1px solid var(--line);
    border-radius: 50%;
    cursor: pointer;
    opacity: 0;
    transform: translateY(20px) scale(0.8);
    pointer-events: none;
    backdrop-filter: blur(8px);
    transition: opacity 0.3s ease, transform 0.35s var(--ease-spring),
                color 0.25s ease, background-color 0.25s ease;
}

.to-top.is-shown {
    opacity: 1;
    transform: none;
    pointer-events: auto;
}

.to-top:hover {
    color: var(--bg);
    background: var(--cyan);
}

.to-top:hover .icon {
    animation: bounce-up 0.6s ease infinite;
}

.to-top-ring {
    position: absolute;
    inset: -1px;
    width: calc(100% + 2px);
    height: calc(100% + 2px);
    transform: rotate(-90deg);
}

.to-top-ring circle {
    fill: none;
    stroke: var(--pink);
    stroke-width: 2.5;
    stroke-dasharray: 100;
    stroke-linecap: round;
}

/* ---------- Шапка ---------- */

header.is-scrolled {
    padding-top: 9px;
    padding-bottom: 9px;
    background-color: rgba(13, 16, 32, 0.86);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.35), 0 1px 0 rgba(0, 229, 255, 0.4);
}

.brand {
    display: flex;
    align-items: center;
    gap: 12px;
    color: var(--text);
}

.brand-logo {
    display: grid;
    place-items: center;
    width: 42px;
    height: 42px;
    color: var(--bg);
    background: linear-gradient(135deg, var(--cyan), var(--pink));
    border-radius: 12px;
    box-shadow: 0 0 20px rgba(0, 229, 255, 0.45);
    transition: transform 0.4s var(--ease-spring);
}

.brand:hover .brand-logo {
    transform: rotate(-12deg) scale(1.08);
}

header h1 {
    font-size: 30px;
}

.glitch {
    position: relative;
}

.glitch::before,
.glitch::after {
    content: attr(data-text);
    position: absolute;
    inset: 0;
    opacity: 0;
    pointer-events: none;
}

.glitch::before {
    color: var(--pink);
    text-shadow: none;
}

.glitch::after {
    color: var(--cyan);
    text-shadow: none;
}

.brand:hover .glitch::before {
    opacity: 0.85;
    animation: glitch-a 0.6s steps(2) infinite;
}

.brand:hover .glitch::after {
    opacity: 0.85;
    animation: glitch-b 0.5s steps(2) infinite;
}

.clock-dot {
    width: 8px;
    height: 8px;
    background: var(--green);
    border-radius: 50%;
    box-shadow: 0 0 8px var(--green);
    animation: pulse 1s ease-in-out infinite alternate;
}

.nav a:hover {
    color: var(--cyan);
    background-color: rgba(0, 229, 255, 0.08);
    border-color: transparent;
    transform: translateY(-1px);
}

.nav a.active,
.nav a.active:hover {
    color: var(--bg);
    background-color: transparent;
    transform: none;
}

.nav-indicator {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    background: linear-gradient(135deg, var(--cyan), #5ff3ff);
    border-radius: 8px;
    box-shadow: 0 0 18px rgba(0, 229, 255, 0.5);
    opacity: 0;
    transition: transform 0.45s var(--ease-spring), width 0.45s var(--ease-spring),
                opacity 0.3s ease;
}

.nav-indicator.is-ready {
    opacity: 1;
}

.header-actions {
    display: flex;
    align-items: center;
    gap: 12px;
}

.fav-counter {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 7px 14px;
    font-family: inherit;
    font-size: 15px;
    font-weight: 600;
    color: var(--pink);
    background: rgba(255, 46, 136, 0.1);
    border: 1px solid rgba(255, 46, 136, 0.45);
    border-radius: 999px;
    cursor: pointer;
    transition: background-color 0.25s ease, color 0.25s ease, transform 0.25s ease;
}

.fav-counter:hover {
    color: #fff;
    background: var(--pink);
    transform: translateY(-2px);
}

.fav-counter.is-bumping {
    animation: bump 0.45s var(--ease-spring);
}

.burger {
    display: none;
    flex-direction: column;
    justify-content: center;
    gap: 5px;
    width: 42px;
    height: 42px;
    padding: 0 10px;
    background: transparent;
    border: 1px solid var(--line);
    border-radius: 10px;
    cursor: pointer;
}

.burger span {
    display: block;
    height: 2px;
    background: var(--text);
    border-radius: 2px;
    transition: transform 0.3s ease, opacity 0.3s ease;
}

header.menu-open .burger span:nth-child(1) {
    transform: translateY(7px) rotate(45deg);
}

header.menu-open .burger span:nth-child(2) {
    opacity: 0;
}

header.menu-open .burger span:nth-child(3) {
    transform: translateY(-7px) rotate(-45deg);
}

/* ---------- Общее: появление, секции, кнопки ---------- */

.page-enter {
    animation: page-in 0.6s var(--ease-out);
}

.reveal:not(.is-visible) {
    opacity: 0;
}

.reveal.is-visible {
    animation: reveal-up 0.8s var(--ease-out) var(--reveal-delay, 0ms) backwards;
}

.reveal--zoom.is-visible {
    animation-name: reveal-zoom;
}

.reveal--left.is-visible {
    animation-name: reveal-left;
}

.section-block {
    margin-bottom: 56px;
}

.section-head {
    margin-bottom: 28px;
    text-align: center;
}

.section-head p {
    margin: 0;
    color: var(--muted);
}

.section-kicker {
    display: inline-block;
    margin-bottom: 10px;
    padding: 4px 14px;
    font-size: 13px;
    font-weight: 600;
    color: var(--cyan);
    letter-spacing: 2px;
    text-transform: uppercase;
    background: rgba(0, 229, 255, 0.08);
    border: 1px solid rgba(0, 229, 255, 0.3);
    border-radius: 999px;
}

.selected-count {
    color: var(--pink);
    font-weight: 600;
}

.gradient-text {
    background: linear-gradient(90deg, var(--cyan), var(--violet), var(--pink), var(--cyan));
    background-size: 300% 100%;
    -webkit-background-clip: text;
    background-clip: text;
    color: transparent;
    animation: gradient-shift 6s linear infinite;
}

.btn::after {
    content: "";
    position: absolute;
    top: 0;
    left: -60%;
    z-index: -1;
    width: 40%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.45), transparent);
    transform: skewX(-20deg);
    transition: left 0.6s ease;
}

.btn:hover::after {
    left: 130%;
}

.btn:active {
    transform: scale(0.97);
}

.btn-glow {
    color: var(--bg);
    background: linear-gradient(110deg, var(--cyan), var(--violet), var(--pink), var(--cyan));
    background-size: 300% 100%;
    animation: gradient-shift 6s linear infinite, glow-pulse 2.6s ease-in-out infinite;
}

.btn-glow:hover {
    color: #fff;
}

.btn.is-loading {
    cursor: wait;
}

.spinner {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(13, 16, 32, 0.3);
    border-top-color: var(--bg);
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
}

.icon-btn {
    display: grid;
    place-items: center;
    width: 38px;
    height: 38px;
    color: var(--text);
    background: transparent;
    border: 1px solid var(--line);
    border-radius: 10px;
    cursor: pointer;
    transition: color 0.2s ease, background-color 0.2s ease, transform 0.2s ease;
}

.icon-btn:hover {
    color: var(--bg);
    background: var(--cyan);
    transform: translateY(-2px);
}

.favorites,
.review,
.form-section {
    background-color: var(--panel);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
}

/* ---------- Hero ---------- */

.hero--compact {
    padding: 48px 32px;
}

.hero-glow {
    position: absolute;
    inset: -60%;
    z-index: -1;
    background: conic-gradient(
        from 0deg,
        transparent 0 20%,
        rgba(0, 229, 255, 0.16) 30%,
        transparent 40% 60%,
        rgba(255, 46, 136, 0.16) 70%,
        transparent 80%
    );
    animation: spin 18s linear infinite;
}

.hero-kicker {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 20px;
    padding: 6px 16px;
    font-size: 14px;
    color: var(--muted);
    background: rgba(13, 16, 32, 0.6);
    border: 1px solid var(--line);
    border-radius: 999px;
    animation: reveal-up 0.8s var(--ease-out) 0.1s backwards;
}

.hero-kicker strong {
    display: inline-block;
    min-width: 3ch;
    color: var(--green);
    font-variant-numeric: tabular-nums;
    animation: tick 0.4s ease;
}

.pulse-dot {
    position: relative;
    width: 9px;
    height: 9px;
    background: var(--green);
    border-radius: 50%;
}

.pulse-dot::after {
    content: "";
    position: absolute;
    inset: 0;
    border-radius: 50%;
    background: var(--green);
    animation: ping 1.6s ease-out infinite;
}

.hero-title {
    margin: 0 0 18px;
    font-size: clamp(30px, 5.2vw, 56px);
    line-height: 1.12;
    animation: reveal-up 0.9s var(--ease-out) 0.2s backwards;
}

.caret {
    display: inline-block;
    width: 3px;
    height: 0.95em;
    margin-left: 4px;
    vertical-align: -0.1em;
    background: var(--cyan);
    box-shadow: 0 0 10px var(--cyan);
    animation: blink 1s steps(1) infinite;
}

.hero-scroll {
    position: absolute;
    left: 50%;
    bottom: 16px;
    width: 22px;
    height: 34px;
    margin-left: -11px;
    border: 2px solid rgba(232, 236, 255, 0.4);
    border-radius: 12px;
}

.hero-scroll span {
    position: absolute;
    top: 6px;
    left: 50%;
    width: 4px;
    height: 7px;
    margin-left: -2px;
    background: var(--cyan);
    border-radius: 2px;
    animation: scroll-wheel 1.8s ease-in-out infinite;
}

/* ---------- Статистика ---------- */

.stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 16px;
    margin-bottom: 56px;
}

.stat {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 22px 12px;
    text-align: center;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 16px;
    opacity: 0;
    transition: border-color 0.3s ease, transform 0.3s ease;
}

.stats.is-visible .stat {
    opacity: 1;
    animation: reveal-up 0.8s var(--ease-out) var(--reveal-delay, 0ms) backwards;
}

.stat:hover {
    border-color: var(--cyan);
    transform: translateY(-4px);
}

.stat-icon {
    display: grid;
    place-items: center;
    width: 44px;
    height: 44px;
    margin-bottom: 6px;
    color: var(--cyan);
    background: rgba(0, 229, 255, 0.1);
    border-radius: 12px;
}

.stat:nth-child(even) .stat-icon {
    color: var(--pink);
    background: rgba(255, 46, 136, 0.1);
}

.stat-value {
    font-family: var(--font-display);
    font-size: 32px;
    line-height: 1.1;
    color: var(--text);
    font-variant-numeric: tabular-nums;
}

.stat-label {
    font-size: 14px;
    color: var(--muted);
}

/* ---------- Карточки услуг с 3D-наклоном ---------- */

.feature-card:nth-child(3) {
    border-top-color: var(--violet);
}

.feature-card.tilt {
    transform: perspective(900px) rotateX(var(--rx, 0deg)) rotateY(var(--ry, 0deg))
               translateY(var(--lift, 0px));
    transform-style: preserve-3d;
    transition: transform 0.18s ease-out, border-color 0.3s ease, box-shadow 0.3s ease;
}

.feature-card.tilt:hover {
    --lift: -6px;
}

.feature-card.tilt.selected {
    --lift: -6px;
    transform: perspective(900px) rotateX(var(--rx, 0deg)) rotateY(var(--ry, 0deg))
               translateY(var(--lift, 0px));
}

.tilt-glare {
    position: absolute;
    inset: 0;
    background: radial-gradient(
        circle at var(--gx, 50%) var(--gy, 0%),
        rgba(255, 255, 255, 0.14),
        transparent 55%
    );
    opacity: 0;
    pointer-events: none;
    transition: opacity 0.3s ease;
}

.feature-card:hover .tilt-glare {
    opacity: 1;
}

.feature-icon {
    display: grid;
    place-items: center;
    width: 52px;
    height: 52px;
    margin-bottom: 16px;
    color: var(--cyan);
    background: rgba(0, 229, 255, 0.1);
    border: 1px solid rgba(0, 229, 255, 0.3);
    border-radius: 14px;
    transition: transform 0.4s var(--ease-spring);
}

.feature-card:nth-child(2) .feature-icon {
    color: var(--pink);
    background: rgba(255, 46, 136, 0.1);
    border-color: rgba(255, 46, 136, 0.3);
}

.feature-card:nth-child(3) .feature-icon {
    color: var(--violet);
    background: rgba(139, 92, 246, 0.12);
    border-color: rgba(139, 92, 246, 0.35);
}

.feature-card:hover .feature-icon {
    transform: translateZ(30px) rotate(-8deg) scale(1.1);
}

.selected-mark {
    position: absolute;
    top: 14px;
    right: 14px;
    display: grid;
    place-items: center;
    width: 28px;
    height: 28px;
    font-size: 15px;
    font-weight: 700;
    color: #fff;
    background: var(--pink);
    border-radius: 50%;
    box-shadow: 0 0 14px rgba(255, 46, 136, 0.7);
    animation: pop-in 0.4s var(--ease-spring);
}

.add-fav-btn.is-active {
    color: #fff;
    background-color: var(--pink);
    border-color: var(--pink);
}

.add-fav-btn.is-active .icon {
    animation: pop-in 0.4s var(--ease-spring);
}

/* ---------- Каталог игр ---------- */

.games-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 18px;
}

.game-card {
    --c1: var(--cyan);
    --c2: var(--violet);
    overflow: hidden;
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 16px;
    animation: card-in 0.55s var(--ease-out) calc(var(--i, 0) * 55ms) backwards;
    transition: transform 0.3s var(--ease-out), border-color 0.3s ease, box-shadow 0.3s ease;
}

.game-card:hover {
    border-color: var(--c1);
    transform: translateY(-6px);
    box-shadow: 0 16px 36px rgba(0, 0, 0, 0.45), 0 0 24px color-mix(in srgb, var(--c1) 35%, transparent);
}

.genre-shooter { --c1: #ff5f6d; --c2: #ffc371; }
.genre-moba { --c1: #8b5cf6; --c2: #00e5ff; }
.genre-fighting { --c1: #ff2e88; --c2: #8b5cf6; }
.genre-racing { --c1: #facc15; --c2: #ff5f6d; }
.genre-sport { --c1: #4ade80; --c2: #00e5ff; }

.game-cover {
    position: relative;
    display: grid;
    place-items: center;
    height: 130px;
    overflow: hidden;
    background:
        radial-gradient(circle at 20% 20%, rgba(255, 255, 255, 0.25), transparent 40%),
        linear-gradient(135deg, var(--c1), var(--c2));
}

.game-cover::after {
    content: "";
    position: absolute;
    inset: 0;
    background: repeating-linear-gradient(
        -45deg,
        rgba(255, 255, 255, 0.06) 0 8px,
        transparent 8px 16px
    );
}

.game-initials {
    position: relative;
    z-index: 1;
    font-family: var(--font-display);
    font-size: 54px;
    color: rgba(13, 16, 32, 0.75);
    letter-spacing: 2px;
    transition: transform 0.5s var(--ease-spring);
}

.game-card:hover .game-initials {
    transform: scale(1.15) rotate(-4deg);
}

.game-hot {
    position: absolute;
    top: 10px;
    left: 10px;
    z-index: 2;
    padding: 3px 10px;
    font-size: 12px;
    font-weight: 700;
    color: #fff;
    background: rgba(13, 16, 32, 0.75);
    border-radius: 999px;
}

.game-fav {
    position: absolute;
    top: 10px;
    right: 10px;
    z-index: 2;
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    color: #fff;
    background: rgba(13, 16, 32, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.25);
    border-radius: 50%;
    cursor: pointer;
    backdrop-filter: blur(4px);
    transition: transform 0.3s var(--ease-spring), background-color 0.25s ease;
}

.game-fav:hover {
    transform: scale(1.12);
}

.game-fav.is-active {
    color: #fff;
    background: var(--pink);
    border-color: var(--pink);
    animation: heart-beat 0.5s var(--ease-spring);
}

.game-fav.is-active .icon path {
    fill: currentColor;
}

.game-body {
    padding: 14px 16px 16px;
}

.game-genre {
    font-size: 12px;
    font-weight: 600;
    color: var(--c1);
    letter-spacing: 1px;
    text-transform: uppercase;
}

.game-body h3 {
    margin: 4px 0 10px;
    font-size: 19px;
    color: var(--text);
}

.game-meta {
    display: flex;
    justify-content: space-between;
    font-size: 14px;
    color: var(--muted);
}

.game-rating {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    color: var(--yellow);
}

.game-rating .icon path {
    fill: currentColor;
}

.game-footer {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px dashed var(--line);
}

.game-price {
    font-family: var(--font-display);
    font-size: 20px;
    color: var(--text);
}

.game-price small {
    font-family: var(--font-body);
    font-size: 13px;
    color: var(--muted);
}

/* ---------- Избранное ---------- */

.favorites-head {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 16px;
}

.favorites-head h2 {
    display: flex;
    align-items: center;
    gap: 12px;
    margin: 0;
}

.count-pill {
    display: inline-grid;
    place-items: center;
    min-width: 32px;
    height: 32px;
    padding: 0 8px;
    font-family: var(--font-body);
    font-size: 15px;
    font-weight: 700;
    color: #fff;
    background: var(--pink);
    border-radius: 999px;
    animation: pop-in 0.4s var(--ease-spring);
}

.clear-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    font-family: inherit;
    font-size: 14px;
    color: var(--muted);
    background: transparent;
    border: 1px solid var(--line);
    border-radius: 8px;
    cursor: pointer;
    transition: color 0.25s ease, border-color 0.25s ease;
}

.clear-btn:hover {
    color: var(--red);
    border-color: var(--red);
}

.empty-heart {
    font-size: 26px;
    color: var(--pink);
    animation: heart-beat 1.6s ease-in-out infinite;
}

#favorites-list li:hover {
    border-color: var(--pink);
    transform: translateX(4px);
}

#favorites-list li.is-leaving {
    animation: fav-out 0.32s ease forwards;
}

.fav-title {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 10px;
}

.fav-heart {
    color: var(--pink);
}

.fav-heart path {
    fill: currentColor;
}

.fav-kind {
    padding: 2px 8px;
    font-size: 12px;
    color: var(--muted);
    background: rgba(255, 255, 255, 0.06);
    border-radius: 999px;
}

/* ---------- Акция ---------- */

.promo-shine {
    position: absolute;
    inset: 0;
    z-index: -1;
    overflow: hidden;
    border-radius: inherit;
}

.promo-shine::before {
    content: "";
    position: absolute;
    top: -50%;
    left: -30%;
    width: 30%;
    height: 200%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.08), transparent);
    transform: rotate(20deg);
    animation: shine 5s ease-in-out infinite;
}

.promo-row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 20px;
}

.promo-discount {
    margin-bottom: 16px;
    padding: 2px 14px;
    font-family: var(--font-display);
    font-size: 26px;
    color: var(--bg);
    background: linear-gradient(120deg, var(--yellow), #ff9f43);
    border-radius: 10px;
    transform: rotate(-4deg);
    animation: wobble 6s ease-in-out infinite;
}

.promo-card.is-urgent {
    border-color: var(--red);
    animation: urgent-glow 1s ease-in-out infinite alternate;
}

.promo-card.is-urgent #promo-timer {
    color: var(--red);
    animation: tick 0.5s ease;
}

.promo-card.is-finished {
    border-color: var(--line);
    filter: saturate(0.6);
}

.promo-card.is-granted {
    border-color: var(--green);
    box-shadow: 0 0 40px rgba(74, 222, 128, 0.25);
}

.promo-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
}

.promo-actions .promo-btn {
    margin-top: 0;
}

#promo-coupon strong {
    padding: 2px 8px;
    font-family: var(--font-display);
    letter-spacing: 2px;
    background: rgba(255, 255, 255, 0.4);
    border: 1px dashed rgba(13, 16, 32, 0.5);
    border-radius: 6px;
}

.copy-btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 14px;
    font-family: inherit;
    font-size: 14px;
    font-weight: 700;
    color: #fff;
    background: var(--bg);
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: transform 0.25s var(--ease-spring), background-color 0.25s ease;
}

.copy-btn:hover {
    transform: translateY(-2px);
}

.copy-btn.is-copied {
    background: #15803d;
}

.confetti {
    position: absolute;
    inset: 0;
    z-index: 5;
    overflow: hidden;
    border-radius: inherit;
    pointer-events: none;
}

.confetti span {
    position: absolute;
    top: -16px;
    animation-name: confetti-fall;
    animation-timing-function: cubic-bezier(0.25, 0.6, 0.45, 1);
    animation-fill-mode: both;
}

/* Конфетти на весь экран (код Konami) */
.confetti-screen {
    position: fixed;
    inset: 0;
    z-index: 130;
    pointer-events: none;
}

.confetti-screen .confetti {
    --fall: 110vh;
    border-radius: 0;
}

/* ---------- Отзывы ---------- */

.review-head {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 16px;
}

.review-head h2 {
    margin: 0;
}

.review-nav {
    display: flex;
    align-items: center;
    gap: 10px;
    animation: reveal-up 0.4s var(--ease-out);
}

.review-counter {
    min-width: 52px;
    font-size: 14px;
    text-align: center;
    color: var(--muted);
    font-variant-numeric: tabular-nums;
}

.review-placeholder {
    display: grid;
    place-items: center;
    min-height: 128px;
    color: var(--muted);
}

.review-slide--next {
    animation: slide-from-right 0.5s var(--ease-out);
}

.review-slide--prev {
    animation: slide-from-left 0.5s var(--ease-out);
}

.review-author-row {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 12px;
}

.review-author-row .review-author {
    margin: 0 0 2px;
}

.avatar {
    display: grid;
    flex-shrink: 0;
    place-items: center;
    width: 48px;
    height: 48px;
    font-family: var(--font-display);
    font-size: 20px;
    color: #fff;
    background: linear-gradient(135deg, hsl(var(--hue) 85% 55%), hsl(calc(var(--hue) + 60) 85% 45%));
    border-radius: 50%;
    box-shadow: 0 0 0 3px rgba(13, 16, 32, 1), 0 0 0 5px hsl(var(--hue) 85% 55% / 0.5);
    animation: pop-in 0.5s var(--ease-spring);
}

.stars {
    display: inline-flex;
    gap: 2px;
}

.star {
    color: var(--line);
    font-size: 17px;
}

.star.is-on {
    color: var(--yellow);
    text-shadow: 0 0 8px rgba(250, 204, 21, 0.6);
    animation: pop-in 0.4s var(--ease-spring) calc(var(--i) * 70ms + 0.15s) backwards;
}

.review-body {
    position: relative;
    padding-left: 28px;
}

.quote-mark {
    position: absolute;
    top: -14px;
    left: -2px;
    font-family: Georgia, serif;
    font-size: 54px;
    line-height: 1;
    color: var(--pink);
    opacity: 0.7;
}

.skeleton {
    display: grid;
    grid-template-columns: 48px 1fr;
    gap: 12px 14px;
    align-items: center;
}

.skeleton-circle,
.skeleton-line {
    background: linear-gradient(90deg, #1b2040 25%, #2c3566 50%, #1b2040 75%);
    background-size: 200% 100%;
    animation: shimmer 1.2s linear infinite;
}

.skeleton-circle {
    width: 48px;
    height: 48px;
    border-radius: 50%;
}

.skeleton-line {
    grid-column: 1 / -1;
    height: 12px;
    border-radius: 6px;
}

.skeleton-circle + .skeleton-line {
    grid-column: 2;
    width: 60%;
}

.skeleton-line.short {
    width: 45%;
}

.dots {
    display: flex;
    justify-content: center;
    gap: 8px;
    margin-bottom: 16px;
}

.dot {
    position: relative;
    width: 10px;
    height: 10px;
    padding: 0;
    overflow: hidden;
    background: var(--line);
    border: none;
    border-radius: 999px;
    cursor: pointer;
    transition: width 0.35s var(--ease-spring), background-color 0.25s ease;
}

.dot:hover {
    background: var(--muted);
}

.dot.is-active {
    width: 34px;
    background: rgba(0, 229, 255, 0.3);
}

.dot-progress {
    position: absolute;
    inset: 0;
    background: var(--cyan);
    transform-origin: left;
    animation: dot-fill linear forwards;
}

/* ---------- FAQ ---------- */

.faq-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.faq-item {
    background: var(--panel);
    border: 1px solid var(--line);
    border-radius: 14px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
}

.faq-item.is-open {
    border-color: rgba(0, 229, 255, 0.6);
    box-shadow: 0 0 24px rgba(0, 229, 255, 0.12);
}

.faq-question {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    padding: 18px 22px;
    font-family: inherit;
    font-size: 17px;
    font-weight: 600;
    text-align: left;
    color: var(--text);
    background: transparent;
    border: none;
    cursor: pointer;
}

.faq-question:hover {
    color: var(--cyan);
}

.faq-chevron {
    flex-shrink: 0;
    color: var(--cyan);
    transition: transform 0.4s var(--ease-spring);
}

.faq-item.is-open .faq-chevron {
    transform: rotate(180deg);
}

.faq-answer {
    display: grid;
    grid-template-rows: 0fr;
    transition: grid-template-rows 0.4s var(--ease-out);
}

.faq-item.is-open .faq-answer {
    grid-template-rows: 1fr;
}

.faq-answer > div {
    overflow: hidden;
}

.faq-answer p {
    margin: 0;
    padding: 0 22px 18px;
    color: var(--muted);
    opacity: 0;
    transform: translateY(-6px);
    transition: opacity 0.3s ease, transform 0.3s ease;
}

.faq-item.is-open .faq-answer p {
    opacity: 1;
    transform: none;
    transition-delay: 0.1s;
}

/* ---------- Форма заявки ---------- */

.form-section::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3px;
    background: linear-gradient(90deg, var(--cyan), var(--violet), var(--pink), var(--cyan));
    background-size: 300% 100%;
    animation: gradient-shift 4s linear infinite;
}

.form-subtitle {
    margin-bottom: 24px;
    text-align: center;
    color: var(--muted);
}

.field {
    position: relative;
}

form .field input[type="text"],
form .field input[type="email"],
form .field input[type="password"],
form .field input[type="tel"] {
    padding: 22px 16px 8px;
    background-color: rgba(13, 16, 32, 0.8);
}

.field label {
    position: absolute;
    top: 16px;
    left: 18px;
    font-size: 16px;
    color: #6b74a8;
    pointer-events: none;
    transform-origin: left top;
    transition: transform 0.25s var(--ease-out), color 0.25s ease;
}

.field input:focus ~ label,
.field input:not(:placeholder-shown) ~ label {
    color: var(--cyan);
    transform: translateY(-10px) scale(0.75);
}

.field-action {
    position: absolute;
    top: 50%;
    right: 10px;
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    color: var(--muted);
    background: transparent;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transform: translateY(-50%);
    transition: color 0.2s ease, background-color 0.2s ease;
}

.field:has(.field-error:not(:empty)) .field-action {
    top: 26px;
}

.field-action:hover {
    color: var(--cyan);
    background: rgba(0, 229, 255, 0.1);
}

.field input[name="password"] {
    padding-right: 52px;
}

.field-error {
    display: block;
    max-height: 0;
    overflow: hidden;
    padding-left: 4px;
    font-size: 13px;
    color: var(--red);
    opacity: 0;
    transition: max-height 0.3s ease, opacity 0.3s ease, margin 0.3s ease;
}

.field.has-error .field-error {
    max-height: 40px;
    margin-top: 6px;
    opacity: 1;
}

form .field.has-error input {
    border-color: var(--red);
}

.field.has-error label {
    color: var(--red);
}

.is-shaking .field.has-error,
.is-shaking .radio-group.has-error {
    animation: shake 0.45s ease;
}

.strength {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: -4px;
    animation: reveal-up 0.3s var(--ease-out);
}

.strength-bars {
    display: grid;
    flex: 1;
    grid-template-columns: repeat(4, 1fr);
    gap: 6px;
}

.strength-bars span {
    height: 5px;
    background: var(--line);
    border-radius: 3px;
    transition: background-color 0.35s ease, transform 0.35s var(--ease-spring);
}

.strength-bars span.is-on {
    transform: scaleY(1.4);
}

.strength--weak .is-on { background: var(--red); }
.strength--fair .is-on { background: #fb923c; }
.strength--good .is-on { background: var(--yellow); }
.strength--strong .is-on { background: var(--green); }

.strength-label {
    min-width: 76px;
    font-size: 13px;
    text-align: right;
    color: var(--muted);
}

.chips-group {
    flex-direction: row;
    flex-wrap: wrap;
}

.radio-group.has-error {
    border-color: var(--red);
}

.chips-group legend {
    width: auto;
}

.chips-group label {
    position: relative;
    flex: 1 1 auto;
}

.chips-group input[type="radio"] {
    position: absolute;
    opacity: 0;
    pointer-events: none;
}

.chips-group label > span {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 10px 14px;
    color: var(--text);
    background: rgba(13, 16, 32, 0.8);
    border: 1px solid var(--line);
    border-radius: 10px;
    transition: color 0.25s ease, background-color 0.25s ease,
                border-color 0.25s ease, transform 0.3s var(--ease-spring);
}

.chips-group label:hover > span {
    border-color: var(--pink);
    transform: translateY(-2px);
}

.chips-group input:checked + span {
    color: #fff;
    background: linear-gradient(135deg, var(--pink), var(--violet));
    border-color: transparent;
    box-shadow: 0 6px 20px rgba(255, 46, 136, 0.35);
    animation: bump 0.35s var(--ease-spring);
}

.chips-group input:focus-visible + span {
    outline: 2px solid var(--cyan);
    outline-offset: 2px;
}

/* ---------- О нас: история ---------- */

.timeline {
    position: relative;
    margin: 0;
    padding: 0 0 0 28px;
    list-style: none;
}

.timeline::before {
    content: "";
    position: absolute;
    top: 6px;
    bottom: 6px;
    left: 7px;
    width: 2px;
    background: linear-gradient(var(--cyan), var(--pink));
}

.timeline li {
    position: relative;
    padding-bottom: 20px;
}

.timeline li:last-child {
    padding-bottom: 0;
}

.timeline li::before {
    content: "";
    position: absolute;
    top: 6px;
    left: -30px;
    width: 14px;
    height: 14px;
    background: var(--bg);
    border: 3px solid var(--cyan);
    border-radius: 50%;
    box-shadow: 0 0 10px var(--cyan);
}

.timeline-year {
    font-family: var(--font-display);
    font-size: 20px;
    color: var(--cyan);
}

.timeline p {
    margin: 2px 0 0;
    color: var(--muted);
}

/* ---------- Модальное окно ---------- */

.modal-overlay.is-closing {
    animation: overlay-fade 0.22s ease reverse forwards;
}

.modal-overlay.is-closing .modal {
    animation: modal-out 0.22s ease forwards;
}

.modal-steps {
    list-style: none;
    counter-reset: step;
    padding-left: 0 !important;
}

.modal-steps li {
    position: relative;
    margin-bottom: 10px;
    padding-left: 40px;
    counter-increment: step;
    animation: reveal-left 0.4s var(--ease-out) backwards;
}

.modal-steps li:nth-child(2) { animation-delay: 0.08s; }
.modal-steps li:nth-child(3) { animation-delay: 0.16s; }

.modal-steps li::before {
    content: counter(step);
    position: absolute;
    top: 0;
    left: 0;
    display: grid;
    place-items: center;
    width: 26px;
    height: 26px;
    font-size: 13px;
    font-weight: 700;
    color: var(--bg);
    background: var(--cyan);
    border-radius: 50%;
}

/* ---------- Уведомления ---------- */

.toasts {
    position: fixed;
    top: 88px;
    right: 20px;
    z-index: 120;
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: min(360px, calc(100vw - 40px));
    pointer-events: none;
}

.toast {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    overflow: hidden;
    font-size: 15px;
    background: rgba(23, 27, 49, 0.92);
    border: 1px solid var(--line);
    border-left: 4px solid var(--cyan);
    border-radius: 12px;
    box-shadow: 0 14px 34px rgba(0, 0, 0, 0.45);
    backdrop-filter: blur(10px);
    cursor: pointer;
    pointer-events: auto;
    animation: toast-in 0.45s var(--ease-spring);
}

.toast.is-leaving {
    animation: toast-out 0.3s ease forwards;
}

.toast-icon {
    display: grid;
    flex-shrink: 0;
    place-items: center;
    width: 28px;
    height: 28px;
    color: var(--bg);
    background: var(--cyan);
    border-radius: 50%;
}

.toast--success { border-left-color: var(--green); }
.toast--success .toast-icon { background: var(--green); }
.toast--error { border-left-color: var(--red); }
.toast--error .toast-icon { background: var(--red); }
.toast--fav { border-left-color: var(--pink); }
.toast--fav .toast-icon { color: #fff; background: var(--pink); }

.toast-timer {
    position: absolute;
    left: 0;
    bottom: 0;
    width: 100%;
    height: 2px;
    background: currentColor;
    opacity: 0.35;
    transform-origin: left;
    animation: dot-fill 3.2s linear reverse forwards;
}

/* ---------- Подвал ---------- */

footer::before {
    content: "";
    position: absolute;
    top: -1px;
    left: 10%;
    right: 10%;
    height: 1px;
    background: linear-gradient(90deg, transparent, var(--cyan), var(--pink), transparent);
}

.footer-brand {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    margin-bottom: 12px;
}

.footer-brand .gradient-text {
    font-family: var(--font-display);
    font-size: 24px;
}

.footer-tag {
    color: var(--muted);
    letter-spacing: 2px;
    text-transform: uppercase;
}

footer p {
    margin: 0 0 6px;
}

.footer-secret {
    margin-top: 4px;
    padding: 6px 12px;
    font-family: inherit;
    font-size: 11px;
    letter-spacing: 3px;
    color: inherit;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 999px;
    cursor: pointer;
    opacity: 0.35;
    transition: opacity 0.3s ease, color 0.3s ease, border-color 0.3s ease,
                transform 0.3s var(--ease-spring);
}

.footer-secret:hover,
.footer-secret:focus-visible {
    color: var(--cyan);
    border-color: rgba(0, 229, 255, 0.4);
    opacity: 1;
    transform: scale(1.05);
}

.footer-secret:active {
    transform: scale(0.95);
}

/* ---------- Пасхалка: режим вечеринки ---------- */

body.party-mode .bg-fx {
    animation: hue-cycle 3s linear infinite;
}

body.party-mode .bg-grid {
    animation-duration: 1.2s;
}

body.party-mode .brand-logo {
    animation: spin 1.4s linear infinite;
}

/* ---------- Адаптивность ---------- */

@media (max-width: 1000px) {
    .burger {
        display: flex;
    }

    .nav {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        flex-direction: column;
        gap: 4px;
        padding: 0 20px;
        max-height: 0;
        overflow: hidden;
        background: #0d1020;
        box-shadow: 0 24px 40px rgba(0, 0, 0, 0.5);
        border-bottom: 1px solid transparent;
        transition: max-height 0.4s var(--ease-out), padding 0.4s var(--ease-out),
                    border-color 0.3s ease;
    }

    header.menu-open .nav {
        max-height: 360px;
        padding: 12px 20px 18px;
        border-bottom-color: rgba(0, 229, 255, 0.35);
    }

    .nav a {
        padding: 12px 14px;
        opacity: 0;
        transform: translateX(-12px);
        transition: opacity 0.3s ease, transform 0.3s ease, color 0.25s ease,
                    background-color 0.25s ease;
    }

    header.menu-open .nav a {
        opacity: 1;
        transform: none;
    }

    header.menu-open .nav a:nth-of-type(2) { transition-delay: 0.04s; }
    header.menu-open .nav a:nth-of-type(3) { transition-delay: 0.08s; }
    header.menu-open .nav a:nth-of-type(4) { transition-delay: 0.12s; }
    header.menu-open .nav a:nth-of-type(5) { transition-delay: 0.16s; }

    .nav a.active,
    .nav a.active:hover {
        background-color: var(--cyan);
    }

    .nav-indicator {
        display: none;
    }

    .stats {
        grid-template-columns: repeat(2, 1fr);
    }
}

@media (max-width: 560px) {
    body {
        padding-top: 70px;
    }

    header {
        flex-direction: row;
        align-items: center;
        padding: 12px 16px;
    }

    header h1 {
        font-size: 24px;
    }

    .brand-logo {
        width: 36px;
        height: 36px;
    }

    #live-clock {
        display: none;
    }

    main {
        padding: 0 16px 48px;
    }

    .hero {
        padding: 44px 18px 64px;
    }

    .hero p {
        min-height: 3em;
    }

    .hero-actions .btn {
        width: 100%;
    }

    .features {
        grid-template-columns: 1fr;
    }

    .favorites,
    .review,
    .form-section {
        padding: 22px 18px;
    }

    .promo-card {
        padding: 32px 20px;
    }

    .badge {
        right: 8px;
    }

    .stat-value {
        font-size: 26px;
    }

    .to-top {
        right: 16px;
        bottom: 16px;
    }
}

@media (max-width: 400px) {
    h1 {
        font-size: 30px;
    }

    .hero p {
        font-size: 23px;
    }

    .nav {
        gap: 4px;
    }

    .stats {
        grid-template-columns: 1fr 1fr;
        gap: 10px;
    }
}

@media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
        scroll-behavior: auto !important;
    }

    .bg-grid,
    .hero-glow {
        animation: none !important;
    }
}

/* ---------- Ключевые кадры ---------- */

@keyframes orb-float {
    0% { transform: translate(0, 0) scale(1); }
    50% { transform: translate(6vw, 4vh) scale(1.12); }
    100% { transform: translate(-4vw, 8vh) scale(0.95); }
}

@keyframes grid-move {
    from { background-position: 0 0; }
    to { background-position: 0 64px; }
}

@keyframes header-in {
    from { opacity: 0; transform: translateY(-100%); }
    to { opacity: 1; transform: none; }
}

@keyframes page-in {
    from { opacity: 0; transform: translateY(16px); }
    to { opacity: 1; transform: none; }
}

@keyframes reveal-up {
    from { opacity: 0; transform: translateY(32px); }
    to { opacity: 1; transform: none; }
}

@keyframes reveal-zoom {
    from { opacity: 0; transform: scale(0.92); }
    to { opacity: 1; transform: none; }
}

@keyframes reveal-left {
    from { opacity: 0; transform: translateX(-28px); }
    to { opacity: 1; transform: none; }
}

@keyframes card-in {
    from { opacity: 0; transform: translateY(24px) scale(0.96); }
    to { opacity: 1; transform: none; }
}

@keyframes gradient-shift {
    from { background-position: 0% 50%; }
    to { background-position: 300% 50%; }
}

@keyframes glow-pulse {
    0%, 100% { box-shadow: 0 0 14px rgba(0, 229, 255, 0.35); }
    50% { box-shadow: 0 0 28px rgba(255, 46, 136, 0.55); }
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

@keyframes pulse {
    from { opacity: 0.4; }
    to { opacity: 1; }
}

@keyframes ping {
    0% { transform: scale(1); opacity: 0.8; }
    100% { transform: scale(3); opacity: 0; }
}

@keyframes blink {
    50% { opacity: 0; }
}

@keyframes tick {
    0% { transform: translateY(-30%); opacity: 0.3; }
    100% { transform: none; opacity: 1; }
}

@keyframes scroll-wheel {
    0% { transform: translateY(0); opacity: 1; }
    80% { transform: translateY(12px); opacity: 0; }
    100% { transform: translateY(0); opacity: 0; }
}

@keyframes bounce-up {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-4px); }
}

@keyframes bump {
    0% { transform: scale(1); }
    40% { transform: scale(1.25); }
    100% { transform: scale(1); }
}

@keyframes pop-in {
    from { opacity: 0; transform: scale(0.4); }
    to { opacity: 1; transform: scale(1); }
}

@keyframes heart-beat {
    0%, 100% { transform: scale(1); }
    30% { transform: scale(1.3); }
    60% { transform: scale(0.95); }
}

@keyframes wobble {
    0%, 100% { transform: rotate(-3deg); }
    50% { transform: rotate(-1deg); }
}

@keyframes fav-in {
    from { opacity: 0; transform: translateX(-30px) scale(0.95); }
    to { opacity: 1; transform: none; }
}

@keyframes fav-out {
    0% { opacity: 1; transform: none; }
    60% { opacity: 0; transform: translateX(40px); max-height: 90px; }
    100% { opacity: 0; transform: translateX(40px); max-height: 0; padding-top: 0; padding-bottom: 0; margin-top: -12px; border-width: 0; }
}

@keyframes shine {
    0%, 60% { left: -30%; }
    100% { left: 130%; }
}

@keyframes badge-float {
    0%, 100% { transform: translateY(0) rotate(0deg); }
    50% { transform: translateY(-4px) rotate(3deg); }
}

@keyframes urgent-glow {
    from { box-shadow: 0 0 10px rgba(248, 113, 113, 0.2); }
    to { box-shadow: 0 0 36px rgba(248, 113, 113, 0.55); }
}

@keyframes coupon-in {
    from { opacity: 0; transform: translateY(16px) scale(0.9); }
    to { opacity: 1; transform: none; }
}

@keyframes confetti-fall {
    0% { transform: translate(0, 0) rotate(0deg); opacity: 1; }
    100% { transform: translate(var(--drift), var(--fall, 420px)) rotate(var(--rotate)); opacity: 0; }
}

@keyframes slide-from-right {
    from { opacity: 0; transform: translateX(40px); }
    to { opacity: 1; transform: none; }
}

@keyframes slide-from-left {
    from { opacity: 0; transform: translateX(-40px); }
    to { opacity: 1; transform: none; }
}

@keyframes shimmer {
    from { background-position: 200% 0; }
    to { background-position: -200% 0; }
}

@keyframes dot-fill {
    from { transform: scaleX(0); }
    to { transform: scaleX(1); }
}

@keyframes shake {
    0%, 100% { transform: translateX(0); }
    20% { transform: translateX(-8px); }
    40% { transform: translateX(7px); }
    60% { transform: translateX(-5px); }
    80% { transform: translateX(3px); }
}

@keyframes modal-out {
    to { opacity: 0; transform: translateY(18px) scale(0.95); }
}

@keyframes toast-in {
    from { opacity: 0; transform: translateX(110%); }
    to { opacity: 1; transform: none; }
}

@keyframes toast-out {
    to { opacity: 0; transform: translateX(110%); margin-top: -60px; }
}

@keyframes hue-cycle {
    to { filter: hue-rotate(360deg); }
}

@keyframes glitch-a {
    0% { clip-path: inset(10% 0 60% 0); transform: translate(-3px, -1px); }
    50% { clip-path: inset(55% 0 15% 0); transform: translate(3px, 1px); }
    100% { clip-path: inset(30% 0 40% 0); transform: translate(-2px, 0); }
}

@keyframes glitch-b {
    0% { clip-path: inset(60% 0 5% 0); transform: translate(3px, 1px); }
    50% { clip-path: inset(5% 0 70% 0); transform: translate(-3px, -1px); }
    100% { clip-path: inset(40% 0 30% 0); transform: translate(2px, 0); }
}
```

Рисунок 8 - Результат выполнения программы task_itog.jsx, task_itog.css

На рис. 8 показано итоговое приложение GameZone с рабочим меню и отдельной страницей «О нас». Маршрутизация не требует переписывать макет: `Header` вынесен наружу `<Routes>`, страницы и секции сохраняют общее состояние приложения, а переходы не перезагружают документ. Вариант Б оставляет каталог и форму на главной, но использует реальные URL и историю браузера; вариант А в `task1.jsx` дополнительно закрывает пункты чек-листа с отдельным каталогом, страницей товара, вложенными вкладками и формой на странице «О сайте». В обоих вариантах состояние React сохраняется при навигации, а несуществующие адреса обрабатываются страницей 404.

## Заключение

В ходе лабораторной работы изучена клиентская маршрутизация React-приложения. `BrowserRouter` связал приложение с History API, `Routes` и `Route` сопоставили URL компонентам, `Link` и `NavLink` обеспечили переходы без перезагрузки и подсветку активного пункта, `useParams` позволил читать динамические параметры, `useNavigate` – выполнять программные переходы, а `Outlet` – вкладывать маршруты и переключать вкладки. Отдельно отработана страница 404 через `path="*"`.

Сравнение с ЛР4 показало разницу между переключением экранов через `useState` и настоящими маршрутами: в первом случае URL не меняется и история не сохраняется, во втором меняется адрес, работают кнопки «назад/вперёд», прямые ссылки и закладки. В итоговом задании вариант А реализован в `task1.jsx`, вариант Б – в `task_itog.jsx`; оба варианта используют одни и те же возможности `react-router-dom`, но по-разному организуют страницы и секции сайта.
