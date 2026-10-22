// ЛАБОРАТОРНАЯ РАБОТА №3
// ЗАДАНИЕ №9: Fetch API — Вариант 04
// Тема: Спортивный клуб «Атлет». Откройте var04.html через Live Server.
// Используем бесплатный тестовый API: https://jsonplaceholder.typicode.com
// (специально сделан для учебных запросов, не требует ключей/регистрации)

// --- Лёгкий уровень ---

// TODO 1: fetch + .then + .catch — список пользователей
// Получите #load-users-btn. По клику сделайте:
// fetch("https://jsonplaceholder.typicode.com/users")
//   .then(response => response.json())
//   .then(data => console.log(data))
//   .catch(error => console.log("Ошибка:", error));

// Подсказка: response.json() тоже возвращает Promise — поэтому и нужны ДВА
// .then() подряд: первый разбирает "конверт" (response), второй получает
// уже готовые данные.

const loadUsersBtn = document.querySelector("#load-users-btn");
const usersResult = document.querySelector("#users-result");

loadUsersBtn.addEventListener("click", function () {
    fetch("https://jsonplaceholder.typicode.com/users")
        .then(function (response) {
            return response.json();
        })
        .then(function (data) {
            console.log(data);

            // TODO 2: покажите количество на странице
            usersResult.textContent = "Пользователей: " + data.length;
        })
        .catch(function (error) {
            console.log("Ошибка:", error);
        });
});


// --- Средний уровень ---

// TODO 2: покажите количество на странице
// В том же .then(data => ...) из TODO 1 запишите в #users-result
// текст вида "Пользователей: " + data.length (через textContent)

// Количество записывается в #users-result прямо во втором .then() из
// TODO 1: usersResult.textContent = "Пользователей: " + data.length;


// TODO 3: async/await — один пользователь
// Напишите async-функцию loadSingleUser(), которая:
// 1) await fetch("https://jsonplaceholder.typicode.com/users/1")
// 2) await response.json()
// 3) записывает data.name в #single-result через textContent
// Всё оберните в try/catch (catch пока просто console.log(error))

// Подсказка: слово async ставится перед function, await — перед fetch(...)
// и перед response.json(). Без async перед функцией await использовать нельзя.

const singleResult = document.querySelector("#single-result");

async function loadSingleUser() {
    try {
        const response = await fetch("https://jsonplaceholder.typicode.com/users/1");
        const data = await response.json();
        singleResult.textContent = data.name;
    } catch (error) {
        console.log(error);
    }
}


// TODO 4: подключите функцию к кнопке
// Получите #load-single-btn и повесьте на неё "click", который вызывает
// loadSingleUser() из TODO 3

const loadSingleBtn = document.querySelector("#load-single-btn");

loadSingleBtn.addEventListener("click", loadSingleUser);


// --- Сложный уровень ---

// TODO 5: обработка HTTP-ошибки (404)
// Напишите async-функцию loadBadUser(), которая делает
// await fetch("https://jsonplaceholder.typicode.com/users/9999") (несуществующий id).
// После получения response проверьте: if (!response.ok) — сделайте
// throw new Error("Пользователь не найден: " + response.status)
// В catch — запишите текст ошибки (error.message) в #error-result

// Подсказка: fetch НЕ переходит в catch сам по себе на 404 — сервер всё
// равно "ответил", просто с кодом ошибки. response.ok — это то единственное,
// что скажет вам, был ли ответ реально успешным (200-299) или нет.

const errorResult = document.querySelector("#error-result");

async function loadBadUser() {
    try {
        const response = await fetch("https://jsonplaceholder.typicode.com/users/9999");

        if (!response.ok) {
            throw new Error("Пользователь не найден: " + response.status);
        }

        const data = await response.json();
        errorResult.textContent = "Загружен: " + data.name;
    } catch (error) {
        errorResult.textContent = error.message;
    }
}


// TODO 6: подключите к кнопке
// Получите #load-bad-btn и повесьте на неё "click", который вызывает
// loadBadUser() из TODO 5

const loadBadBtn = document.querySelector("#load-bad-btn");

loadBadBtn.addEventListener("click", loadBadUser);


// --- Задача уровня собеседования ---

// TODO 7: почему .catch() одного не хватает
// Опишите в комментарии (2-3 предложения): почему просто fetch(...).catch(...)
// не поймает ситуацию "пользователь не найден" (404)? Что вообще считается
// "ошибкой" с точки зрения fetch, а что — просто обычным (хоть и неудачным)
// ответом сервера?

// Ответ: .catch() у fetch срабатывает только при СЕТЕВЫХ ошибках (сервер
// недоступен, соединение оборвалось). Ответ 404 — это обычный успешный
// ответ: тело прочитать можно, просто статус не 2xx. Поэтому проверить
// response.ok нужно вручную, иначе «пользователь не найден» попадёт в
// .then как обычные данные, а не в .catch.
