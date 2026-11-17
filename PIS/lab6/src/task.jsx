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
import './task.css'

// Данные каталога.
const PRODUCTS = [
  {
    id: '1',
    title: 'Абонемент на месяц',
    price: 2500,
    duration: '30 дней',
    summary: 'Зал и групповые занятия без ограничений.',
    description: 'Тренируйтесь в зале и на групповых занятиях весь месяц.',
    perks: ['Безлимитный зал', 'Групповые тренировки', 'Заморозка на 7 дней'],
  },
  {
    id: '2',
    title: 'Разовое занятие',
    price: 500,
    duration: '1 визит, 60 минут',
    summary: 'Одно занятие без абонемента.',
    description: 'Приходите в удобное время и тренируйтесь на тренажёрах или в группе.',
    perks: ['Доступ к залу на день', 'Групповая тренировка', 'Инструктаж тренера'],
  },
  {
    id: '3',
    title: 'Персональная тренировка',
    price: 1500,
    duration: '60 минут',
    summary: 'Тренировка с тренером под ваши цели.',
    description: 'Тренер подберёт упражнения, проверит технику и составит план на неделю.',
    perks: ['Программа на занятие', 'Контроль техники', 'План на неделю'],
  },
]

// Отзывы по услугам: ключ — id услуги из каталога.
const REVIEWS = {
  1: [
    {
      id: '1-1',
      author: 'Мария К.',
      rating: 5,
      date: '14 октября 2026',
      text: 'Хожу почти каждый день. Зал чистый, занятия начинаются вовремя.',
    },
    {
      id: '1-2',
      author: 'Дмитрий П.',
      rating: 4,
      date: '2 октября 2026',
      text: 'Всё нужное уже включено в абонемент. По вечерам многолюдно.',
    },
  ],
  2: [
    {
      id: '2-1',
      author: 'Анна С.',
      rating: 5,
      date: '21 октября 2026',
      text: 'Пришла посмотреть зал и осталась. Всё хорошо объяснили.',
    },
    {
      id: '2-2',
      author: 'Илья В.',
      rating: 4,
      date: '9 октября 2026',
      text: 'Удобно зайти на одну тренировку, когда нет времени на абонемент.',
    },
  ],
  3: [
    {
      id: '3-1',
      author: 'Ольга Р.',
      rating: 5,
      date: '18 октября 2026',
      text: 'Разобрали технику и дали план на неделю. Прогресс заметен.',
    },
    {
      id: '3-2',
      author: 'Сергей М.',
      rating: 5,
      date: '5 октября 2026',
      text: 'Наконец понятно, зачем каждое упражнение. Рекомендую.',
    },
  ],
}

function getProduct(id) {
  return PRODUCTS.find((item) => item.id === String(id))
}

// ============================================================
// ТЕМА 1. BrowserRouter, Routes, Route
// ============================================================

// TODO 2. HomePage на path="/".
// TODO 6. Ссылка «О сайте» на главную.
// TODO 8. Ошибка: <a href> перезагружает страницу, нужен <Link>.
function HomePage() {
  return (
    <div className="page">
      <section className="hero">
        <div className="hero-content">
          <p className="eyebrow">Спортивный клуб «Атлет»</p>
          <h1>Сильнее с каждым занятием</h1>
          <p className="hero-lead">
            Тренажёрный зал, групповые тренировки и работа с персональным
            тренером в одном месте. Выберите удобный формат и начните сегодня.
          </p>
          <div className="hero-actions">
            <Link to="/catalog" className="btn">
              Услуги и цены
            </Link>
            <Link to="/about" className="btn btn--ghost">
              О сайте
            </Link>
          </div>
        </div>

        <div className="hero-card" aria-hidden="true">
          <span className="hero-card__badge">АТЛЕТ</span>
          <p>Зал · Тренеры · Результат</p>
          <div className="hero-card__stat">
            <strong>3</strong>
            <span>формата занятий</span>
          </div>
        </div>
      </section>

      <section className="benefits" aria-label="Почему выбирают Атлет">
        <article className="benefit">
          <span className="benefit__icon">01</span>
          <h2>Современный зал</h2>
          <p>
            Тренажёры для силовых и кардио тренировок, свободное пространство
            и чистый инвентарь.
          </p>
        </article>
        <article className="benefit">
          <span className="benefit__icon">02</span>
          <h2>Опытные тренеры</h2>
          <p>
            Помогут поставить технику, составят программу и поддержат, когда
            мотивация падает.
          </p>
        </article>
        <article className="benefit">
          <span className="benefit__icon">03</span>
          <h2>Гибкий график</h2>
          <p>
            Занимайтесь утром или вечером — тренировки проходят каждый день
            удобным вам временем.
          </p>
        </article>
      </section>
    </div>
  )
}

