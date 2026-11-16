# Лабораторная работа №5

React Router. Вариант 04. Отчёт по проделанной работе.

В работе изучена маршрутизация React-приложения средствами `react-router-dom`: `BrowserRouter`, `Routes`, `Route`, `Link`, `NavLink`, `useParams`, `useNavigate`, `useLocation` и `Outlet`. Проект собран на React 19 и Vite; тема варианта – спортивный клуб «Атлет», итоговое приложение оформлено в стиле игрового клуба GameZone.

Итоговое задание выполнено в двух вариантах. Файл `src/task1.jsx` реализует вариант А – структуру и названия страниц строго по чек-листу; после каждой темы в нём добавлен собственный компонент `AppTopic1`–`AppTopic7`, а итоговый `App` возвращает `AppTopic7`. Файл `src/task_itog.jsx` реализует вариант Б – собственную структуру сайта GameZone, где часть разделов вынесена в отдельные маршруты, а часть работает как секции главной страницы. Основным запускаемым вариантом является А: именно `task1.jsx` импортируется в `main.jsx`.

## Задание 1. Установка и базовая настройка (BrowserRouter, Routes, Route)

Изучена одноразовая настройка роутера и базовое сопоставление адресов. В `main.jsx` приложение обёрнуто в `BrowserRouter` из `react-router-dom`, поэтому все дочерние компоненты получают контекст роутера. В `task1.jsx` созданы компоненты `HomePage` с заголовком «Спортивный клуб «Атлет»» и `AboutPage` с заголовком «О сайте». Они подключены через `<Routes>` и `<Route>` с путями `/` и `/about`; маршруты темы собраны в `AppTopic1`.

В TODO 4 разобран сломанный `App`. В нём два соседних `<Route>` нарушают правило JSX об одном корневом элементе, а отсутствие `<Routes>` не даёт роутеру выбрать подходящий маршрут. Исправленный `AppTopic1` оборачивает оба `<Route>` в `<Routes>`.

На листинге 1 показано: подключение `BrowserRouter` в точке входа приложения.

**Листинг 1 - Задание №1 (main.jsx)**

```jsx
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import './index.css'
import App from './task1.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>,
)
```

На листинге 2 показано: импорты `react-router-dom`, данные каталога, компоненты `HomePage` и `AboutPage` с комментариями TODO и маршруты `/` и `/about` в `AppTopic1`.

**Листинг 2 - Задание №1 (task1.jsx)**

```jsx
// TODO 1. <App /> обёрнут в <BrowserRouter> в main.jsx.
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

// Данные каталога.
const PRODUCTS = [
  { id: '1', title: 'Абонемент на месяц', price: 2500 },
  { id: '2', title: 'Разовое занятие', price: 500 },
  { id: '3', title: 'Персональная тренировка', price: 1500 },
]

// ============================================================
// ТЕМА 1. BrowserRouter, Routes, Route
// ============================================================

// TODO 2. HomePage на path="/".
// TODO 6. Ссылка «О сайте» на главную.
// TODO 8. Ошибка: <a href> перезагружает страницу, нужен <Link>.
function HomePage() {
  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>
      <Link to="/about">О сайте</Link>
    </div>
  )
}

// TODO 3. AboutPage на path="/about".
// TODO 7. Ссылка «На главную» на странице «О сайте».
function AboutPage() {
  // TODO 22. Controlled-форма и navigate('/') после отправки.
  const [email, setEmail] = useState('')
  const navigate = useNavigate()

  function handleSubmit(event) {
    event.preventDefault()
    navigate('/')
  }

  return (
    <div>
      <h1>О сайте</h1>
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

// TODO 4. Исправленный App: оба <Route> внутри общего <Routes>.
// TODO 5. Вопрос: BrowserRouter против useState — ответ в отчёте.
function AppTopic1() {
  return (
    <main>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/about" element={<AboutPage />} />
      </Routes>
    </main>
  )
}
```

Рисунок 1 - Результат выполнения программы main.jsx, task1.jsx

На рис. 1 показан результат запуска приложения с двумя базовыми маршрутами. Переход через `BrowserRouter` меняет реальный URL в адресной строке через History API и сохраняет историю браузера, поэтому работают кнопки «назад/вперёд», прямые ссылки и закладки. Переключение экранов через `useState`, которое применялось в ЛР4, меняло только отрисованный компонент: URL оставался прежним, а история и закладки на конкретный экран не появлялись. Именно поэтому в ЛР5 экраны заменены на маршруты: `BrowserRouter` хранит адрес в истории браузера, а `useState` – только внутри компонента (TODO 5).

