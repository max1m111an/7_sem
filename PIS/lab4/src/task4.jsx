// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 4: Обработка событий — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

function App() {
  // TODO 1: onClick — передаём саму функцию, без скобок.
  function handleAddClick() {
    console.log("Абонемент на месяц добавлен в корзину");
  }

  // TODO 2: почему onClick={handleAddClick()} ломается
  // Скобки означают немедленный вызов функции прямо во время рендера, поэтому
  // сообщение появляется сразу при загрузке страницы, а в onClick попадает
  // результат вызова (undefined). Правильно — onClick={handleAddClick}.

  // TODO 3: onChange — при каждом вводимом символе печатаем значение поля.
  function handleSearchChange(e) {
    console.log(e.target.value);
  }

  // TODO 5: аргумент в обработчик передаётся через стрелочную обёртку,
  // иначе removeItem("...") вызовется сразу при рендере.
  function removeItem(itemName) {
    console.log(itemName + " удалён из корзины");
  }

  // TODO 6: наведение и уход курсора.
  function handleMouseEnter() {
    console.log("наведение на Абонемент на месяц");
  }

  function handleMouseLeave() {
    console.log("курсор ушёл с Абонемент на месяц");
  }

  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      <div onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave}>
        <h2>Абонемент на месяц</h2>
        <p>2500 ₽</p>

        <button onClick={handleAddClick}>Добавить в корзину</button>
        <button onClick={() => removeItem("Абонемент на месяц")}>Удалить</button>
      </div>

      <input
        placeholder="Поиск по каталогу «Спортивный клуб «Атлет»»"
        onChange={handleSearchChange}
      />
    </div>
  );
}

export default App;

// TODO 4: onChange в React ≠ change в HTML
// В HTML событие "change" у текстового поля срабатывает только когда поле
// теряет фокус (или пользователь нажал Enter). В React onChange — это аналог
// HTML-события "input": обработчик вызывается сразу при каждом изменении
// значения, то есть на каждый введённый или удалённый символ.

// TODO 7: Что такое e (event) в обработчике
// React сам передаёт первым параметром объект синтетического события — в нём
// есть тип события, координаты мыши, нажатые клавиши и т.д. Свойство
// e.target — это ссылка на конкретный DOM-элемент, на котором событие
// произошло (например, то самое поле поиска), поэтому e.target.value даёт
// текущий текст этого поля.
