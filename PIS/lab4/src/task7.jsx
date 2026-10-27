// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 7: Формы (controlled components) — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

import { useState } from "react";

function App() {
  // TODO 1: контролируемое поле имени
  const [customerName, setCustomerName] = useState("");

  // TODO 5: второе контролируемое поле — количество
  const [quantity, setQuantity] = useState(1);

  // TODO 2: value без onChange
  // Если оставить <input value={customerName} /> без onChange, поле
  // "замерзает": React при каждом рендере принудительно возвращает в него
  // значение из state, а state никогда не обновляется. Ввод игнорируется.
  // Лечится привязкой onChange, который вызывает setCustomerName.

  // TODO 4 + TODO 6: отправка формы с проверкой
  function handleSubmit(e) {
    e.preventDefault();

    if (customerName.trim() === "") {
      console.log("Введите имя!");
      return;
    }

    console.log("Заказ от " + customerName + " в «Спортивный клуб «Атлет»»");
    console.log({ customerName, quantity });
  }

  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      {/* TODO 3: кнопка «Очистить» доказывает, что поле контролируемое —
          видимое значение меняется вместе со state */}
      <form onSubmit={handleSubmit}>
        <label>
          Имя покупателя:
          <input
            value={customerName}
            onChange={(e) => setCustomerName(e.target.value)}
          />
        </label>

        <button type="button" onClick={() => setCustomerName("")}>
          Очистить
        </button>

        <label>
          Количество «Абонемент на месяц»:
          <input
            type="number"
            value={quantity}
            onChange={(e) => setQuantity(e.target.value)}
          />
        </label>

        <button type="submit">Оформить заказ</button>
      </form>
    </div>
  );
}

export default App;

// TODO 7: "Единственный источник истины"
// Для controlled-компонента значение поля живёт не в DOM, а в state: input
// лишь отображает customerName, а любое изменение сразу пишется в state через
// setCustomerName. Если убрать value и оставить только onChange, поле станет
// неконтролируемым: истинным значением будет то, что хранит сам DOM, а state
// будет лишь копией, которую можно рассинхронизировать (например, очистить
// state кнопкой, а в поле текст останется).
