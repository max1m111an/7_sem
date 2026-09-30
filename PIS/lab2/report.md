# Лабораторная работа №2

HTML и CSS. Вариант 04. Отчёт по проделанной работе.

## Задание 1. Структура HTML-документа и базовые теги

Изучена структура HTML-документа и базовые теги. В `<head>` указаны кодировка `UTF-8`, обязательный тег `viewport` и заголовок страницы. В `<body>` размещены заголовок первого уровня `<h1>` с названием «Музыкальная школа» и абзац с описанием, подзаголовок `<h2>` и два абзаца под ним, один из которых содержит выделенное слово в теге `<span>`.

Применены маркированный список `<ul>` из трёх пунктов и нумерованный список `<ol>` из трёх преимуществ, обёрнутый вместе с заголовком в `<div>`. Добавлены ссылка `<a>` на внешний сайт и изображение `<img>` с заполненным атрибутом `alt`, вложенное в ссылку. Отдельно разобрана ошибка вложенности тегов: запись `<p><b>жирный текст</p></b>` исправлена на `<p><b>жирный текст</b></p>`, поскольку закрывающие теги должны идти в обратном порядке открытия.

На листинг 1 показано: заголовки и абзацы, маркированный и нумерованный списки, ссылка с картинкой, span внутри абзаца, исправление ошибки вложенности.

**Листинг 1 - Задание №1**

```html
<!DOCTYPE html>
< lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Вариант 04</title>
  </head>
  <body>

    <!--TODO 1-->
    <h1>Музыкальная школа</h1>
    <p>Учебное заведение для подготовки будущих концертмейстеров.</p>

    <!--TODO 2-->
    <h2>Академия Артистов</h2>
    <p>Адрес: просп. Испытателей, д. 30, корп. 2 этаж 3</p>
    <p>
        Открыт <span>новый набор</span> на 2026-й учебный год!
    </p>

    <!--TODO 3-->
    <p>Используются классические инструменты</p>
    <ul>
        <li>Фортепиано</li>
        <li>Смычковые</li>
        <li>Духовые и т.д.</li>
    </ul>

    <!--TODO 4-->
    <a href="https://akkords.pro/">
    <img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.musicgrotto.com%2Fwp-content%2Fuploads%2F2022%2F10%2Fclassical-musician-playing-violin-music-background-graphic.jpg&f=1&nofb=1&ipt=041b69eb3112d8ac7d9833dc3290fdfb85a8016a6e22b55e9051dfe14011374b" alt="Сайт гитарных аккордов">
    </a>

    <!--TODO 5-->
    <!-- <span> — inline-тег, <div> — блочный: новая строка. -->

    <!--TODO 6-->
    <div>
        <h2>Преимущества нашей музыкальной школы:</h2>
        <ol>
            <li>Более 85% выпускников поступают в консерватории;</li>
            <li>Обучение за год не дороже 100 000 руб.;</li>
            <li>В центре города.</li>
        </ol>
    </div>

    <!--TODO 7-->
    <p><b>жирный текст</b></p>

  </body>
</html>
```

Рисунок 1.1 - Результат выполнения программы task1.html

Тег `<span>` является строчным (inline) тегом: он не начинает новую строку и не имеет собственных отступов, поэтому обёрнутое слово остаётся внутри той же строки. Тег `<div>` является блочным тегом: он всегда встаёт с новой строки, занимает всю ширину родителя и по умолчанию имеет вертикальные отступы. Поэтому слово внутри абзаца оборачивается в `<span>`, а `<div>` ставится только как отдельный блок, как в случае с нумерованным списком преимуществ.

## Задание 2. Семантические теги

Изучены семантические теги, которые описывают смысл каждой части страницы. В `<header>` размещён заголовок `<h1>` с названием сайта и элемент `<nav>` с тремя ссылками на разделы. В `<main>` добавлен `<article>` с заголовком `<h2>` и двумя абзацами, а вне статьи, но внутри `<main>`, – `<aside>` с дополнительной информацией по теме.

После `<main>` добавлен `<footer>` с абзацем-копирайтом и ссылкой на почту `mailto:info@example.ru`. Весь контент внутри `<body>` обёрнут в `<div>`, который нужен только для оформления, например для ограничения ширины страницы через CSS, и собственного смысла не несёт.

На листинг 2 показано: header с h1 и nav, main с article и aside, footer с копирайтом и почтовой ссылкой, div-обёртка всего контента.

**Листинг 2 - Задание №2**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Вариант 04</title>
  </head>
  <body>

    <!--TODO 1-->
    <div>
    <header>
        <h1>Музыкальная школа</h1>

    <!--TODO 2-->
         <nav>
             <a href="#">№1</a>
             <a href="#">№2</a>
             <a href="#">№3</a>
         </nav>
    </header>

    <!--TODO 3-->
    <main>
        <article>
            <h2>а</h2>
            <p>б</p>
            <p>в</p>
        </article>

    <!--TODO 4-->
         <aside>
             <p>Дополнительная информация</p>
         </aside>
    </main>

    <!--TODO 5-->
    <footer>
        <p>&copy; 2026 «Музыкальная школа»</p>
        <a href="mailto:info@example.ru"></a>
    </footer>

    <!--TODO 6-->
    <!-- <div> — безликая обёртка для оформления, своего смысла не имеет. -->
    </div>

  </body>