## Задание 2. Link – навигация без перезагрузки

Изучены клиентские ссылки `Link`. На `HomePage` добавлена ссылка `<Link to="/about">О сайте</Link>`, на `AboutPage` – `<Link to="/">На главную</Link>`. Компонент `Header` содержит три ссылки на `/`, `/catalog` и `/about` с текстами «Расписание», «Тренеры» и «Цены». Для пути `/catalog` создана заглушка `CatalogPage`: до темы 4 она показывает только текст «Список услуг появится позже.», а список товаров включает позже через проп `withProducts`. Компонент `<Header />` размещён в `AppTopic2` снаружи `<Routes>`, поэтому меню видно на любой странице и не пропадает при переходе. Ссылки меню сразу выполнены на `NavLink`, потому что в теме 3 их нужно подсвечивать.

В TODO 8 показана ошибка: обычный `<a href="/about">` выполняет полноценный запрос к серверу, перезагружает документ и теряет состояние React. Исправление – заменить его на `<Link to="/about">`.

На листинге 3 показано: меню `Header`, каталог `CatalogPage` с заглушкой и маршруты `AppTopic2`.

**Листинг 3 - Задание №2 (task1.jsx)**

```jsx
// TODO 9. Header снаружи <Routes>, меню на всех страницах.
// TODO 10. Вопрос: Link против <a> — ответ в отчёте.
function Header() {
  // TODO 11. Все ссылки — NavLink.
  // TODO 12. Правило .active — в task1.css.
  // TODO 13. Проверка подсветки — описание в отчёте.
  // TODO 14. Функция className и стрелка «→» у ссылки «Цены».
  // TODO 15. Вопрос: NavLink без CSS — ответ в отчёте.
  return (
    <header className="site-header">
      <nav className="menu">
        <NavLink to="/" end>
          Расписание
        </NavLink>
        <NavLink to="/catalog">Тренеры</NavLink>
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

// TODO 18. Список товаров через .map() и key; до темы 4 — заглушка.
function CatalogPage({ withProducts = false }) {
  return (
    <div>
      <h1>Каталог</h1>

      {withProducts ? (
        <ul className="product-list">
          {PRODUCTS.map((product) => (
            <li key={product.id}>
              <Link to={`/product/${product.id}`}>
                {product.title} — {product.price} ₽
              </Link>
            </li>
          ))}
        </ul>
      ) : (
        <p>Список услуг появится позже.</p>
      )}
    </div>
  )
}

// Маршруты к концу темы 2.
function AppTopic2() {
  return (
    <>
      <Header />
      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/catalog" element={<CatalogPage />} />
        </Routes>
      </main>
    </>
  )
}
```

Рисунок 2 - Результат выполнения программы task1.jsx

На рис. 2 показано меню, которое остаётся на месте при переходе между страницами. Ссылка `<a href>` перезагружает документ: React-приложение размонтируется, всё состояние и контекст теряются. `Link` перехватывает клик, меняет URL через History API и просит роутер отрисовать нужный маршрут без перезагрузки, поэтому состояние приложения сохраняется. Вынос `Header` за пределы `<Routes>` обеспечивает постоянное меню на всех страницах. Ответ на TODO 10: `Link` выполняет клиентский переход без перезагрузки, а `<a href>` запрашивает документ заново и теряет состояние приложения.

## Задание 3. NavLink – подсветка активной ссылки

Изучен `NavLink`, который автоматически определяет активность ссылки. В `Header` все три ссылки выполнены через `NavLink`; для главной добавлен проп `end`, чтобы она не считалась активной на вложенных адресах. Для ссылки на `/about` использована функция в `className`: `({ isActive }) => (isActive ? 'active about-link' : 'about-link')`. Дополнительно применена children-функция, которая при активности выводит перед текстом «Цены» дополнительную стрелку. Маршруты темы не изменились, поэтому `AppTopic3` возвращает `AppTopic2`.

В `task1.css` добавлено правило `.menu a.active`, которое задаёт цвет, жирность и подчёркивание активного пункта.

На листинге 4 показано: меню `Header` на `NavLink` с функцией `className` и children-функцией для стрелки.

**Листинг 4 - Задание №3 (task1.jsx)**