// TODO 3. AboutPage на path="/about".
// TODO 7. Ссылка «На главную» на странице «О сайте».
function AboutPage() {
  // TODO 22. Controlled-форма и navigate('/') после отправки.
  const [email, setEmail] = useState('')
  const [error, setError] = useState('')
  const navigate = useNavigate()

  function handleSubmit(event) {
    event.preventDefault()

    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email.trim())) {
      setError('Введите корректный email')
      return
    }

    navigate('/')
  }

  return (
    <div className="page page--narrow">
      <section className="about-hero">
        <p className="eyebrow">О сайте</p>
        <h1>Клуб «Атлет»</h1>
        <p className="section-lead">
          Мы создали место, куда хочется возвращаться: без лишнего шума, с
          понятными тренировками и командой, которая реально помогает
          двигаться к цели.
        </p>
        <Link to="/" className="back-link">
          ← На главную
        </Link>
      </section>

      <section className="about-grid">
        <article className="info-card">
          <h2>Наша цель</h2>
          <p>
            Сделать тренировки регулярной и приятной привычкой. Для этого у нас
            есть удобный зал, понятные программы и тренеры, которые умеют
            объяснять.
          </p>
        </article>
        <article className="info-card">
          <h2>Что предлагаем</h2>
          <p>
            Разовые занятия для знакомства, месячные абонементы для постоянных
            тренировок и персональные сессии, когда нужна индивидуальная
            работа с тренером.
          </p>
        </article>
      </section>

      <section className="signup-card">
        <h2>Получить прайс на email</h2>
        <p>Оставьте почту — пришлём актуальные цены и расписание тренировок.</p>
        <form className="signup-form" onSubmit={handleSubmit} noValidate>
          <label htmlFor="about-email">Ваш email</label>
          <input
            id="about-email"
            type="email"
            value={email}
            onChange={(event) => {
              setEmail(event.target.value)
              setError('')
            }}
            placeholder="you@example.com"
          />
          <button type="submit" className="btn">
            Отправить
          </button>
        </form>
        {error && <p className="form-error">{error}</p>}
      </section>
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
  // TODO 12. Правило .active — в task.css.
  // TODO 13. Проверка подсветки — описание в отчёте.
  // TODO 14. Функция className и стрелка «→» у ссылки «Цены».
  // TODO 15. Вопрос: NavLink без CSS — ответ в отчёте.
  return (
    <header className="site-header">
      <div className="header-inner">
        <Link to="/" className="brand">
          Атлет<span>.</span>
        </Link>
        <nav className="menu" aria-label="Главное меню">
          <NavLink to="/" end>
            Главная
          </NavLink>
          <NavLink
            to="/catalog"
            className={({ isActive }) =>
              isActive ? 'active price-link' : 'price-link'
            }
          >
            {({ isActive }) => (isActive ? '→ Услуги и цены' : 'Услуги и цены')}
          </NavLink>
          <NavLink to="/about">О сайте</NavLink>
          <NavLink to="/trains">Тренировки</NavLink>
          <NavLink to="/trainers">Тренеры</NavLink>
          <NavLink to="/schedule">Расписание</NavLink>
        </nav>
      </div>
    </header>
  )
}

