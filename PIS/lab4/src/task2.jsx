// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 2: Props — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

// --- Лёгкий уровень ---

// TODO 1 + TODO 2: Компонент с props и обращение через деструктуризацию.
// Если параметр не деструктурирован (function ProductCard(props)), то name —
// это не отдельная переменная, а поле объекта props: props.name. Поэтому код
// {name} из TODO 2 падал бы с ошибкой "name is not defined". Ниже параметр
// деструктурирован, значит name/price — уже готовые переменные.
//
// TODO 3: discount получает значение по умолчанию "0%".
// TODO 4: quantity — число, переданное через {} (quantity={3}), итоговая
// сумма = цена * количество.
// TODO 5: props только для чтения — не переприсваиваем name, а создаём
// отдельную переменную upperName.
function ProductCard({ name, price, discount = "0%", quantity = 1 }) {
  const upperName = name.toUpperCase();
  const total = price * quantity;

  return (
    <div className="product-card">
      <h3>{upperName}</h3>
      <p>Цена: {price} ₽</p>
      <p>Скидка: {discount}</p>
      <p>Количество: {quantity}</p>
      <p>Итого: {total} ₽</p>
      <img src="/product.png" alt="" className="product-img" />
    </div>
  );
}

function App() {
  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      {/* TODO 3: вызов без discount — сработает значение по умолчанию "0%" */}
      <ProductCard name="Абонемент на месяц" price={2500} />
      <ProductCard name="Абонемент на месяц" price={2500} discount="10%" />

      {/* TODO 4: quantity передаётся числом через {} */}
      <ProductCard name="Разовое занятие" price={500} quantity={3} />

      {/* TODO 6: один компонент — три разных набора props */}
      <ProductCard name="Абонемент на месяц" price={2500} />
      <ProductCard name="Разовое занятие" price={500} />
      <ProductCard name="Персональная тренировка" price={1500} />
    </div>
  );
}

export default App;

// TODO 7: HTML-атрибут vs JSX prop
// Обычный HTML-атрибут (data-price="220") всегда передаёт строку — браузер не
// умеет хранить в атрибуте число, объект или функцию. JSX prop — это просто
// элемент объекта props, поэтому через {} можно передать любое JS-значение:
// число (price={220}), boolean, массив, объект, функцию-обработчик и даже
// другой JSX-элемент.