```jsx
function Header() {
  // TODO 11. Все ссылки — NavLink.
  // TODO 12. Правило .active — в task1.css.
  // TODO 13. Проверка подсветки — описание в отчёте.
  // TODO 14. Функция className и стрелка «→» у ссылки «Цены».
  // TODO 15. Вопрос: NavLink без CSS — ответ в отчёте.
  return (
    <header className="site-header">
      <nav className="menu">
        <NavLink to="/" end>
          Расписание
        </NavLink>
        <NavLink to="/catalog">Тренеры</NavLink>
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

На рис. 3 показана подсветка активного пункта меню. При переходе подсветка автоматически переезжает на выбранный пункт, а остальные пункты возвращаются к обычному виду (TODO 13). У ссылки «Цены» в активном состоянии дополнительно появляется стрелка. `NavLink` действительно добавляет класс `active`, но без CSS-правила для `.active` этот класс не влияет на внешний вид, поэтому кажется, что подсветка не работает (TODO 15). Правило в `task1.css` делает состояние видимым.

## Задание 4. useParams – динамические маршруты

Изучены динамические маршруты и хук `useParams`. В `AppTopic4` добавлен маршрут `path="/product/:id"` с компонентом `ProductPage`. В `CatalogPage` на этом этапе включён проп `withProducts`, поэтому выводится список из трёх товаров через `PRODUCTS.map()` с `key`; каждая карточка является ссылкой `<Link to={`/product/${product.id}`}>`. В `ProductPage` параметр `id` получен через `useParams()`, товар найден через `PRODUCTS.find((item) => item.id === String(id))`. Если товара нет, выводится текст «Товар не найден».

На листинге 6 показано: данные товаров, каталог с пропом `withProducts`, страница `ProductPage` с `useParams` и маршруты `AppTopic4`.

**Листинг 6 - Задание №4 (task1.jsx)**

```jsx
// Данные каталога.
const PRODUCTS = [
  { id: '1', title: 'Абонемент на месяц', price: 2500 },
  { id: '2', title: 'Разовое занятие', price: 500 },
  { id: '3', title: 'Персональная тренировка', price: 1500 },
]

// ...

// TODO 18. Список товаров через .map() и key; до темы 4 — заглушка.
function CatalogPage({ withProducts = false }) {
  return (
    <div>
      <h1>Каталог</h1>

      {withProducts ? (
        <ul className="product-list">
          {PRODUCTS.map((product) => (
            <li key={product.id}>
              <Link to={`/product/${product.id}`}>
                {product.title} — {product.price} ₽
              </Link>
            </li>
          ))}
        </ul>
      ) : (
        <p>Список услуг появится позже.</p>
      )}
    </div>
  )
}

// ...

// TODO 20. Вопрос: почему параметры всегда строки — ответ в отчёте.
// TODO 29. Вопрос: забытый Outlet — ответ в отчёте.
// withTabs подключает вкладки товара из темы 6.
function ProductPage({ withTabs = false }) {
  // TODO 17. Чтение id из адреса.
  const { id } = useParams()
  const navigate = useNavigate()

  // TODO 19. Поиск товара по id; иначе «Товар не найден».
  const product = PRODUCTS.find((item) => item.id === String(id))

  if (!product) {
    return <h1>Товар не найден</h1>
  }

  // TODO 21. Кнопка «Назад к каталогу» через navigate(-1).
  // TODO 27. Вкладки «Описание»/«Отзывы» и <Outlet />.
  return (
    <div>
      <h1>{product.title}</h1>
      <p>{product.price} ₽</p>

      <button type="button" onClick={() => navigate(-1)}>
        Назад к каталогу
      </button>

      {withTabs && (
        <>
          <nav className="tabs">
            <Link to="">Описание</Link>
            <Link to="reviews">Отзывы</Link>
          </nav>
          <Outlet />
        </>
      )}
    </div>
  )
}

// ...

