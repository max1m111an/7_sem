// ЛАБОРАТОРНАЯ РАБОТА №3
// ЗАДАНИЕ №10: JSON (JSON.stringify / JSON.parse) — Вариант 04
// Тема: Спортивный клуб «Атлет». Откройте var04.html через Live Server.

// --- Лёгкий уровень ---

// TODO 1: JSON.stringify — объект в строку
// Создайте обычный объект const person = { name: "Аня", age: 20 };
// Выведите в консоль сам объект (console.log(person)), а затем
// console.log(JSON.stringify(person)) — сравните, как они выглядят в консоли

// Подсказка: у JSON-строки ключи в кавычках ("name", а не name), и вся
// структура — это ОДНА строка текста, а не объект.

const person = { name: "Аня", age: 20 };
console.log(person);
console.log(JSON.stringify(person));
// В консоли видно разницу: первый вывод — живой объект (в консоли браузера
// он раскрывается в сворачиваемое дерево), второй — одна строка
// '{"name":"Аня","age":20}', где ключи в кавычках.


// TODO 2: JSON.parse — строка в объект
// Создайте строку const raw = '{"name":"Аня","age":20}';
// Через JSON.parse(raw) получите из неё реальный объект, выведите его в
// консоль и отдельно выведите parsed.name (докажите, что это теперь
// настоящий объект)

const raw = '{"name":"Аня","age":20}';
const parsed = JSON.parse(raw);
console.log(parsed);
console.log(parsed.name);
// parsed.name — это строка "Аня": значит, JSON.parse действительно вернул
// объект, к которому можно обращаться по свойствам.


// --- Средний уровень ---

// TODO 3: сохранение в localStorage
// Получите #name-field, #age-field, #save-btn. По клику на кнопку:
// соберите объект { name: nameField.value, age: ageField.value },
// превратите его в строку через JSON.stringify и сохраните через
// localStorage.setItem("profile", ...)

// Подсказка: localStorage.setItem("ключ", "значение") — оба аргумента строки.
// Объект без JSON.stringify не сохранится корректно (см. TODO 5 — там как
// раз демонстрация этой ошибки).

const nameField = document.querySelector("#name-field");
const ageField = document.querySelector("#age-field");
const saveBtn = document.querySelector("#save-btn");
const output = document.querySelector("#output");

saveBtn.addEventListener("click", function () {
    const profile = {
        name: nameField.value,
        age: ageField.value
    };

    localStorage.setItem("profile", JSON.stringify(profile));
    console.log("Сохранено:", localStorage.getItem("profile"));
});


// TODO 4: загрузка из localStorage
// Получите #load-btn. По клику: localStorage.getItem("profile") достаёт
// строку, JSON.parse превращает её обратно в объект, запишите в #output
// что-то вродое "Имя: " + parsed.name + ", возраст: " + parsed.age

// Подсказка: если ничего не сохраняли — getItem вернёт null. Проверьте
// результат через if перед использованием (parsed.name), иначе можно
// получить ошибку при обращении к свойству null.

const loadBtn = document.querySelector("#load-btn");

loadBtn.addEventListener("click", function () {
    const saved = localStorage.getItem("profile");

    if (!saved) {                       // ничего не сохраняли — getItem вернул null
        output.textContent = "Данные ещё не сохранены";
        return;
    }

    const data = JSON.parse(saved);
    output.textContent = "Имя: " + data.name + ", возраст: " + data.age;
});


// --- Сложный уровень ---

// TODO 5: демонстрация ошибки — сохранение БЕЗ stringify
// Получите #bad-save-btn. По клику сохраните объект НАПРЯМУЮ, без
// JSON.stringify: localStorage.setItem("bad", { foo: "bar" })
// (да, специально неправильно — это часть задания, чтобы увидеть результат)

const badSaveBtn = document.querySelector("#bad-save-btn");

badSaveBtn.addEventListener("click", function () {
    // Специально БЕЗ JSON.stringify — так localStorage превратит объект
    // в строку "[object Object]".
    localStorage.setItem("bad", { foo: "bar" });
    console.log("Сохранено:", localStorage.getItem("bad"));
});


// TODO 6: попытка прочитать это же значение
// Получите #bad-load-btn. По клику получите localStorage.getItem("bad") и
// выведите в консоль — увидите строку "[object Object]", а не рабочие
// данные. Оберните JSON.parse этой строки в try/catch — в catch выведите в
// #bad-output текст "Не удалось разобрать JSON: испорченные данные"

// Подсказка: JSON.parse("[object Object]") выбросит SyntaxError, т.к. это
// не валидный JSON — вот почему try/catch тут обязателен, если данные могут
// быть повреждены.

const badLoadBtn = document.querySelector("#bad-load-btn");
const badOutput = document.querySelector("#bad-output");

badLoadBtn.addEventListener("click", function () {
    const saved = localStorage.getItem("bad");
    console.log(saved);                 // это строка "[object Object]"

    try {
        const data = JSON.parse(saved);
        badOutput.textContent = "Неожиданно разобралось: " + JSON.stringify(data);
    } catch (error) {
        badOutput.textContent = "Не удалось разобрать JSON: испорченные данные";
    }
});


// --- Задача уровня собеседования ---

// TODO 7: почему localStorage нужен именно JSON
// Опишите в комментарии (2-3 предложения): почему localStorage вообще
// требует JSON.stringify/JSON.parse — что будет, если положить туда объект
// напрямую, и почему localStorage в принципе не может хранить "живые"
// JS-объекты?

// Ответ: localStorage хранит только строки, поэтому объект, положенный
// напрямую, неявно превращается в "[object Object]" — восстановить из этой
// строки данные уже нельзя. JSON.stringify и JSON.parse — это способ
// превратить объект в строку и обратно без потери данных (сериализация).
// «Живые» JS-объекты (ссылки, методы, функции) в localStorage сохранить
// невозможно: там лежит только текст.
