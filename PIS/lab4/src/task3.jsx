// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 3: State (useState) — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

import { useState } from "react";

const PRODUCT_PRICE = 2500;

function App() {
  // TODO 1: первый state
  const [cartCount, setCartCount] = useState(0);

  // TODO 4: второй, независимый state
  const [isFavorite, setIsFavorite] = useState(false);

  // TODO 6: state, который считает сумму
  const [total, setTotal] = useState(0);

  // TODO 2: почему нельзя менять напрямую
  // cartCount = cartCount + 1 меняет обычную переменную "по факту", но React
  // не узнаёт об изменении и не перерисовывает компонент. Перерисовку
  // запускает только вызов setCartCount(...).
  //
  // TODO 3: кнопка увеличения. Один обработчик меняет сразу два state:
  // количество и сумму (TODO 6).
  function handleAdd() {
    setCartCount((prev) => prev + 1);
    setTotal((prev) => prev + PRODUCT_PRICE);
  }

  // TODO 5: сброс состояния
  function handleClear() {
    setCartCount(0);
    setTotal(0);
  }

  function handleToggleFavorite() {
    setIsFavorite((prev) => !prev);
  }

  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      <p>В корзине «Спортивный клуб «Атлет»»: {cartCount}</p>
      <p>Сумма заказа: {total} ₽</p>
      <p>В избранном: {String(isFavorite)}</p>

      <button onClick={handleAdd}>Добавить в корзину</button>
      <button onClick={handleClear}>Очистить корзину</button>
      <button onClick={handleToggleFavorite}>
        {isFavorite ? "Убрать из избранного" : "В избранное"}
      </button>
    </div>
  );
}

export default App;

// TODO 7: Почему setState, а не прямое присваивание
// React не следит за обычными переменными: он узнаёт об изменениях только по
// вызову setter-функции. При вызове setCartCount React сохраняет новое
// значение в своём внутреннем хранилище и планирует перерисовку компонента,
// после чего вызывает функцию App заново с уже обновлённым state. Прямое
// присваивание (cartCount = cartCount + 1) просто потеряется при следующем
// рендере, потому что при каждом вызове компонента useState снова отдаёт
// сохранённое внутри React значение.
