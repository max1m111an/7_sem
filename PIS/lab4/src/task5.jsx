// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 5: Условный рендеринг — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

import { useState } from "react";

function App() {
  // TODO 1: наличие товара
  const [inStock, setInStock] = useState(true);

  // TODO 2: ловушка с числом 0
  const [cartCount, setCartCount] = useState(0);

  // TODO 3: скидка
  const [hasDiscount, setHasDiscount] = useState(false);

  // TODO 4: загрузка
  const [isLoading, setIsLoading] = useState(true);

  // TODO 6: избранное
  const [isFavorite, setIsFavorite] = useState(false);

  // TODO 4: ранний return — обычный if ДО основного return.
  if (isLoading) {
    return (
      <div>
        <p>Загрузка каталога «Спортивный клуб «Атлет»»...</p>
        <button onClick={() => setIsLoading(false)}>Завершить загрузку</button>
      </div>
    );
  }

  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      {/* TODO 6: условный className */}
      <div className={isFavorite ? "card card--favorite" : "card"}>
        <h2>Абонемент на месяц</h2>
        <p>2500 ₽</p>

        {/* TODO 1: тернарный оператор */}
        <p>{inStock ? "В наличии" : "Нет в наличии"}</p>

        {/* TODO 3: плашка только при hasDiscount */}
        {hasDiscount && <p className="discount-badge">Скидка!</p>}
      </div>

      {/* TODO 2: было {cartCount && <p>...</p>} — при 0 на экране печатался
          сам ноль, потому что && возвращает левый операнд, если он ложный,
          а 0 — ложное значение, которое React выводит как текст. */}
      {cartCount > 0 && <p>Товаров в корзине: {cartCount}</p>}

      {/* TODO 5: цепочка тернарников */}
      <p>
        {cartCount === 0
          ? "Корзина пуста"
          : cartCount < 3
            ? "Товаров мало"
            : "Полная корзина"}
      </p>

      <button onClick={() => setCartCount((prev) => prev + 1)}>
        Добавить в корзину
      </button>
      <button onClick={() => setCartCount(0)}>Очистить корзину</button>
      <button onClick={() => setInStock((prev) => !prev)}>
        Переключить наличие
      </button>
      <button onClick={() => setHasDiscount((prev) => !prev)}>
        Переключить скидку
      </button>
      <button onClick={() => setIsFavorite((prev) => !prev)}>
        {isFavorite ? "Убрать из избранного" : "В избранное"}
      </button>
    </div>
  );
}

export default App;

// TODO 7: Почему if нельзя писать прямо в JSX
// Фигурные скобки {} в JSX принимают только выражения (то, что возвращает
// значение), а if — это инструкция (statement), у неё нет результата, поэтому
// {if (...) {...}} — синтаксическая ошибка. Вместо неё используют три способа:
// 1) тернарный оператор {condition ? <A /> : <B />};
// 2) логическое И {condition && <A />} для "показать или ничего";
// 3) ранний return с обычным if в теле компонента ДО основного return
//    (и вообще любой if/else в обработчиках событий).
