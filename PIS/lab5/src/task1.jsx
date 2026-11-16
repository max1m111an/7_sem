// ЛР №5 — React Router, вариант 04 (спортивный клуб «Атлет»).
// После каждой темы идёт свой App; итоговый App — в конце файла.
// Чтобы запустить отдельную тему, экспортируйте нужный AppTopicN.

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

// ============================================================
// ТЕМА 2. Link — навигация без перезагрузки
// ============================================================

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
          {({ isActive }) => (isActive ? '→ О сайте' : 'О сайте')}
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

// ============================================================
// ТЕМА 3. NavLink — подсветка активной ссылки
// ============================================================

// Набор маршрутов тот же, изменился только вид ссылок.
function AppTopic3() {
  return <AppTopic2 />
}

// ============================================================
// ТЕМА 4. useParams — динамические маршруты
// ============================================================

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

// ============================================================
// ТЕМА 5. useNavigate — переход по коду
// ============================================================

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

// ============================================================
// ТЕМА 6. Вложенные маршруты и Outlet
// ============================================================

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

// ============================================================
// ТЕМА 7. Страница 404
// ============================================================

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

// ============================================================
// ИТОГОВОЕ ЗАДАНИЕ
// ============================================================

// Вариант А: приложение темы 7 и есть итоговый вариант.
function App() {
  return <AppTopic5 />
}

export default App
