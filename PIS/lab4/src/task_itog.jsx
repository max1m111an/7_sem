// ЛАБОРАТОРНАЯ РАБОТА №4
// ИТОГОВОЕ ЗАДАНИЕ — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Здесь собраны все темы 1–9: компоненты и JSX, props, useState, события,
// условный рендеринг, списки и key, controlled-форма, useEffect и подъём
// состояния. Файл можно целиком вставить в src/App.jsx проекта.

import { useEffect, useState } from "react";

const PRODUCTS = [
  { id: 1, name: "Абонемент на месяц", price: 2500 },
  { id: 2, name: "Разовое занятие", price: 500 },
  { id: 3, name: "Персональная тренировка", price: 1500 },
];

const NAV_ITEMS = ["Главная", "Расписание", "Тренеры", "Цены"];

// Тема 1 + 2: компонент без состояния, данные получает через props
function Header({ title, navItems, activeItem, onSelect }) {
  return (
    <header>
      <h1>{title}</h1>
      <nav>
        {navItems.map((item) => (
          <a
            key={item}
            href="#"
            className={item === activeItem ? "active" : ""}
            onClick={(e) => {
              e.preventDefault();
              onSelect(item);
            }}
          >
            {item}
          </a>
        ))}
      </nav>
    </header>
  );
}

// Тема 6 + 9: карточка товара — данные и колбэки приходят сверху
function ProductCard({ product, isFavorite, onToggleFavorite, onAddToCart }) {
  return (
    <li className={isFavorite ? "card card--favorite" : "card"}>
      <h3>{product.name}</h3>
      <p>{product.price} ₽</p>

      <button onClick={() => onToggleFavorite(product.id)}>
        {isFavorite ? "Убрать из избранного" : "В избранное"}
      </button>
      <button onClick={() => onAddToCart(product)}>
        Добавить в корзину
      </button>
    </li>
  );
}

// Тема 9: счётчик корзины — только отображает значение
function CartSummary({ count, total }) {
  return (
    <p>
      В корзине: {count} · Сумма: {total} ₽
    </p>
  );
}

function App() {
  // Тема 3 + 9: состояние живёт в App
  const [activeItem, setActiveItem] = useState(NAV_ITEMS[0]);
  const [favorites, setFavorites] = useState([]);
  const [cart, setCart] = useState([]);

  // Тема 7: controlled-форма
  const [customerName, setCustomerName] = useState("");
  const [phone, setPhone] = useState("");
  const [status, setStatus] = useState("");

  // Тема 8: имитация загрузки каталога
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const id = setTimeout(() => setIsLoading(false), 1000);
    return () => clearTimeout(id);
  }, []);

  // Тема 8: эффект с зависимостью
  useEffect(() => {
    console.log("Корзина изменилась: теперь " + cart.length + " товаров");
  }, [cart]);

  // Тема 5: ранний return
  if (isLoading) {
    return <p>Загрузка каталога «Спортивный клуб «Атлет»»...</p>;
  }

  const total = cart.reduce((sum, product) => sum + product.price, 0);

  function toggleFavorite(id) {
    setFavorites((prev) =>
      prev.includes(id) ? prev.filter((item) => item !== id) : [...prev, id]
    );
  }

  function addToCart(product) {
    setCart((prev) => [...prev, product]);
  }

  // Тема 7: проверка перед отправкой
  function handleSubmit(e) {
    e.preventDefault();

    if (customerName.trim() === "") {
      setStatus("Введите имя!");
      return;
    }

    if (phone.trim() === "") {
      setStatus("Введите телефон!");
      return;
    }

    setStatus(
      "Заказ от " + customerName + " принят: " + cart.length + " товаров"
    );
    setCustomerName("");
    setPhone("");
  }

  return (
    <div>
      <Header
        title="Спортивный клуб «Атлет»"
        navItems={NAV_ITEMS}
        activeItem={activeItem}
        onSelect={setActiveItem}
      />

      <main>
        <section>
          <h2>Каталог</h2>
          <CartSummary count={cart.length} total={total} />

          <ul>
            {PRODUCTS.map((product) => (
              <ProductCard
                key={product.id}
                product={product}
                isFavorite={favorites.includes(product.id)}
                onToggleFavorite={toggleFavorite}
                onAddToCart={addToCart}
              />
            ))}
          </ul>

          <button onClick={() => setCart([])}>Очистить корзину</button>
        </section>

        <section>
          <h2>Оставить заявку</h2>
          <form onSubmit={handleSubmit}>
            <label>
              Имя:
              <input
                value={customerName}
                onChange={(e) => setCustomerName(e.target.value)}
              />
            </label>

            <label>
              Телефон:
              <input
                type="tel"
                value={phone}
                onChange={(e) => setPhone(e.target.value)}
              />
            </label>

            <button type="submit">Отправить</button>
          </form>

          {/* Тема 5: условный рендеринг статуса */}
          {status && <p className="form-status">{status}</p>}
        </section>
      </main>
    </div>
  );
}

export default App;