// TODO 18. Список товаров через .map() и key; до темы 4 — заглушка.
function CatalogPage({ withProducts = false }) {
  return (
    <div className="page">
      <section className="section-head">
        <p className="eyebrow">Клуб «Атлет»</p>
        <h1>Услуги и цены</h1>
        <p className="section-lead">
          Выберите формат занятий: попробовать клуб, тренироваться весь месяц
          или работать с тренером индивидуально.
        </p>
      </section>

      {withProducts ? (
        <ul className="product-list">
          {PRODUCTS.map((product, index) => (
            <li key={product.id}>
              <Link to={`/product/${product.id}`} className="product-card">
                <span className="product-card__number">0{index + 1}</span>
                <h2>{product.title}</h2>
                <p>{product.summary}</p>
                <span className="product-card__footer">
                  <span className="price">{product.price} ₽</span>
                  <span className="product-card__more">Подробнее →</span>
                </span>
              </Link>
            </li>
          ))}
        </ul>
      ) : (
        <p className="empty-note">Список услуг появится позже.</p>
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
  const product = getProduct(id)

  if (!product) {
    return (
      <div className="page page--narrow">
        <section className="not-found">
          <p className="eyebrow">Ошибка</p>
          <h1>Товар не найден</h1>
          <p>Возможно, эта услуга больше не действует.</p>
          <Link to="/catalog" className="btn">
            Вернуться в каталог
          </Link>
        </section>
      </div>
    )
  }

  // TODO 21. Кнопка «Назад» через navigate(-1).
  // TODO 27. Вкладки «Описание»/«Отзывы» и <Outlet />.
  return (
    <div className="page">
      <section className="product-hero">
        <Link to="/catalog" className="back-link">
          ← Услуги и цены
        </Link>
        <p className="eyebrow">Клуб «Атлет»</p>
        <h1>{product.title}</h1>
        <p className="product-summary">{product.summary}</p>
        <div className="product-meta">
          <span className="product-meta__item">
            <span>Стоимость</span>
            <strong>{product.price} ₽</strong>
          </span>
          <span className="product-meta__item">
            <span>Формат</span>
            <strong>{product.duration}</strong>
          </span>
        </div>
        <button
          type="button"
          className="btn btn--ghost"
          onClick={() => navigate(-1)}
        >
          Назад к каталогу
        </button>
      </section>

      {withTabs && (
        <>
          <nav className="tabs" aria-label="Информация об услуге">
            <NavLink to="" end>
              Описание
            </NavLink>
            <NavLink to="reviews">Отзывы</NavLink>
          </nav>
          <div className="tab-panel">
            <Outlet />
          </div>
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
  const { id } = useParams()
  const product = getProduct(id)

  if (!product) {
    return <p className="empty-note">Описание услуги пока недоступно.</p>
  }

  return (
    <div className="tab-content">
      <h2>Об услуге</h2>
      <p>{product.description}</p>
      <h3>Что входит</h3>
      <ul className="perk-list">
        {product.perks.map((perk) => (
          <li key={perk}>{perk}</li>
        ))}
      </ul>
    </div>
  )
}

function ProductReviews() {
  // TODO 28. Отзывы-заглушки через .map() с key.
  const { id } = useParams()
  const reviews = REVIEWS[id] ?? []

  return (
    <div className="tab-content">
      <h2>Отзывы</h2>

      {reviews.length === 0 ? (
        <p className="empty-note">Отзывов пока нет.</p>
      ) : (
        <ul className="review-list">
          {reviews.map((review) => (
            <li key={review.id} className="review-card">
              <div className="review-card__head">
                <span className="review-avatar">{review.author[0]}</span>
                <div>
                  <p className="review-author">{review.author}</p>
                  <time className="review-date">{review.date}</time>
                </div>
                <span
                  className="review-rating"
                  aria-label={`Оценка ${review.rating} из 5`}
                >
                  {'★'.repeat(review.rating)}
                  <span>{'★'.repeat(5 - review.rating)}</span>
                </span>
              </div>
              <p className="review-text">{review.text}</p>
            </li>
          ))}
        </ul>
      )}
    </div>
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
    <div className="page page--narrow">
      <section className="not-found">
        <p className="eyebrow">Ошибка 404</p>
        <h1>404 — страница не найдена</h1>
        <p>
          Возможно, адрес изменился или такого раздела никогда не было.
          Вернитесь на главную и выберите нужный пункт в меню.
        </p>
        <Link to="/" className="btn">
          Вернуться на главную
        </Link>
      </section>
    </div>
  )
}

// ============================================================
// ДАННЫЕ С СЕРВЕРА (ЛР6): тренировки, тренеры, расписание
// ============================================================

// Same-origin /api is proxied to Express by vite.config.js in dev/preview.
// VITE_API_BASE can override it for a separate production backend.
const API_BASE = import.meta.env.VITE_API_BASE || '/api'

async function api(path, options = {}) {
  const { headers, ...rest } = options
  let response
  try {
    response = await fetch(`${API_BASE}${path}`, {
      ...rest,
      headers: { 'Content-Type': 'application/json', ...headers },
    })
  } catch (err) {
    throw new Error(
      `Нет связи с API (${API_BASE}). Запустите сервер БД командой "make api" (или "make all"). ${err.message}`,
    )
  }

  let data = null
  try {
    data = await response.json()
  } catch {
    // сервер может вернуть пустое тело
  }

  if (!response.ok) {
    if (data?.error) throw new Error(data.error)
    if (response.status >= 500) {
      throw new Error(
        `API недоступен (HTTP ${response.status}). Запустите "make api" (или "make all") — сервер task.js должен слушать порт ${import.meta.env.VITE_API_PORT || 3000}.`,
      )
    }
    throw new Error(`Ошибка запроса: ${response.status}`)
  }
  return data
}

function useCrud(path) {
  const [items, setItems] = useState([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')

  async function refresh() {
    setLoading(true)
    setError('')
    try {
      setItems(await api(path))
    } catch (err) {
      setError(err.message)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    refresh()
  }, [path])

  async function create(values) {
    await api(path, { method: 'POST', body: JSON.stringify(values) })
    await refresh()
  }

  async function update(id, values) {
    await api(`${path}/${id}`, {
      method: 'PUT',
      body: JSON.stringify(values),
    })
    await refresh()
  }

  async function remove(id) {
    await api(`${path}/${id}`, { method: 'DELETE' })
    await refresh()
  }

  return { items, loading, error, refresh, create, update, remove }
}

function toFormValues(item, fields) {
  return Object.fromEntries(
    fields.map(({ name, type }) => {
      const value = item?.[name]
      if (value === undefined || value === null) return [name, '']
      return [name, type === 'time' ? String(value).slice(0, 5) : String(value)]
    })
  )
}

function buildPayload(form, fields) {
  const payload = {}
  for (const field of fields) {
    const raw = String(form[field.name] ?? '').trim()
    if (!raw) throw new Error(`Заполните поле «${field.label}»`)

    if (field.type === 'number') {
      const number = Number(raw)
      if (!Number.isInteger(number)) {
        throw new Error(`Поле «${field.label}» должно быть целым числом`)
      }
      payload[field.name] = number
    } else {
      payload[field.name] = raw
    }
  }
  return payload
}

function FieldInput({ field, value, onChange }) {
  if (field.type === 'select') {
    return (
      <label className="admin-field">
        <span>{field.label}</span>
        <select
          value={value}
          onChange={(event) => onChange(field.name, event.target.value)}
          required
        >
          <option value="">Выберите...</option>
          {field.options.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </label>
    )
  }

  const inputType =
    field.type === 'number' ? 'number' : field.type === 'time' ? 'time' : 'text'

  return (
    <label className="admin-field">
      <span>{field.label}</span>
      <input
        type={inputType}
        value={value}
        onChange={(event) => onChange(field.name, event.target.value)}
        required
      />
    </label>
  )
}

function EntityPage({ title, description, path, fields, empty, columns, hint }) {
  const { items, loading, error, refresh, create, update, remove } = useCrud(path)
  const [form, setForm] = useState(empty)
  const [editingId, setEditingId] = useState(null)
  const [editForm, setEditForm] = useState(empty)
  const [actionError, setActionError] = useState('')

  function fieldSetter(setter) {
    return (name, value) => setter((current) => ({ ...current, [name]: value }))
  }

  const changeForm = fieldSetter(setForm)
  const changeEditForm = fieldSetter(setEditForm)

  async function handleAdd(event) {
    event.preventDefault()
    setActionError('')
    try {
      await create(buildPayload(form, fields))
      setForm(empty)
    } catch (err) {
      setActionError(err.message)
    }
  }

  function startEdit(item) {
    setActionError('')
    setEditingId(item.id)
    setEditForm(toFormValues(item, fields))
  }

  async function handleSave(id) {
    setActionError('')
    try {
      await update(id, buildPayload(editForm, fields))
      setEditingId(null)
    } catch (err) {
      setActionError(err.message)
    }
  }

  async function handleDelete(id) {
    setActionError('')
    try {
      await remove(id)
      if (editingId === id) setEditingId(null)
    } catch (err) {
      setActionError(err.message)
    }
  }

  return (
    <div className="page">
      <section className="section-head">
        <p className="eyebrow">Клуб «Атлет» · база данных</p>
        <h1>{title}</h1>
        <p className="section-lead">{description}</p>
      </section>

      {hint && <p className="empty-note">{hint}</p>}

      <section className="admin-card">
        <h2>Добавить запись</h2>
        <form className="admin-form" onSubmit={handleAdd}>
          {fields.map((field) => (
            <FieldInput
              key={field.name}
              field={field}
              value={form[field.name]}
              onChange={changeForm}
            />
          ))}
          <button type="submit" className="btn">Добавить</button>
        </form>
        {actionError && <p className="form-error">{actionError}</p>}
      </section>

      <section className="admin-card">
        <h2>Список</h2>
        {loading && <p className="empty-note">Загрузка...</p>}
        {error && (
          <p className="form-error">
            {error}{' '}
            <button type="button" className="link-button" onClick={refresh}>
              Повторить
            </button>
          </p>
        )}
        {!loading && !error && items.length === 0 && (
          <p className="empty-note">Записей пока нет.</p>
        )}

        {items.length > 0 && (
          <div className="admin-table-wrap">
            <table className="admin-table">
              <thead>
                <tr>
                  {columns.map((column) => (
                    <th key={column.name}>{column.label}</th>
                  ))}
                  <th>Действия</th>
                </tr>
              </thead>
              <tbody>
                {items.map((item) =>
                  editingId === item.id ? (
                    <tr key={item.id}>
                      {fields.map((field) => (
                        <td key={field.name}>
                          <FieldInput
                            field={field}
                            value={editForm[field.name]}
                            onChange={changeEditForm}
                          />
                        </td>
                      ))}
                      <td className="admin-actions">
                        <button
                          type="button"
                          className="btn btn--small"
                          onClick={() => handleSave(item.id)}
                        >
                          Сохранить
                        </button>
                        <button
                          type="button"
                          className="btn btn--small btn--ghost"
                          onClick={() => setEditingId(null)}
                        >
                          Отмена
                        </button>
                      </td>
                    </tr>
                  ) : (
                    <tr key={item.id}>
                      {columns.map((column) => (
                        <td key={column.name}>
                          {column.format
                            ? column.format(item[column.name])
                            : item[column.name]}
                        </td>
                      ))}
                      <td className="admin-actions">
                        <button
                          type="button"
                          className="btn btn--small btn--ghost"
                          onClick={() => startEdit(item)}
                        >
                          Изменить
                        </button>
                        <button
                          type="button"
                          className="btn btn--small btn--danger"
                          onClick={() => handleDelete(item.id)}
                        >
                          Удалить
                        </button>
                      </td>
                    </tr>
                  )
                )}
              </tbody>
            </table>
          </div>
        )}
      </section>
    </div>
  )
}

const TRAIN_FIELDS = [
  { name: 'name', label: 'Название' },
  { name: 'day', label: 'День недели' },
  { name: 'duration_min', label: 'Длительность, мин', type: 'number' },
  { name: 'level', label: 'Уровень' },
  { name: 'max_participants', label: 'Макс. участников', type: 'number' },
  { name: 'hall', label: 'Зал' },
]
const EMPTY_TRAIN = {
  name: '',
  day: '',
  duration_min: '',
  level: '',
  max_participants: '',
  hall: '',
}

function TrainsPage() {
  return (
    <EntityPage
      title="Тренировки"
      description="Список тренировок хранится в PostgreSQL. Добавление, редактирование и удаление работают через API."
      path="/trains"
      fields={TRAIN_FIELDS}
      empty={EMPTY_TRAIN}
      columns={TRAIN_FIELDS}
    />
  )
}

const TRAINER_FIELDS = [
  { name: 'name', label: 'Имя' },
  { name: 'specialty', label: 'Специализация' },
  { name: 'experience_years', label: 'Стаж, лет', type: 'number' },
  { name: 'phone', label: 'Телефон' },
  { name: 'rating', label: 'Рейтинг', type: 'number' },
  { name: 'photo_url', label: 'Ссылка на фото' },
]
const EMPTY_TRAINER = {
  name: '',
  specialty: '',
  experience_years: '',
  phone: '',
  rating: '',
  photo_url: '',
}

function TrainersPage() {
  return (
    <EntityPage
      title="Тренеры"
      description="Тренеры — независимая таблица. Пара телефон + photo_url уникальна, сервер вернёт ошибку при дубле."
      path="/trainers"
      fields={TRAINER_FIELDS}
      empty={EMPTY_TRAINER}
      columns={TRAINER_FIELDS}
    />
  )
}

const SCHEDULE_FIELDS = [
  { name: 'workout_id', label: 'Тренировка', type: 'select' },
  { name: 'trainer_id', label: 'Тренер', type: 'select' },
  { name: 'time', label: 'Время', type: 'time' },
  { name: 'room', label: 'Зал' },
]
const EMPTY_SCHEDULE = { workout_id: '', trainer_id: '', time: '', room: '' }
const SCHEDULE_COLUMNS = [
  { name: 'workout_name', label: 'Тренировка' },
  { name: 'trainer_name', label: 'Тренер' },
  {
    name: 'time',
    label: 'Время',
    format: (value) => (value ? String(value).slice(0, 5) : ''),
  },
  { name: 'room', label: 'Зал' },
]

function SchedulePage() {
  const trains = useCrud('/trains')
  const trainers = useCrud('/trainers')

  if (trains.loading || trainers.loading) {
    return (
      <div className="page">
        <p className="empty-note">Загрузка тренировок и тренеров...</p>
      </div>
    )
  }

  if (trains.error || trainers.error) {
    return (
      <div className="page">
        <p className="form-error">
          Не удалось загрузить справочники: {trains.error || trainers.error}
        </p>
      </div>
    )
  }

  const fields = SCHEDULE_FIELDS.map((field) => ({
    ...field,
    options:
      field.name === 'workout_id'
        ? trains.items.map((item) => ({ value: String(item.id), label: item.name }))
        : field.name === 'trainer_id'
          ? trainers.items.map((item) => ({ value: String(item.id), label: item.name }))
          : [],
  }))

  const hint =
    trains.items.length === 0 || trainers.items.length === 0
      ? 'Сначала добавьте хотя бы одну тренировку и одного тренера.'
      : ''

  return (
    <EntityPage
      title="Расписание"
      description="Связка тренировки и тренера: время и зал. Список приходит с сервера через JOIN."
      path="/schedule"
      fields={fields}
      empty={EMPTY_SCHEDULE}
      columns={SCHEDULE_COLUMNS}
      hint={hint}
    />
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
          <Route path="/trains" element={<TrainsPage />} />
          <Route path="/trainers" element={<TrainersPage />} />
          <Route path="/schedule" element={<SchedulePage />} />
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
export function App() {
  return <AppTopic7 />
}

export default App