</html>
```

Рисунок 2.1 - Результат выполнения программы task2.html

Именно `<div>` здесь правильно использовать потому, что у этой обёртки нет собственного смысла: она не относится ни к шапке, ни к основному содержимому, ни к подвалу, а нужна только для оформления, например для ограничения ширины страницы через CSS. Семантические теги `header`, `main` и `footer` уже заняты своими ролями, а обёртка не принадлежит ни одной из этих частей страницы, поэтому для неё подходит безликий `<div>`.

## Задание 3. Подключение CSS и базовые селекторы

Изучено подключение CSS к документу через тег `<link rel="stylesheet">` в `<head>` и базовые селекторы. Применён селектор по тегу `h1` (цвет `darkorange`, размер шрифта 28px), селектор по классу `.highlight` (фон `darkcyan`, жирное начертание) и селектор по тегу `p` (размер шрифта 20px). Добавлен селектор по идентификатору `#main-banner`, который выравнивает текст по центру и задаёт фон `cadetblue`.

На листинг 3 показано: подключение таблицы стилей и готовая разметка страницы с абзацами двух видов и блоком с id.

**Листинг 3 - Задание №3**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Практика CSS — вариант 04</title>
    <link rel="stylesheet" href="task3.css">
  </head>
  <body>

    <h1>Музыкальная школа</h1>

    <p class="highlight">Этот абзац — важный, у него есть класс highlight.</p>
    <p>Этот абзац обычный, класса у него нет.</p>
    <p class="highlight">Ещё один важный абзац с тем же классом.</p>

    <div id="main-banner">
      <p>Это уникальный блок с id, такой на странице только один.</p>
    </div>

  </body>
</html>
```

На листинг 4 показано: селекторы по тегу, по классу и по идентификатору, а также двойное объявление класса `.highlight`.

**Листинг 4 - Задание №3**

```css
/*TODO 1*/
h1 {
  color: darkorange;
  font-size: 28px;
}

/*TODO 2*/
.highlight {
  background-color: darkcyan;
  font-weight: bold;
}

/*TODO 3*/
p {
  font-size: 20px;
}

/*TODO 4*/
#main-banner {
  text-align: center;
  background-color: cadetblue;
}

/*TODO 5*/
.highlight {
  color: blue;
}
```

Рисунок 3.1 - Результат выполнения программы task3.html, task3.css

Свойства из селектора по классу и из селектора по тегу применяются к элементу `.highlight` одновременно, потому что CSS складывает их: такой абзац получает и фон `darkcyan` с жирным начертанием, и общий размер шрифта 20px от селектора `p`. Селекторы разной specificity не конфликтуют, а дополняют друг друга, поскольку задают разные свойства одного элемента.

## Задание 4. Модульная шкала размеров

Изучена модульная шкала размеров, в которой размер каждого уровня выводится из базового по формуле «размер = базовый × коэффициент ^ уровень». Принята база 17px и коэффициент 1.333. Базовому уровню 0 соответствует тег `p` с размером 17px, первому уровню – тег `h4`, второму – `h3`, третьему – `h2`, четвёртому – `h1`.

Рассчитан и применён размер уровня −1 для класса `.caption`: база делится на коэффициент один раз. Для каждого уровня расчёт записан в комментарии перед правилом, результаты округлены до целых пикселей.

На листинг 5 показано: готовая разметка с заголовками четырёх уровней, основным текстом и мелкой подписью.

**Листинг 5 - Задание №4**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Модульная шкала — вариант 04</title>
    <link rel="stylesheet" href="task4.css">
  </head>
  <body>

    <h1>Музыкальная школа — заголовок первого уровня</h1>
    <h2>Заголовок второго уровня</h2>
    <h3>Заголовок третьего уровня</h3>
    <h4>Заголовок четвёртого уровня</h4>
    <p>Основной текст — базовый размер шкалы.</p>
    <p class="caption">Мелкая подпись — на уровень меньше базового.</p>

  </body>
</html>
```

На листинг 6 показано: базовый размер для p, размеры уровней 1–4 для h4, h3, h2, h1 и размер уровня −1 для .caption с расчётами в комментариях.

**Листинг 6 - Задание №4**

```css
/*TODO 1*/
p {
  font-size: 17px;
}

/* 17 × 1.333 = 23 ~ 24 */
/*TODO 2*/
h4 {
  font-size: 24px;
}

/* 23 × 1.333 = 27 ~ 28 */
/*TODO 3*/
h3 {
  font-size: 28px;
}

/* 27 × 1.333 = 36 */
h2 {
  font-size: 36px;
}

/* 32 × 1.333 = 37 ~ 38 */
h1 {
  font-size: 38px;
}

/*TODO 4*/
/* 17 ÷ 1.333 = 12.75 ~ 14 */
.caption {
  font-size: 14px;
}
```

Рисунок 4.1 - Результат выполнения программы task4.html, task4.css

Размеры уровней −1 и 0 задают 14px и 17px, то есть подпись оказывается мельче базового текста, а заголовки h4, h3, h2, h1 последовательно увеличиваются до 24px, 28px, 36px и 38px. Важно, что уровень −1 получается делением базы на коэффициент, а не умножением, поэтому подпись всегда меньше основного текста, тогда как каждый следующий уровень заголовка получается умножением базы на коэффициент и потому последовательно больше предыдущего.

## Задание 5. Блочная модель (box model)

Изучена блочная модель элемента. Общему классу `.box` заданы `width: 260px`, `padding: 18px`, `border: 7px solid navy` и `margin: 12px`. Реальная ширина блока замерена в DevTools и составила 260px при `border-box` и 310px при `content-box`, что объясняется расчётом 260 + 2×18 + 2×7 = 310.

