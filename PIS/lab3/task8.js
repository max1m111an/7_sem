// ЛАБОРАТОРНАЯ РАБОТА №3
// ЗАДАНИЕ №8: таймеры (setTimeout, setInterval) — Вариант 04
// Тема: Спортивный клуб «Атлет». Откройте var04.html через Live Server.

// --- Лёгкий уровень ---

// TODO 1: setTimeout — уведомление само исчезает
// Получите #toast-btn и #toast. По клику на кнопку уберите у #toast класс
// "hidden" (появится), а через setTimeout — верните класс "hidden" обратно
// через 3000мс (3 секунды)

// Подсказка: setTimeout(function() { ... }, 3000) — функция внутри
// выполнится ОДИН РАЗ, через 3 секунды после вызова setTimeout, а не сразу.
// Показать/скрыть — это classList.remove("hidden") / classList.add("hidden"),
// а не .remove() самого элемента (это удалило бы его из DOM насовсем).

const toastBtn = document.querySelector("#toast-btn");
const toast = document.querySelector("#toast");

toastBtn.addEventListener("click", function () {
    toast.classList.remove("hidden");            // показать
    setTimeout(function () {
        toast.classList.add("hidden");           // скрыть через 3 секунды
    }, 3000);
});


// TODO 2: setInterval — живые часы
// Получите #clock. Через setInterval КАЖДУЮ секунду (1000мс) записывайте в
// него текущее время: new Date().toLocaleTimeString()

// Подсказка: new Date() создаёт объект с текущей датой/временем прямо в
// момент вызова — вызывать его нужно ВНУТРИ функции таймера (не один раз
// снаружи), чтобы время реально обновлялось на каждом тике.

const clock = document.querySelector("#clock");

setInterval(function () {
    clock.textContent = new Date().toLocaleTimeString();
}, 1000);


// --- Средний уровень ---

// TODO 3: setInterval — обратный отсчёт
// Получите #countdown и #start-btn. По клику на "Старт" запустите
// setInterval (1000мс), который каждый раз уменьшает число в #countdown на 1
// (Number(countdownEl.textContent) - 1, записать обратно через textContent)

// Подсказка: id таймера нужно сохранить в переменную ДО того, как он вам
// понадобится (см. TODO 5) — объявите её ВНЕ функций-обработчиков.

// id интервала объявляем ВНЕ обработчиков — тогда он доступен и в TODO 3/4,
// и в TODO 5, и в TODO 6.
let countdownIntervalId = null;
const countdown = document.querySelector("#countdown");
const startBtn = document.querySelector("#start-btn");

startBtn.addEventListener("click", function () {
    if (countdownIntervalId !== null) {
        clearInterval(countdownIntervalId);      // гасим старый интервал
    }

    countdownIntervalId = setInterval(function () {
        let value = Number(countdown.textContent) - 1;
        countdown.textContent = value;

        // TODO 4: остановка при достижении нуля
        if (value <= 0) {
            clearInterval(countdownIntervalId);
            countdownIntervalId = null;
        }
    }, 1000);
});


// TODO 4: остановка при достижении нуля
// В ТОМ ЖЕ интервале из TODO 3 — после уменьшения проверьте: если число
// стало 0 (или меньше), остановите сам интервал через clearInterval
// (используя id, который вернул setInterval при запуске)

// Подсказка: clearInterval(id) вызывается с тем же id, что вернул setInterval —
// без сохранённого id остановить интервал нечем.

// Проверка if (value <= 0) стоит прямо внутри интервала из TODO 3:
// при достижении нуля вызывается clearInterval(countdownIntervalId) и
// счётчик больше не тикает.


// --- Сложный уровень ---

// TODO 5: кнопка "Стоп"
// Получите #stop-btn. По клику остановите текущий интервал через
// clearInterval, используя тот же id, что и в TODO 3/4 (переменная должна
// быть объявлена ВНЕ функций-обработчиков, чтобы быть доступной во всех их)

const stopBtn = document.querySelector("#stop-btn");

stopBtn.addEventListener("click", function () {
    if (countdownIntervalId !== null) {
        clearInterval(countdownIntervalId);      // останавливаем отсчёт
        countdownIntervalId = null;
    }
});


// TODO 6: кнопка "Сброс"
// Получите #reset-btn. По клику остановите интервал (как в TODO 5) И
// верните #countdown.textContent обратно в "10"

// Подсказка: на всякий случай перед КАЖДЫМ новым запуском setInterval (в
// TODO 3) стоит сначала останавливать предыдущий, если он ещё жив —
// иначе при повторном нажатии "Старт" получится два интервала одновременно,
// и счётчик будет "скакать" быстрее.

const resetBtn = document.querySelector("#reset-btn");

resetBtn.addEventListener("click", function () {
    if (countdownIntervalId !== null) {
        clearInterval(countdownIntervalId);      // останавливаем интервал
        countdownIntervalId = null;
    }
    countdown.textContent = "10";                // возвращаем исходное значение
});


// --- Задача уровня собеседования ---

// TODO 7: зачем сохранять id таймера
// Опишите в комментарии (2-3 предложения): зачем обязательно сохранять то,
// что вернул setInterval/setTimeout, в переменную? Что произойдёт, если
// несколько раз запустить setInterval для одного и того же счётчика, не
// останавливая предыдущие через clearInterval?

// Ответ: setInterval/setTimeout возвращают числовой id, и только он позволяет
// потом вызвать clearInterval(id)/clearTimeout(id) и остановить таймер. Если
// id не сохранить, остановить таймер просто нечем. А если запустить интервал
// несколько раз, не гася предыдущие, счётчик будет уменьшаться сразу
// несколькими таймерами — то есть слишком быстро.
