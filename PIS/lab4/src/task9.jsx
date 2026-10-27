// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 9: Подъём состояния (lifting state up) — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

import { useState } from "react";

// TODO 1: компонент-сосед, получающий значение через props (данные вниз)
function CartCount({ count }) {
  return <p>В корзине: {count}</p>;
}

// TODO 1: компонент-сосед, получающий функцию через props (событие вверх)
function AddButton({ onAdd }) {
  return <button onClick={onAdd}>Добавить Абонемент на месяц</button>;
}

// TODO 3: третий компонент-сосед
function ResetButton({ onReset }) {
  return <button onClick={onReset}>Очистить корзину</button>;
}

// TODO 5: четвёртый компонент — избранное
function FavoriteButton({ isFavorite, onToggle }) {
  return (
    <div>
      <p>В избранном: {String(isFavorite)}</p>
      <button onClick={onToggle}>
        {isFavorite ? "Убрать из избранного" : "В избранное"}
      </button>
    </div>
  );
}

function App() {
  // TODO 1: общий state для соседей
  const [cartCount, setCartCount] = useState(0);

  // TODO 5: второй поднятый state
  const [isFavorite, setIsFavorite] = useState(false);

  // TODO 6: все компоненты в одном блоке, состояние — только в App
  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      <CartCount count={cartCount} />
      <AddButton onAdd={() => setCartCount((prev) => prev + 1)} />
      <ResetButton onReset={() => setCartCount(0)} />
      <FavoriteButton
        isFavorite={isFavorite}
        onToggle={() => setIsFavorite((prev) => !prev)}
      />
    </div>
  );
}

export default App;

// TODO 2: Ошибка — дублирование state
// Если CartCount заводит собственный useState(0), он показывает только своё
// локальное значение. Кнопка AddButton меняет другой state — тот, что лежит
// в App, — поэтому локальный count компонента CartCount никогда не узнает об
// этих изменениях и всегда будет показывать 0. Общее состояние должно жить в
// ближайшем общем родителе.

// TODO 4: Направление данных — вниз или вверх?
// В CartCount данные идут ВНИЗ: значение cartCount передаётся пропом count.
// В AddButton вниз передаётся функция onAdd (обработчик), а когда пользователь
// кликает, вызывается onAdd — это уже событие "вверх": компонент-ребёнок
// просит родителя изменить состояние. Сам state всегда принадлежит родителю.

// TODO 7: Почему соседи не могут напрямую общаться
// Props передаются только от родителя к прямому потомку. Соседние компоненты
// не видят друг друга: CartCount ничего не знает о существовании AddButton,
// и наоборот. Поэтому общий state поднимают в ближайшего общего родителя
// (App), а соседи общаются через него: один получает значение вниз, другой —
// функцию-колбэк, которая меняет это значение.