Для визуального различения блоков классу `.box-a` задан фон `darkorange` без явного `box-sizing`, а классу `.box-b` – `box-sizing: border-box` и фон `slateblue`. Добавлен глобальный сброс через селектор `*`, который применяет `box-sizing: border-box` сразу ко всем элементам страницы.

На листинг 7 показано: два блока с общими классами `.box`, один из которых работает в content-box, а другой в border-box.

**Листинг 7 - Задание №5**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Блочная модель — вариант 04</title>
    <link rel="stylesheet" href="task5.css">
  </head>
  <body>

    <div class="box box-a">content-box (по умолчанию)</div>
    <div class="box box-b">border-box</div>

  </body>
</html>
```

На листинг 8 показано: общие свойства блока, content-box и border-box, глобальный сброс через селектор *.

**Листинг 8 - Задание №5**

```css
/*TODO 1*/
/* Замер: 260px при border-box, 310px при content-box */
/* (260 + 2×18 + 2×7). */
.box {
    width: 260px;
    padding: 18px;
    border: 7px solid navy;
    margin: 12px;
}

/*TODO 2*/
.box-a {
    background-color: darkorange;
}

/*TODO 3*/
/* При border-box padding и border входят в width, поэтому 260px. */
.box-b {
    box-sizing: border-box;
    background-color: slateblue;
}

/*TODO 4*/
* {
    box-sizing: border-box;
}
```

Рисунок 5.1 - Результат выполнения программы task5.html, task5.css

При `border-box` padding и border входят в заданную ширину, поэтому `.box-b` остаётся ровно 260px, тогда как `.box-a` при `content-box` к 260px добавляет 2×18 padding и 2×7 border. Отступы `margin` в обеих моделях не входят в ширину элемента и только отодвигают его от соседей. После применения глобального сброса `box-sizing: border-box` действует для всех элементов страницы, поэтому реальная ширина обоих блоков одинакова.

## Задание 6. Flexbox

Изучены флексбоксы. Класс `.row` превращён в флекс-контейнер с `display: flex` и `gap: 18px`, благодаря чему три карточки встали в ряд с одинаковым промежутком. Класс `.centered` сделан флекс-контейнером высотой 240px, а карточка внутри него выровнена по центру одновременно по горизонтали и по вертикали двумя свойствами выравнивания.

Класс `.column` сделан флекс-контейнером с направлением по столбцу и `gap: 16px`. В задании на прогноз у `.row` свойство `justify-content` заменено на `space-between` при сохранённом `flex-direction: row`, и результат сравнен с `gap`.

На листинг 9 показано: три контейнера с карточками – ряд, блок для центрирования и столбец.

**Листинг 9 - Задание №6**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Flexbox — вариант 04</title>
    <link rel="stylesheet" href="task6.css">
  </head>
  <body>

    <div class="row">
      <div class="card">1</div>
      <div class="card">2</div>
      <div class="card">3</div>
    </div>

    <div class="centered">
      <div class="card">Я по центру</div>
    </div>

    <div class="column">
      <div class="card">Сверху</div>
      <div class="card">Снизу</div>
    </div>

  </body>
</html>
```

На листинг 10 показано: базовые свойства карточки, ряд с промежутком, центрирование по двум осям, столбец, сравнение gap и space-between.

**Листинг 10 - Задание №6**

```css
* {
  box-sizing: border-box;
}

.card {
  width: 80px;
  height: 80px;
  background-color: darkorange;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
}

/*TODO 1*/
.row {
    display: flex;
    gap: 18px;
}

/*TODO 2*/
.centered {
    display: flex;
    height: 240px;
    align-items: center;
    justify-content: center;
}

/*TODO 3*/
.column {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

/*TODO 4*/
/* space-between раздаёт всё свободное место, gap – только 18px. */
.row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}
```

Рисунок 6.1 - Результат выполнения программы task6.html, task6.css

Свойство `gap` задаёт промежуток только между карточками, то есть ровно 18px, а свободное место слева и справа от карточек остаётся незанятым. Свойство `space-between` раздаёт всё свободное место контейнера между карточками, поэтому при сужении экрана промежуток между ними становится больше 18px, а отступы у краёв контейнера исчезают. Прогноз полностью подтвердился при проверке в браузере.

## Задание 7. Position (relative, absolute, fixed)

Изучены три значения свойства `position`. Класс `.top-bar` получил `position: fixed` с `top: 0`, `left: 0` и `width: 100%`, поэтому шапка остаётся на месте при прокрутке страницы; под неё в `body` добавлен `padding-top: 60px`, чтобы она не перекрывала контент. Классу `.shifted` заданы `position: relative` и сдвиг `top: 35px`, `left: 40px`.

Класс `.card` получил `position: relative` без сдвигов и стал точкой отсчёта для вложенного бейджика `.badge`, которому заданы `position: absolute`, `top: -12px`, `right: -12px`, фон `darkslategray` и небольшой `padding`. В задании на прогноз строка `position: relative` у `.card` была временно отключена, изменение проверено в браузере, после чего свойство возвращено обратно.

На листинг 11 показано: фиксированная шапка, карточка с относительным позиционированием, абсолютный бейджик и сдвинутый блок.

**Листинг 11 - Задание №7**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Position — вариант 04</title>
    <link rel="stylesheet" href="task7.css">
  </head>
  <body>

    <div class="top-bar">Музыкальная школа — я всегда сверху экрана (fixed)</div>

    <div class="card">
      Карточка товара
      <span class="badge">NEW</span>
    </div>

    <div class="shifted">Сдвинутый блок (relative)</div>

  </body>