function AppTopic4() {
  // TODO 16. Маршрут path="/product/:id".
  return (
    <>
      <Header />
      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/catalog" element={<CatalogPage withProducts />} />
          <Route path="/product/:id" element={<ProductPage />} />
        </Routes>
      </main>
    </>
  )
}
```

Рисунок 4 - Результат выполнения программы task1.jsx

На рис. 4 показана страница конкретного товара, открытая по адресу `/product/1`. `useParams()` возвращает значения строками, потому что параметр – это часть URL, то есть текст; роутер не знает, что в конкретном проекте `:id` означает число. Если забыть приведение типа, сравнение `"1" === 1` даст `false`, и страница товара с существующим `id` покажет «Товар не найден». Поэтому в коде используется `String(id)`. Это ответ на TODO 20: параметр маршрута всегда строка, потому что он извлекается из URL.

## Задание 5. useNavigate – переход по коду

Изучен программный переход через `useNavigate`. В `ProductPage` добавлена кнопка «Назад к каталогу» с `onClick={() => navigate(-1)}`. На `AboutPage` создана controlled-форма с полем «Ваш email»: значение хранится в `useState`, изменяется через `onChange`, а в `handleSubmit` вызываются `event.preventDefault()` и `navigate("/")`. В TODO 23 исправлен `BrokenRedirect`: вызов `navigate("/")` перенесён из тела компонента в `useEffect`, чтобы переход выполнялся как побочный эффект, а не при каждом рендере. Маршруты темы не изменились, поэтому `AppTopic5` возвращает `AppTopic4`.

На листинге 7 показано: форма на `AboutPage`, страница `ProductPage` с кнопкой возврата, исправленный `BrokenRedirect` и `AppTopic5`.

**Листинг 7 - Задание №5 (task1.jsx)**

```jsx
// TODO 3. AboutPage на path="/about".
// TODO 7. Ссылка «На главную» на странице «О сайте».
function AboutPage() {
  // TODO 22. Controlled-форма и navigate('/') после отправки.
  const [email, setEmail] = useState('')
  const navigate = useNavigate()

  function handleSubmit(event) {
    event.preventDefault()
    navigate('/')
  }

  return (
    <div>
      <h1>О сайте</h1>
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

// TODO 20. Вопрос: почему параметры всегда строки — ответ в отчёте.
// TODO 29. Вопрос: забытый Outlet — ответ в отчёте.
// withTabs подключает вкладки товара из темы 6.
function ProductPage({ withTabs = false }) {
  // TODO 17. Чтение id из адреса.
  const { id } = useParams()
  const navigate = useNavigate()

  // TODO 19. Поиск товара по id; иначе «Товар не найден».
  const product = PRODUCTS.find((item) => item.id === String(id))

  if (!product) {
    return <h1>Товар не найден</h1>
  }

  // TODO 21. Кнопка «Назад к каталогу» через navigate(-1).
  // TODO 27. Вкладки «Описание»/«Отзывы» и <Outlet />.
  return (
    <div>
      <h1>{product.title}</h1>
      <p>{product.price} ₽</p>

      <button type="button" onClick={() => navigate(-1)}>
        Назад к каталогу
      </button>

      {withTabs && (
        <>
          <nav className="tabs">
            <Link to="">Описание</Link>
            <Link to="reviews">Отзывы</Link>
          </nav>
          <Outlet />
        </>
      )}
    </div>
  )
}

// ...

// TODO 23. Ошибка: navigate в теле компонента — перенос в useEffect.
// TODO 24. Вопрос: navigate('/path') против navigate(-1) — ответ в отчёте.
function BrokenRedirect() {
  const navigate = useNavigate()

  useEffect(() => {
    navigate('/')
  }, [navigate])

  return <p>Секунду...</p>
}

// Маршруты те же, изменилось поведение компонентов.
function AppTopic5() {
  return <AppTopic4 />
}
```

Рисунок 5 - Результат выполнения программы task1.jsx

На рис. 5 показан переход на главную страницу после отправки формы. `navigate("/path")` добавляет новую запись в историю и ведёт на конкретную страницу; это уместно после отправки формы, например `navigate("/")`. `navigate(-1)` возвращает на предыдущую запись истории, как кнопка «назад» браузера; это уместно для кнопки «Назад к каталогу», чтобы вернуть пользователя туда, откуда он пришёл. Вызывать `navigate` в теле компонента нельзя: это побочный эффект, который срабатывает на каждом рендере и приводит к циклу переходов. Это ответ на TODO 24.

## Задание 6. Вложенные маршруты и Outlet

Изучены вложенные маршруты и компонент `Outlet`. Созданы `ProductDetails` с текстом «Описание товара» и `ProductReviews`, который выводит список отзывов-заглушек через `.map()` с `key`. Оба компонента вложены в маршрут `/product/:id`: `ProductDetails` подключён как `index`-маршрут, `ProductReviews` – как `path="reviews"` без слэша в начале. В `ProductPage` добавлены `<Outlet />` и две ссылки-вкладки: `<Link to="">Описание</Link>` и `<Link to="reviews">Отзывы</Link>`. Вкладки включаются только при `withTabs`, поэтому `AppTopic4` остаётся без них, а `AppTopic6` передаёт `<ProductPage withTabs />`.

На листинге 8 показано: страница `ProductPage` с `Outlet` и вкладками, компоненты `ProductDetails` и `ProductReviews`, а также вложенные маршруты `AppTopic6`.

**Листинг 8 - Задание №6 (task1.jsx)**

```jsx
// TODO 20. Вопрос: почему параметры всегда строки — ответ в отчёте.
// TODO 29. Вопрос: забытый Outlet — ответ в отчёте.
// withTabs подключает вкладки товара из темы 6.
function ProductPage({ withTabs = false }) {
  // TODO 17. Чтение id из адреса.
  const { id } = useParams()
  const navigate = useNavigate()

  // TODO 19. Поиск товара по id; иначе «Товар не найден».
  const product = PRODUCTS.find((item) => item.id === String(id))

  if (!product) {
    return <h1>Товар не найден</h1>
  }

  // TODO 21. Кнопка «Назад к каталогу» через navigate(-1).
  // TODO 27. Вкладки «Описание»/«Отзывы» и <Outlet />.
  return (
    <div>
      <h1>{product.title}</h1>
      <p>{product.price} ₽</p>

      <button type="button" onClick={() => navigate(-1)}>
        Назад к каталогу
      </button>

      {withTabs && (
        <>
          <nav className="tabs">
            <Link to="">Описание</Link>
            <Link to="reviews">Отзывы</Link>
          </nav>
          <Outlet />
        </>
      )}
    </div>
  )
}

// ...

// TODO 25. Компоненты вкладок.
function ProductDetails() {
  return <p>Описание товара</p>
}

function ProductReviews() {
  // TODO 28. Отзывы-заглушки через .map() с key.
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

function AppTopic6() {
  // TODO 26. Вложенные index и path="reviews" в /product/:id.
  return (
    <>
      <Header />
      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/catalog" element={<CatalogPage withProducts />} />
          <Route path="/product/:id" element={<ProductPage withTabs />}>
            <Route index element={<ProductDetails />} />
            <Route path="reviews" element={<ProductReviews />} />
          </Route>
        </Routes>
      </main>
    </>
  )
}
```

Рисунок 6 - Результат выполнения программы task1.jsx

На рис. 6 показаны вкладки «Описание» и «Отзывы», которые переключаются без перезагрузки страницы. Если вложенные маршруты настроены, но `<Outlet />` в родительском компоненте забыт, роутер будет считать их активными и менять URL, однако элементы не отрисуются: родителю некуда вставить дочерний маршрут. Пользователь увидит страницу товара без вкладки «Описание» или «Отзывы», хотя адрес будет меняться. `Outlet` – обязательная точка вставки вложенного содержимого. Это ответ на TODO 29.

## Задание 7. Страница 404

Изучена обработка несуществующих адресов. Создан `NotFoundPage` с текстом «404 – страница не найдена» и ссылкой `<Link to="/">Вернуться на главную</Link>`. В конец `<Routes>` в `AppTopic7` добавлен маршрут `path="*"` с элементом `<NotFoundPage />`; звёздочка ловит любой адрес, который не подошёл маршрутам выше.

На листинге 9 показано: компонент `NotFoundPage` и маршрут-ловушка `path="*"`.

**Листинг 9 - Задание №7 (task1.jsx)**

```jsx
// TODO 30. NotFoundPage со ссылкой на главную.
// TODO 32. Проверка несуществующего адреса — описание в отчёте.
function NotFoundPage() {
  return (
    <div>
      <h1>404 — страница не найдена</h1>
      <Link to="/">Вернуться на главную</Link>
    </div>
  )
}

// ...

function AppTopic7() {
  // TODO 31. Маршрут path="*" последним в Routes.
  return (
    <>
      <Header />
      <main>
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/about" element={<AboutPage />} />
          <Route path="/catalog" element={<CatalogPage withProducts />} />
          <Route path="/product/:id" element={<ProductPage withTabs />}>
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

Рисунок 7 - Результат выполнения программы task1.jsx

На рис. 7 показана страница 404 для несуществующего адреса. До добавления `path="*"` ни один маршрут не совпадал с таким адресом, и `<Routes>` ничего не рендерил: на экране оставалась пустая область, а снаружи мог отображаться только `Header`. После добавления `path="*"` на адрес вроде `/qwerty` показывается `NotFoundPage`, а не пустая страница. Это ответ на TODO 32.

## Итоговое задание

Итоговое приложение варианта А собрано в `task1.jsx`: после каждой темы добавлен собственный компонент `AppTopic1`–`AppTopic7`, маршруты накапливаются от темы к теме, а итоговый `App` возвращает `AppTopic7`. Сайт GameZone варианта Б реализован в `task_itog.jsx` и использует маршруты `/` и `/about`, а также ловушку `*`; разделы «Услуги», «Игры» и «Контакты» открываются как секции главной через хэш-адреса `/#features`, `/#catalog` и `/#contacts`. Меню `Header` построено на `Link` и получает активный пункт из `App`; переходы к секциям выполняются через состояние `scrollRequest` и `scrollIntoView`. Хук `useLocation` синхронизирует подсветку меню с `pathname` и `hash`, а `useNavigate` используется в кнопках «О нас» и «На главную». Компоненты итогового приложения вынесены в `src/components/*.jsx`, пользовательские хуки – в `src/hooks/*.js`.

Вариант А полностью реализован в `task1.jsx`: маршруты `/`, `/about`, `/catalog`, `/product/:id` с вложенными вкладками и `path="*"`, форма на `/about`, кнопка `navigate(-1)` и `NavLink` в меню. Промежуточные приложения сохранены: `AppTopic3` возвращает `AppTopic2`, `AppTopic5` – `AppTopic4`, `CatalogPage` получает проп `withProducts`, а `ProductPage` – `withTabs`. В `task_itog.jsx` сохранены те же принципы маршрутизации, но структура сайта собственная: каталог и форма являются секциями главной, а отдельной страницей вынесена «О нас».

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

На листинге 11 показаны ключевые фрагменты стилей итогового приложения `task_itog.css`: переменные темы, шапка и меню, hero-блок, карточки преимуществ и игр, форма, кнопки и адаптивные правила. Полный файл дополнительно содержит анимации, стили каталога, отзывов, уведомлений и модальных окон.

**Листинг 11 - Итоговое задание (task_itog.css)**

```css
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

/* ... */

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

/* ... */

.nav a.active,
.nav a.active:hover {
    color: var(--bg);
    background-color: transparent;
    transform: none;
}

/* ... */

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

/* ... */

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

/* ... */

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

/* ... */

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


/* ... */

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

/* ... */

.btn-glow {
    color: var(--bg);
    background: linear-gradient(110deg, var(--cyan), var(--violet), var(--pink), var(--cyan));
    background-size: 300% 100%;
    animation: gradient-shift 6s linear infinite, glow-pulse 2.6s ease-in-out infinite;
}

.btn-glow:hover {
    color: #fff;
}

/* ... */

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

/* ... */

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
```

Рисунок 8 - Результат выполнения программы task_itog.jsx, task_itog.css

На рис. 8 показано итоговое приложение GameZone с рабочим меню и отдельной страницей «О нас». Маршрутизация не требует переписывать макет: `Header` вынесен наружу `<Routes>`, страницы и секции сохраняют общее состояние приложения, а переходы не перезагружают документ. Вариант Б оставляет каталог и форму на главной, но использует реальные URL и историю браузера; вариант А в `task1.jsx` дополнительно закрывает пункты чек-листа с отдельным каталогом, страницей товара, вложенными вкладками и формой на странице «О сайте». В обоих вариантах состояние React сохраняется при навигации, а несуществующие адреса обрабатываются страницей 404.

## Заключение

В ходе лабораторной работы изучена клиентская маршрутизация React-приложения. `BrowserRouter` связал приложение с History API, `Routes` и `Route` сопоставили URL компонентам, `Link` и `NavLink` обеспечили переходы без перезагрузки и подсветку активного пункта, `useParams` позволил читать динамические параметры, `useNavigate` – выполнять программные переходы, а `Outlet` – вкладывать маршруты и переключать вкладки. Отдельно отработана страница 404 через `path="*"`.

Сравнение с ЛР4 показало разницу между переключением экранов через `useState` и настоящими маршрутами: в первом случае URL не меняется и история не сохраняется, во втором меняется адрес, работают кнопки «назад/вперёд», прямые ссылки и закладки. В итоговом задании вариант А реализован в `task1.jsx`, вариант Б – в `task_itog.jsx`; оба варианта используют одни и те же возможности `react-router-dom`, но по-разному организуют страницы и секции сайта.
