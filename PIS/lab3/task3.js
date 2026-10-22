// ЛАБОРАТОРНАЯ РАБОТА №3
// ЗАДАНИЕ №3: стили через JS — Вариант 04
// Тема: Спортивный клуб «Атлет». Откройте var04.html через Live Server.

// --- Лёгкий уровень ---

// TODO 1: element.style
// Получите #js-box и через style.backgroundColor задайте ему фон "orange"

// Подсказка: element.style.свойство = "значение" — имя свойства пишется
// camelCase (backgroundColor), а не через дефис, как в CSS-файле
// (background-color).
const jsBox = document.getElementById("js-box");
jsBox.style.backgroundColor = "orange";


// TODO 2: element.style + единицы измерения
// Задайте #js-box ширину 150px и высоту 80px через style.width / style.height
// (не забудьте "px" в строке!)

// Подсказка: box.style.width = 150 (число без единиц) просто не сработает —
// нужна строка с единицей измерения: box.style.width = "150px".
jsBox.style.width = "150px";
jsBox.style.height = "80px";



// --- Средний уровень ---

// TODO 3: несколько свойств сразу
// Задайте #js-box borderRadius "10px" и border "2px solid black"

// Подсказка: просто две отдельные строки кода, каждая — своё свойство style,
// как в TODO 1 и 2, друг за другом.
jsBox.style.borderRadius = "10px";
jsBox.style.border = "2px solid black"; 


// TODO 4: element.style на элементе со стилями из CSS-класса
// Получите #preset-box (у него стиль задан через class="preset" в CSS).
// Выведите в консоль preset-box.style.backgroundColor — что вывелось и почему?

// Подсказка: element.style показывает ТОЛЬКО инлайн-стили (заданные прямо
// через JS или атрибут style="..."). Стили из CSS-класса через него не видны —
// поэтому результат может вас удивить.
//
// ВАЖНО: здесь НЕ задаём стиль через JS — иначе в TODO 5 getComputedStyle
// показал бы уже наш собственный цвет, а не цвет из CSS-класса .preset.
const presetBox = document.querySelector("#preset-box");
console.log(presetBox.style.backgroundColor);
// Вывелась пустая строка (""): у #preset-box нет инлайн-стилей, весь вид
// задаёт CSS-класс .preset из <style> — в element.style попадают только
// стили, записанные прямо в атрибуте style="..." или через JS.


// --- Сложный уровень ---

// TODO 5: getComputedStyle
// Через getComputedStyle(document.querySelector("#preset-box")) получите
// РЕАЛЬНЫЙ применённый backgroundColor и выведите в консоль. Сравните с TODO 4

// Подсказка: getComputedStyle(element) — это обычная функция (не метод
// самого элемента!), вызывается отдельно: getComputedStyle(el), а не
// el.getComputedStyle().
console.log(getComputedStyle(presetBox).backgroundColor);


// TODO 6: getComputedStyle — ширина
// Тем же способом получите и выведите реальную ширину (width) #preset-box

// Подсказка: getComputedStyle(element) возвращает объект со ВСЕМИ реальными
// применёнными стилями, независимо от того, инлайн они, из класса или из
// стилей браузера по умолчанию.
console.log(getComputedStyle(presetBox).width);


// --- Задача уровня собеседования ---

// TODO 7: element.style vs getComputedStyle
// Опишите в комментарии (2-3 предложения): почему в TODO 4
// element.style.backgroundColor вывел пустую строку, а в TODO 5
// getComputedStyle вывел реальный цвет? В чём принципиальная разница между
// этими двумя способами?

// Ответ: element.style — это объект ИНЛАЙН-стилей элемента, то есть только
// того, что задано атрибутом style="..." или через JS (в TODO 4 инлайн-стилей
// не было — вывод пустой). getComputedStyle(el) возвращает итоговые значения
// после применения ВСЕХ источников: CSS-классов, наследования, стилей
// браузера, поэтому в TODO 5 получился реальный teal из класса .preset.
// Читать реальные значения нужно через getComputedStyle, а записывать —
// через element.style.