</html>
```

На листинг 12 показано: фиксированная шапка, relative как точка отсчёта, абсолютный бейджик, сдвиг relative, проверка поведения без relative.

**Листинг 12 - Задание №7**

```css
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  padding-top: 60px;
}

.card {
  width: 220px;
  height: 100px;
  margin: 40px;
  padding: 16px;
  background-color: #eee;
}

/*TODO 1*/
.top-bar {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    background-color: darkorange;
}

/*TODO 2*/
.card {
    position: relative;
}

/*TODO 3*/
.badge {
    position: absolute;
    top: -12px;
    right: -12px;
    background-color: darkslategray;
    padding: 8px;
}

/*TODO 4*/
.shifted {
    position: relative;
    top: 35px;
    left: 40px;
}

/*TODO 5*/
/* Без relative точкой отсчёта для .badge становится body, и бейджик
   уходит в правый верхний угол окна вместо угла карточки. */
```

Рисунок 7.1 - Результат выполнения программы task7.html, task7.css

Без `position: relative` у `.card` ближайшим позиционированным предком для `.badge` становится `body`, поэтому бейджик встаёт в правый верхний угол окна страницы, а не в правый верхний угол карточки. Свойство `position: relative` не выносит элемент из потока документа, поэтому после сдвига `top: 35px` и `left: 40px` исходное место блока осталось пустым, и соседние элементы под него не подвинулись.

## Задание 8. CSS Grid

Изучены CSS Grid. Класс `.gallery` превращён в грид-контейнер с тремя равными колонками `repeat(3, 1fr)` и `gap: 18px`, благодаря чему шесть плиток разложились в две строки по три. Класс `.uneven` также сделан грид-контейнером, но с колонками разной ширины `1fr 3fr 1fr`, поэтому средняя плитка втрое шире крайних.

У `.gallery` задана фиксированная высота строк `grid-template-rows: 180px 180px`. В задании на прогноз количество колонок изменено на `repeat(2, 1fr)`, а результат проверен в браузере.

На листинг 13 показано: грид из шести плиток и грид из трёх плиток разной ширины.

**Листинг 13 - Задание №8**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CSS Grid — вариант 04</title>
    <link rel="stylesheet" href="task8.css">
  </head>
  <body>

    <div class="gallery">
      <div class="tile">1</div>
      <div class="tile">2</div>
      <div class="tile">3</div>
      <div class="tile">4</div>
      <div class="tile">5</div>
      <div class="tile">6</div>
    </div>

    <div class="uneven">
      <div class="tile">узкая</div>
      <div class="tile">широкая</div>
      <div class="tile">узкая</div>
    </div>

  </body>
</html>
```

На листинг 14 показано: равные колонки, колонки разной ширины, фиксированная высота строк, смена количества колонок.

**Листинг 14 - Задание №8**

```css
* {
  box-sizing: border-box;
}

.tile {
  background-color: darkorange;
  color: white;
  padding: 20px;
  text-align: center;
}

.gallery,
.uneven {
  margin-bottom: 40px;
}

/*TODO 1*/
.gallery {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 18px;
}

/*TODO 2*/
.uneven {
    display: grid;
    grid-template-columns: 1fr 3fr 1fr;
    gap: 18px;
}

/*TODO 3*/
.gallery {
    grid-template-rows: 180px 180px;
}

/*TODO 4*/
/* 3 строки по 2 колонки; третья неявна, без grid-template-rows. */
.gallery {
    grid-template-columns: repeat(2, 1fr);
}
```

Рисунок 8.1 - Результат выполнения программы task8.html, task8.css

При `repeat(2, 1fr)` у шести плиток получается три строки по две колонки, и высота 180px остаётся только у первых двух строк, перечисленных в `grid-template-rows: 180px 180px`. Третья строка не описана в этом свойстве, поэтому она считается неявным треком, и её высота определяется контентом, а не заданным значением. Прогноз полностью подтвердился при проверке в браузере.

## Задание 9. Формы

Изучено построение форм. Создана форма с методом отправки `POST`. Поле имени реализовано как `label for="name"` и `input type="text"` с `id`, `name` и атрибутом `required`. Поле email выполнено на базе `input type="email"` с атрибутом `required` и `placeholder`, поле возраста – на базе `input type="number"` с ограничениями `min="16"` и `max="64"`.

Добавлена группа из трёх radio-кнопок для выбора тарифа «Базовый», «Стандарт» и «Премиум», у которых одинаковый `name="plan"` и разные `value` (`basic`, `standard`, `premium`), поэтому выбрать можно только один вариант. Выпадающий список оформлен тегом `select` с `name="city"` и тремя вариантами городов внутри `option`. В задании на поиск ошибки исправлена кнопка отправки.

На листинг 15 показано: текстовое поле, email, числовое поле, radio-группа из трёх вариантов, выпадающий список, исправленная кнопка отправки.

**Листинг 15 - Задание №9**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Форма — вариант 04</title>
  </head>
  <body>

    <form action="/submit" method="POST">

    <!--TODO 1-->
    <label for="name">Имя</label>
    <input type="text" id="name" name="name" required>

    <!--TODO 2-->
    <label for="email">Email</label>
    <input type="email" id="email" name="email" required placeholder="you@example.com">

    <!--TODO 3-->
    <label for="age">Возраст</label>
    <input type="number" id="age" name="age" min="16" max="64">

    <!--TODO 4-->
    <label for="plan">Тариф</label>
    <label for="plan">Базовый</label>
    <input type="radio" id="plan" name="plan" value="basic">
    <label for="plan">Стандарт</label>
    <input type="radio" id="plan" name="plan" value="standard">
    <label for="plan">Премиум</label>
    <input type="radio" id="plan" name="plan" value="premium">

    <!--TODO 5-->
    <label for="city">Город</label>
    <select id="city" name="city">
        <option value="novosibirsk">Новосибирск</option>
        <option value="volgograd">Волгоград</option>
        <option value="vladivostok">Владивосток</option>
    </select>

    <!--TODO 6-->
    <input type="button" value="Отправить" onclick="submitForm()">

    </form>

  </body>
