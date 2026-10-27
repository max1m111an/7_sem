// ЛАБОРАТОРНАЯ РАБОТА №4
// ТЕМА 8: useEffect — Вариант 04
// Тема варианта: Спортивный клуб «Атлет»
//
// Все TODO выполнены. Файл можно целиком вставить в src/App.jsx проекта.

import { useEffect, useState } from "react";

function App() {
  // TODO 3: счётчик времени на странице
  const [seconds, setSeconds] = useState(0);

  // TODO 5: корзина
  const [cartCount, setCartCount] = useState(0);

  // TODO 6: имитация загрузки
  const [isLoading, setIsLoading] = useState(true);

  // TODO 1: эффект при первом появлении
  useEffect(() => {
    console.log("Витрина «Спортивный клуб «Атлет»» загружена");
  }, []);

  // TODO 3 + TODO 4: интервал с обязательной очисткой.
  // Без return () => clearInterval(id) таймер продолжал бы тикать после
  // размонтирования компонента (или создавался бы заново), из-за чего
  // появляются "фоновые" интервалы, утечки памяти и ошибки обновления state
  // уже несуществующего компонента.
  useEffect(() => {
    const id = setInterval(() => {
      setSeconds((prev) => prev + 1);
    }, 1000);

    return () => clearInterval(id);
  }, []);

  // TODO 5: эффект, зависящий от state
  useEffect(() => {
    console.log(
      "Корзина «Спортивный клуб «Атлет»» изменилась: теперь " +
        cartCount +
        " товаров"
    );
  }, [cartCount]);

  // TODO 6: setTimeout с clearTimeout в cleanup
  useEffect(() => {
    const id = setTimeout(() => {
      setIsLoading(false);
    }, 1500);

    return () => clearTimeout(id);
  }, []);

  // TODO 6: ранний return, пока идёт загрузка
  if (isLoading) {
    return <p>Загрузка каталога...</p>;
  }

  return (
    <div>
      <h1>Спортивный клуб «Атлет»</h1>

      <p>Время на странице «Спортивный клуб «Атлет»»: {seconds} сек</p>
      <p>В корзине: {cartCount} товаров</p>

      <button onClick={() => setCartCount((prev) => prev + 1)}>
        Добавить в корзину
      </button>
    </div>
  );
}

export default App;

// TODO 2: Без массива / [] / [зависимость]
// useEffect(() => {...}) без второго аргумента выполняется после КАЖДОГО
// рендера компонента. useEffect(() => {...}, []) — только один раз после
// первого рендера (монтирования). useEffect(() => {...}, [count]) — после
// первого рендера и затем при каждом изменении значения count.

// TODO 7: Опасность забытого массива зависимостей
// Без массива зависимостей эффект запускается на каждый рендер. Если внутри
// вызвать fetch и по его результату сделать setState, это вызовет новый
// рендер, затем снова fetch — получится бесконечный цикл запросов к серверу
// (и потенциальный "лагающий" интерфейс). Для обычного console.log это лишь
// лишние сообщения, а для fetch — реальная нагрузка на сеть и сервер.
