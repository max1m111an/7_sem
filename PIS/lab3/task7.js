// ЛАБОРАТОРНАЯ РАБОТА №3
// ЗАДАНИЕ №7: работа с формами — Вариант 04
// Тема: Спортивный клуб «Атлет». Откройте var04.html через Live Server.

const form = document.querySelector("#reg-form");

// --- Лёгкий уровень ---

// TODO 1: текстовое поле
// Получите #username-field. Подпишитесь на "submit" формы (не забудьте
// event.preventDefault()), внутри выведите в консоль username.value

// Подсказка: событие "submit" вешается на САМУ ФОРМУ (form, которая уже
// получена в переменную form выше), а не на поле ввода — у полей ввода
// такого события нет, оно не сработает.

const username = document.querySelector("#username-field");
const agreeCheckbox = document.querySelector("#agree-checkbox");
const countrySelect = document.querySelector("#country-select");
const errorBox = document.querySelector("#error-message");

form.addEventListener("submit", function (event) {
    event.preventDefault();

    // TODO 5: валидация — обязательное поле
    if (username.value === "") {
        errorBox.textContent = "Введите имя";
        return;                       // дальше код выполняться не должен
    }

    // TODO 6: валидация — согласие с условиями
    if (!agreeCheckbox.checked) {
        errorBox.textContent = "Нужно согласиться с условиями";
        return;
    }
    errorBox.textContent = "";       // обе проверки пройдены — ошибок нет

    // TODO 1: текстовое поле
    console.log(username.value);

    // TODO 2: чекбокс — .checked, а не .value!
    console.log(agreeCheckbox.checked);

    // TODO 3: радиокнопки — :checked
    const accountType = document.querySelector('input[name="account-type"]:checked');
    console.log(accountType ? accountType.value : "ничего не выбрано");

    // TODO 4: select
    console.log(countrySelect.value);
});


// --- Средний уровень ---

// TODO 2: чекбокс — .checked, а не .value!
// В ТОМ ЖЕ обработчике получите #agree-checkbox и выведите в консоль его .checked

// Подсказка: checkbox.value почти всегда просто "on" — бесполезно. Нужно
// именно .checked (true/false)

// Значение .checked выводится в обработчике выше (console.log(agreeCheckbox.checked)).
// У чекбокса .value почти всегда равен "on" — это значение атрибута value,
// а не состояние, поэтому для проверки «отмечен/не отмечен» нужен .checked.


// TODO 3: радиокнопки — :checked
// В ТОМ ЖЕ обработчике получите выбранную радиокнопку через
// document.querySelector('input[name="account-type"]:checked')
// и выведите в консоль её .value

// Подсказка: 'input[name="account-type"]:checked' — это ОДИН CSS-селектор:
// сначала фильтр по атрибуту name, затем псевдокласс :checked (только
// отмеченный элемент). querySelector найдёт ровно ту радиокнопку, что выбрана.

// Выбор радиокнопки читается в обработчике выше:
// const accountType = document.querySelector('input[name="account-type"]:checked');
// console.log(accountType.value);


// TODO 4: select
// В ТОМ ЖЕ обработчике получите #country-select и выведите в консоль его .value

// Подсказка: у select всё так же просто, как у текстового поля — обычное
// .value, никаких новых приёмов не нужно.

// Значение select читается в обработчике выше: console.log(countrySelect.value).


// --- Сложный уровень ---

// TODO 5: валидация — обязательное поле
// В САМОМ НАЧАЛЕ обработчика (сразу после preventDefault) проверьте: если
// #username-field.value === "" — запишите в #error-message textContent
// "Введите имя" и сделайте return (дальше код выполняться не должен)

// Подсказка: тот же принцип, что и с null-проверками в теме 1 — если условие
// (пустая строка) верно, выполняем действие и сразу return, чтобы код ниже
// не выполнился на невалидных данных.

// Проверка стоит первой строкой после event.preventDefault() — поэтому при
// пустом имени обработчик сразу выходит и остальные проверки не выполняются.


// TODO 6: валидация — согласие с условиями
// После проверки из TODO 5 (если поле не пустое) добавьте вторую проверку:
// если #agree-checkbox НЕ отмечен — запишите в #error-message
// "Нужно согласиться с условиями" и return. Если обе проверки прошли —
// очистите #error-message (errorBox.textContent = "")

// Подсказка: проверки идут одна за другой — сначала имя, потом чекбокс.
// Если первая проверка сработала (return) — до второй дело не дойдёт.

// Вторая проверка — следом за первой (см. обработчик выше). Если обе
// проверки пройдены, errorBox.textContent = "" очищает старую ошибку.


// --- Задача уровня собеседования ---

// TODO 7: почему не .value у чекбокса
// Опишите в комментарии (2-3 предложения): почему для чекбокса используют
// .checked, а не .value, хотя у текстового поля именно .value? Что вообще
// хранится в .value у чекбокса, и почему оно бесполезно для проверки
// "отмечен/не отмечен"?

// Ответ: .value у чекбокса — это значение атрибута value (по умолчанию "on"),
// оно задаётся разработчиком и не меняется при клике, поэтому о состоянии
// ничего не говорит. .checked — это как раз текущее состояние (true/false).
// У текстового поля .value хранит сам введённый текст, поэтому там оно и
// полезно.