</html>
```

Рисунок 9.1 - Результат выполнения программы task9.html

Поле с `type="email"` проверяет адрес на соответствие шаблону до отправки, а `type="number"` ограничивает ввод только числами и учитывает диапазон из `min` и `max`. Переключатели radio объединяются в одну группу общим `name`, поэтому браузер разрешает выбрать только один из них, тогда как разные `value` позволяют серверу различить варианты. Атрибут `required` делает поле обязательным, а связка `label for` и `id` позволяет щёлкнуть по подписи и сразу установить фокус в поле.

## Задание 10. Псевдоклассы и псевдоэлементы

Изучены псевдоклассы и псевдоэлементы. Псевдокласс `:hover` применён к кнопке `.btn` и при наведении меняет её фон на более тёмный `darkgoldenrod`. Псевдокласс `:focus` применён к `input` и добавляет рамку `2px solid firebrick`, перезаписывая рамку поля по умолчанию.

Для оформления списка применена псевдокласс `:nth-child(even)`, которая задаёт чётным строкам списка фон `forestgreen`, – так реализована «зебра». Псевдоэлемент `::after` применён к классу `.required` и добавляет после текста красную звёздочку, указывающую на обязательное поле.

На листинг 16 показано: кнопка, поле ввода, список из пяти пунктов и подпись обязательного поля.

**Листинг 16 - Задание №10**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Псевдоклассы — вариант 04</title>
    <link rel="stylesheet" href="task10.css">
  </head>
  <body>

    <button class="btn">Наведи на меня</button>

    <form>
      <input type="text" placeholder="Кликни сюда">
    </form>

    <ul class="list">
      <li>Первый</li>
      <li>Второй</li>
      <li>Третий</li>
      <li>Четвёртый</li>
      <li>Пятый</li>
    </ul>

    <p class="required">Заполните поле</p>

  </body>
</html>
```

На листинг 17 показано: :hover для кнопки, :focus для поля, :nth-child(even) для чётных строк, ::after для звёздочки.

**Листинг 17 - Задание №10**

```css
* {
  box-sizing: border-box;
}

.btn {
  padding: 12px 24px;
  background-color: darkorange;
  color: white;
  border: none;
}

.list {
  list-style: none;
  padding: 0;
}

.list li {
  padding: 8px;
}

/*TODO 1*/
.btn:hover {
  background-color: darkgoldenrod;
}

/*TODO 2*/
input:focus {
  border: 2px solid firebrick;
}

/*TODO 3*/
.list li:nth-child(even) {
  background-color: forestgreen;
}

/*TODO 4*/
.required::after {
  content: " *";
  color: red;
}
```

Рисунок 10.1 - Результат выполнения программы task10.html, task10.css

Псевдоклассы не добавляют элементы в разметку, а меняют стиль уже существующих элементов по состоянию, поэтому для `:hover` и `:focus` не нужно писать обработчики событий. Псевдоэлемент `::after` вставляет дополнительное содержимое, но только если обязательно указать свойство `content` – без него не появится ничего, поэтому в нём и записана звёздочка. Селектор `:nth-child(even)` считает позицию среди всех соседей, поэтому даже строки списка получают одинаковый фон, что и создаёт эффект чередования.

## Задание 11. Адаптивность (media queries)

Изучены адаптивные стили и медиазапросы. Базовое правило для `.gallery` задаёт грид-контейнер с тремя равными колонками `repeat(3, 1fr)` и `gap: 12px` без медиазапроса, то есть на широком экране. В `@media (max-width: 560px)` колонка переопределяется на одну, поэтому на узком экране плитки выстраиваются в столбец. В `@media (max-width: 440px)` для `.menu` задаётся `flex-direction: column`, и пункты меню встают друг под другом.

На листинг 18 показано: грид из трёх плиток и горизонтальное меню из трёх ссылок.

**Листинг 18 - Задание №11**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Адаптивность — вариант 04</title>
    <link rel="stylesheet" href="task11.css">
  </head>
  <body>

    <div class="gallery">
      <div class="tile">1</div>
      <div class="tile">2</div>
      <div class="tile">3</div>
    </div>

    <nav class="menu">
      <a href="#">Главная</a>
      <a href="#">О нас</a>
      <a href="#">Контакты</a>
    </nav>

  </body>
</html>
```

На листинг 19 показано: базовая сетка из трёх колонок, переход на одну колонку, переход меню в столбец, разбор срабатывания запросов.

**Листинг 19 - Задание №11**

```css
* {
  box-sizing: border-box;
}

.tile {
  background-color: teal;
  color: white;
  padding: 20px;
  text-align: center;
}

.menu {
  display: flex;
  gap: 16px;
  background-color: #333;
  padding: 12px;
}

.menu a {
  color: white;
  text-decoration: none;
}

/*TODO 1*/
.gallery {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
}

/*TODO 2*/
@media (max-width: 560px) {
  .gallery {
    grid-template-columns: 1fr;
  }
}

/*TODO 3*/
@media (max-width: 440px) {
  .menu {
    flex-direction: column;
  }
}

/*TODO 4*/
/* 460 <= 560, но 460 <= 440 ложно => сработает только TODO 2. */
```

Рисунок 11.1 - Результат выполнения программы task11.html, task11.css

При ширине экрана 460px сработает только один медиазапрос, из условии `max-width: 560px`, потому что выполняется 460 <= 560, а условие 460 <= 440 ложно, поэтому запрос для самого узкого экрана не применяется. Оба медиазапроса сработают одновременно только на экранах шириной 440px и уже, поскольку условие `max-width` означает «экран шириной не больше, чем», и чем меньше ширина, тем больше условий выполняется. Базовые правила при этом не перестают действовать, а переопределяются стилями из медиазапросов, объявленных позже.

## Итоговое задание

Тема: одностраничный сайт в стиле «Игра» – GameZone.

Свёрстана полноценная страница с использованием материала лабораторной работы. Создана семантическая структура документа: `header` с названием и `nav` из четырёх ссылок, `main` с секциями «герой», преимущества, промо-блок и форма, а также `footer` с копирайтом. Секция преимуществ содержит три карточки, в промо-блоке размещён бейджик `<span class="badge">`, который рассчитан на абсолютное позиционирование.

В форме заявки размещены четыре текстовых поля разных типов: `text`, `email`, `password` и `tel`, все с атрибутом `required` и с подсказками в `placeholder`. Поля собраны в вертикальный флексбокс шириной 420px, который по свойству `margin: 0 auto` выровнен по центру секции вместе с заголовком. Две группы radio-кнопок оформлены элементами `fieldset` с `legend`: выбор пола и выбор уровня игры, – в каждой по три варианта с одинаковым `name` и разными `value`. Форма отправляется методом `POST` кнопкой типа `submit` на всю ширину.

Оформление задано в таблице стилей: палитра в тёмных тонах с неоновыми акцентами, глобальный сброс `box-sizing: border-box`, модульная шкала размеров с базой 17px и коэффициентом 1.333, фиксированная шапка на `position: fixed`, сетка `repeat(3, 1fr)` для карточек, вертикальный флексбокс для полей и групп radio-кнопок, абсолютный бейджик внутри относительной карточки, псевдоклассы `:hover`, `:focus`, `:nth-child` и псевдоэлементы `::before`, `::after`. Плавные переходы заданы свойством `transition`: ссылки, пункты меню, карточки, промо-блок, поля формы, группы переключателей и кнопка при наведении плавно меняют цвет и сдвигаются по вертикали, а бейджик поворачивается и увеличивается. Завершают стили два медиазапроса для экранов 560px и 440px.

На листинг 20 показано: семантический каркас страницы, секции, карточки преимуществ, промо-блок с бейджиком, форма с четырьмя типами полей и двумя radio-группами.

**Листинг 20 - Итоговое задание**

```html
<!DOCTYPE html>
<html lang="ru">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GameZone — вариант 04</title>
    <link rel="stylesheet" href="task_itog.css">
  </head>
  <body>

    <header>
      <h1>GameZone</h1>
      <nav class="nav">
        <a href="#">Главная</a>
        <a href="#">О нас</a>
        <a href="#">Услуги</a>
        <a href="#">Контакты</a>
      </nav>
    </header>

    <main>

      <section class="hero">
        <p>Игры, турниры и мерч для геймеров.</p>
      </section>

      <section class="features">
        <div class="feature-card">
          <h3>Качество</h3>
          <p>Мы тщательно следим за каждой деталью.</p>
        </div>
        <div class="feature-card">
          <h3>Скорость</h3>
          <p>Быстро решаем любые вопросы клиентов.</p>
        </div>
        <div class="feature-card">
          <h3>Забота о клиенте</h3>
          <p>Индивидуальный подход к каждому.</p>
        </div>
      </section>

      <section class="promo">
        <div class="promo-card">
          <span class="badge">Топ выбор</span>
          <h2>Специальное предложение</h2>
          <p>Оставьте заявку прямо сейчас — расскажем обо всех деталях.</p>
        </div>
      </section>

      <section class="form-section">
        <h2>Оставить заявку</h2>
        <form action="/submit" method="POST">

          <input type="text" name="name" placeholder="Ваше имя" required>
          <input type="email" name="email" placeholder="you@example.com" required>
          <input type="password" name="password" placeholder="Ваш пароль" required>
          <input type="tel" name="phone" placeholder="+7 (___) ___-__-__" required>

          <fieldset class="radio-group">
            <legend>Пол</legend>
            <label><input type="radio" name="gender" value="male"> Мужской</label>
            <label><input type="radio" name="gender" value="female"> Женский</label>
            <label><input type="radio" name="gender" value="other"> Другое</label>
          </fieldset>

          <fieldset class="radio-group">
            <legend>Уровень игры</legend>
            <label><input type="radio" name="level" value="novice"> Новичок</label>
            <label><input type="radio" name="level" value="middle"> Средний</label>
            <label><input type="radio" name="level" value="pro"> Профи</label>
          </fieldset>

          <button type="submit" class="btn">Отправить заявку</button>

        </form>
      </section>

    </main>

    <footer>
      <p>&copy; 2026 GameZone. Учебный проект — лабораторная работа №2.</p>
    </footer>

  </body>
</html>
```

На листинг 21 показано: палитра и базовые стили, модульная шкала размеров, фиксированная шапка с флексбоксом, сетка карточек, промо-блок с абсолютным бейджиком, форма с псевдоклассами и два медиазапроса.

**Листинг 21 - Итоговое задание**

```css
/* --- Глобальный сброс (задание 5): border-box всем элементам,
       чтобы padding и border не увеличивали заданную ширину --- */
* {
    box-sizing: border-box;
}

/* --- Модульная шкала размеров (задание 4): база 17px,
       коэффициент 1.333 ---
   уровень −1: 17 ÷ 1.333 = 13     мелкие подписи
   уровень  0: 17                 основной текст
   уровень  1: 17 × 1.333 = 23    h3 и текст в форме
   уровень  2: 17 × 1.333² = 30   h2 и текст в шапке-герое
   уровень  3: 17 × 1.333³ = 40   h1
*/

/* --- Базовые стили страницы --- */
body {
    margin: 0;
    padding-top: 80px;
    font-family: 'Segoe UI', Tahoma, sans-serif;
    font-size: 17px;
    line-height: 1.5;
    color: #e8ecff;
    background-color: #0d1020;
}

h1 {
    font-size: 40px;
    margin: 0;
    letter-spacing: 2px;
    text-transform: uppercase;
    text-shadow: 0 0 18px rgba(0, 229, 255, 0.55);
}

h2 {
    font-size: 30px;
    margin: 0 0 16px;
}

h3 {
    font-size: 23px;
    margin: 0 0 8px;
    color: #00e5ff;
}

p {
    margin: 0 0 16px;
}

a {
    color: #00e5ff;
    text-decoration: none;
    transition: color 0.25s ease;
}

/* --- Шапка: фиксированная (задание 7) + flex (задание 6) --- */
header {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    padding: 16px 32px;
    background-color: #171b31;
    border-bottom: 2px solid #00e5ff;
}

.nav {
    display: flex;
    gap: 24px;
}

.nav a {
    color: #e8ecff;
    padding: 8px 12px;
    border: 1px solid transparent;
    border-radius: 8px;
    transition: color 0.25s ease, background-color 0.25s ease,
                border-color 0.25s ease, transform 0.25s ease;
}

.nav a:hover {
    color: #0d1020;
    background-color: #00e5ff;
    border-color: #00e5ff;
    transform: translateY(-2px);
}

/* --- Основное содержимое --- */
main {
    max-width: 1100px;
    margin: 0 auto;
    padding: 0 32px 64px;
}

/* --- Первый экран --- */
.hero {
    margin: 48px 0;
    padding: 56px 32px;
    text-align: center;
    background: linear-gradient(135deg, #171b31, #241a4d);
    border: 1px solid #2c3566;
    border-radius: 16px;
    transition: border-color 0.3s ease, box-shadow 0.3s ease;
}

.hero:hover {
    border-color: #00e5ff;
    box-shadow: 0 0 40px rgba(0, 229, 255, 0.18);
}

.hero p {
    font-size: 30px;
    margin: 0;
    color: #cfd6ff;
}

.hero p::before {
    content: "▶ ";
    color: #ff2e88;
}

.hero p::after {
    content: " ◀";
    color: #ff2e88;
}

/* --- Преимущества: grid (задание 8) --- */
.features {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
    margin-bottom: 48px;
}

.feature-card {
    padding: 24px;
    background-color: #171b31;
    border: 1px solid #2c3566;
    border-top: 4px solid #00e5ff;
    border-radius: 16px;
    transition: border-color 0.3s ease, transform 0.3s ease,
                box-shadow 0.3s ease;
}

.feature-card p {
    margin: 0;
    color: #a9b1d9;
}

/* :nth-child(odd) — вторая карточка с другим цветом акцента */
.feature-card:nth-child(2) {
    border-top-color: #ff2e88;
}

.feature-card:hover {
    border-color: #00e5ff;
    transform: translateY(-6px);
    box-shadow: 0 14px 34px rgba(0, 229, 255, 0.25);
}

/* --- Промо-блок: relative + absolute (задание 7) --- */
.promo {
    margin-bottom: 48px;
}

.promo-card {
    position: relative;
    padding: 40px 32px;
    background: linear-gradient(120deg, #3a1060, #171b31);
    border: 1px solid #ff2e88;
    border-radius: 16px;
    transition: box-shadow 0.3s ease;
}

.promo-card:hover {
    box-shadow: 0 14px 34px rgba(255, 46, 136, 0.3);
}

.badge {
    position: absolute;
    top: -14px;
    right: -14px;
    padding: 8px 16px;
    font-size: 13px;
    color: #ffffff;
    background-color: #ff2e88;
    border-radius: 999px;
    letter-spacing: 1px;
    text-transform: uppercase;
    transition: transform 0.3s ease, box-shadow 0.3s ease;
}

/* бейджик поворачивается и увеличивается при наведении на карточку */
.promo-card:hover .badge {
    transform: rotate(6deg) scale(1.06);
    box-shadow: 0 0 18px rgba(255, 46, 136, 0.7);
}

/* --- Форма: вертикальный flex (задание 6) + псевдоклассы (задание 10) --- */
.form-section {
    padding: 32px;
    background-color: #171b31;
    border: 1px solid #2c3566;
    border-radius: 16px;
}

.form-section h2 {
    text-align: center;
}

.form-section h2::after {
    content: " — поля обязательны";
    font-size: 13px;
    color: #ff2e88;
    letter-spacing: 1px;
    text-transform: uppercase;
}

form {
    display: flex;
    flex-direction: column;
    gap: 14px;
    width: 100%;
    max-width: 420px;
    margin: 0 auto;
}

form input[type="text"],
form input[type="email"],
form input[type="password"],
form input[type="tel"] {
    width: 100%;
    padding: 14px 16px;
    font-family: inherit;
    font-size: 17px;
    color: #e8ecff;
    background-color: #0d1020;
    border: 2px solid #2c3566;
    border-radius: 10px;
    transition: border-color 0.25s ease, box-shadow 0.25s ease,
                background-color 0.25s ease;
}

form input::placeholder {
    color: #6b74a8;
}

form input:hover {
    border-color: #4b5690;
}

form input:focus {
    border-color: #00e5ff;
    box-shadow: 0 0 0 3px rgba(0, 229, 255, 0.25);
    outline: none;
}

/* --- Группы radio-кнопок --- */
.radio-group {
    display: flex;
    flex-direction: column;
    gap: 10px;
    margin: 6px 0 0;
    padding: 14px 18px 18px;
    border: 1px solid #2c3566;
    border-radius: 10px;
    transition: border-color 0.25s ease, box-shadow 0.25s ease;
}

.radio-group:hover {
    border-color: #ff2e88;
    box-shadow: 0 0 0 3px rgba(255, 46, 136, 0.12);
}

.radio-group legend {
    padding: 0 8px;
    font-size: 13px;
    color: #a9b1d9;
    letter-spacing: 1px;
    text-transform: uppercase;
}

.radio-group label {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    transition: color 0.25s ease;
}

.radio-group label:hover {
    color: #00e5ff;
}

form input[type="radio"] {
    width: 18px;
    height: 18px;
    accent-color: #ff2e88;
}

.btn {
    width: 100%;
    margin-top: 6px;
    padding: 16px 24px;
    font-family: inherit;
    font-size: 17px;
    color: #0d1020;
    background-color: #00e5ff;
    border: none;
    border-radius: 10px;
    letter-spacing: 1px;
    text-transform: uppercase;
    cursor: pointer;
    transition: color 0.25s ease, background-color 0.25s ease,
                transform 0.25s ease, box-shadow 0.25s ease;
}

.btn:hover {
    color: #ffffff;
    background-color: #ff2e88;
    transform: translateY(-2px);
    box-shadow: 0 10px 26px rgba(255, 46, 136, 0.45);
}

.btn:active {
    transform: translateY(0);
}

/* --- Подвал --- */
footer {
    padding: 24px 32px;
    font-size: 13px;
    text-align: center;
    color: #6b74a8;
    background-color: #0a0c18;
    border-top: 1px solid #2c3566;
}

/* --- Адаптивность: media queries (задание 11) --- */

/* 560px и уже: одна колонка, шапка в столбец */
@media (max-width: 560px) {
    body {
        padding-top: 150px;
    }

    header {
        flex-direction: column;
        align-items: flex-start;
    }

    .features {
        grid-template-columns: 1fr;
    }
}

/* 440px и уже: заголовки меньше, меню в столбец */
@media (max-width: 440px) {
    h1 {
        font-size: 30px;
    }

    .hero p {
        font-size: 23px;
    }

    .nav {
        flex-direction: column;
        gap: 8px;
    }
}

```

Рисунок 12.1 - Результат выполнения программы task_itog.html, task_itog.css

Итоговое задание объединило материал лабораторной работы в одном документе: семантическую вёрстку для разделов страницы, блочную модель с глобальным сбросом `box-sizing`, чтобы padding и border не увеличивали заданную ширину элементов, а также приёмы позиционирования, которые позволили вынести бейджик «Топ выбор» в правый верхний угол промо-карточки. Форма собрана по правилам проверки данных: типы полей различают текст, адрес электронной почты, пароль и телефон, а объединение radio-кнопок общим `name` внутри каждой группы не даёт выбрать сразу несколько вариантов. Плавность интерфейса задана свойством `transition`: без него смена цвета и сдвиг при наведении происходили бы мгновенно, а с ним изменения проходят плавно за доли секунды. Медиазапросы на 560px и 440px перестраивают сетку карточек в одну колонку, а меню – в столбец, причём на самом узком экране срабатывают оба запроса сразу.

## Заключение

В ходе лабораторной работы изучены и применены на практике основы вёрстки на HTML и CSS. Освоена структура HTML-документа и базовые теги: заголовки и абзацы, маркированные и нумерованные списки, ссылки и изображения, а также различие строчного тега `span` и блочного тега `div` и порядок закрытия вложенных тегов. Изучены семантические теги `header`, `nav`, `main`, `article`, `aside` и `footer`, а также обоснован выбор безликого `div` для оформления.

Изучены подключение таблицы стилей и базовые селекторы по тегу, классу и идентификатору, их совместная работа и сложение свойств. Рассмотрена модульная шкала размеров, в которой уровень −1 получается делением базы на коэффициент, а каждый следующий уровень – умножением. Разобрана блочная модель: различие `content-box` и `border-box` даёт для одного и того же блока разную ширину, а отступы `margin` не входят в ширину элемента ни в одной из моделей. Освоены флексбоксы для ряда, столбца и центрирования, различие `gap` и `space-between`, свойства `position` `fixed`, `relative` и `absolute`, а также двумерная сетка CSS Grid с равными и разными колонками, фиксированной высотой строк и неявными треками. Изучены построение форм и типы полей, псевдоклассы `:hover`, `:focus` и `:nth-child(even)`, псевдоэлемент `::after` и адаптивность через медиазапросы.

Все полученные знания применены в итоговом задании при вёрстке одностраничного сайта GameZone, где собраны семантический каркас страницы, секции, карточки, промо-блок с бейджиком и форма заявки с полями разных типов. Практика показала, что понимание того, как браузер складывает правила, как он считает ширину элемента и в каком порядке применяются медиазапросы, необходимо для предсказуемого результата вёрстки.
