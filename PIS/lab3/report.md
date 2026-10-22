# Лабораторная работа №3

DOM, события, формы, таймеры, Fetch API и JSON. Вариант 04. Отчёт по проделанной работе.

## Задание 1. DOM и выбор элементов

Изучены способы выбора элементов DOM. Заголовок `#main-title` получен через `document.getElementById`, первый элемент с классом `.card` – через `document.querySelector(".card")`. Все карточки получены через `document.querySelectorAll(".card")`, а их количество выведено по свойству `length` (3 элемента).

Для каждой карточки выполнен вложенный поиск через `card.querySelector(".card-title")` внутри `forEach`, поэтому выведены заголовки всех трёх карточек, а не только первой. Кнопка `#buy-btn` найдена через `querySelector`, а для несуществующего `#not-exist` получен `null`: обращение к его `textContent` защищено проверкой `if (el)`. Ссылки меню `.menu-link` перебраны через `querySelectorAll` и `forEach`, выведены их количество (3) и текст каждой.

На листинг 1 показано: выбор по id, классу и селектору, перебор `NodeList`, вложенный поиск, проверка на `null`, перебор ссылок меню.

**Листинг 1 - Задание №1 (task1.js)**

```js
const nav = document.querySelector(".nav");

nav.addEventListener("click", (event) => {
  const link = event.target.closest("a");
  if (!link) return;

  event.preventDefault();

  nav.querySelectorAll("a").forEach((item) => item.classList.remove("active"));
  link.classList.add("active");
});

const featuresSection = document.querySelector(".features");

featuresSection.addEventListener("click", (event) => {
  if (event.target.closest(".add-fav-btn")) return;

  const card = event.target.closest(".feature-card");
  if (!card) return;

  card.classList.toggle("selected");
});

const favoritesList = document.querySelector("#favorites-list");
const favoritesEmpty = document.querySelector("#favorites-empty");

function updateFavoritesEmpty() {
  favoritesEmpty.hidden = favoritesList.children.length > 0;
}

featuresSection.addEventListener("click", (event) => {
  const favBtn = event.target.closest(".add-fav-btn");
  if (!favBtn) return;

  const card = favBtn.closest(".feature-card");
  const title = card.querySelector("h3").textContent;

  const listItem = document.createElement("li");
  listItem.textContent = title;

  const removeBtn = document.createElement("button");
  removeBtn.textContent = "Удалить";
  removeBtn.classList.add("remove-fav-btn");
  removeBtn.addEventListener("click", () => {
    listItem.remove();
    updateFavoritesEmpty();
  });

  listItem.appendChild(removeBtn);
  favoritesList.appendChild(listItem);
  updateFavoritesEmpty();
});

const promoTimer = document.querySelector("#promo-timer");
const promoBadge = document.querySelector(".promo .badge");

let promoSecondsLeft = 300;

function formatTime(totalSeconds) {
  const minutes = String(Math.floor(totalSeconds / 60)).padStart(2, "0");
  const seconds = String(totalSeconds % 60).padStart(2, "0");
  return `${minutes}:${seconds}`;
}

const promoIntervalId = setInterval(() => {
  promoSecondsLeft -= 1;
  promoTimer.textContent = formatTime(promoSecondsLeft);

  if (promoSecondsLeft <= 0) {
    clearInterval(promoIntervalId);
    promoBadge.textContent = "Акция завершена";
  }
}, 1000);

const requestForm = document.querySelector("#request-form");
const formStatus = document.querySelector("#form-status");
const STORAGE_KEY = "gamezone-request";

function setFormStatus(message, isSuccess) {
  formStatus.textContent = message;
  formStatus.className = isSuccess ? "status-success" : "status-error";
}

requestForm.addEventListener("submit", (event) => {
  event.preventDefault();

  const formData = new FormData(requestForm);
  const name = formData.get("name").trim();
  const email = formData.get("email").trim();
  const password = formData.get("password").trim();
  const phone = formData.get("phone").trim();
  const gender = formData.get("gender");
  const level = formData.get("level");

  const errors = [];
  if (!name) errors.push("Введите имя");
  if (!email) errors.push("Введите email");
  else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.push("Email имеет неверный формат");
  }
  if (!password) errors.push("Введите пароль");
  if (!phone) errors.push("Введите телефон");
  else if (!/^\+?[\d\s\-()]{10,}$/.test(phone)) {
    errors.push("Телефон имеет неверный формат");
  }
  if (!gender) errors.push("Выберите пол");
  if (!level) errors.push("Выберите уровень игры");

  if (errors.length > 0) {
    setFormStatus(errors.join(". ") + ".", false);
    return;
  }

  const data = { name, email, password, phone, gender, level };
  localStorage.setItem(STORAGE_KEY, JSON.stringify(data));

  setFormStatus("Заявка успешно отправлена!", true);
  requestForm.reset();

  clearInterval(promoIntervalId);
  promoBadge.textContent = "Купон получен";

  if (!document.querySelector("#promo-coupon")) {
    const coupon = document.createElement("p");
    coupon.id = "promo-coupon";
    coupon.textContent =
      "Ваш купон на скидку 20%: GAMEZONE20 — назовите его при оформлении!";
    document.querySelector(".promo-card").appendChild(coupon);
  }
});

const savedData = localStorage.getItem(STORAGE_KEY);
if (savedData) {
  const data = JSON.parse(savedData);
  requestForm.elements.name.value = data.name ?? "";
  requestForm.elements.email.value = data.email ?? "";
  requestForm.elements.password.value = data.password ?? "";
  requestForm.elements.phone.value = data.phone ?? "";
  if (data.gender) {
    requestForm.querySelector(`input[name="gender"][value="${data.gender}"]`).checked = true;
  }
  if (data.level) {
    requestForm.querySelector(`input[name="level"][value="${data.level}"]`).checked = true;
  }
}

const reviewContent = document.querySelector("#review-content");
const loadReviewBtn = document.querySelector("#load-review-btn");

let reviews = [];
let reviewIndex = 0;

async function fetchReviews() {
  const response = await fetch(
    "https://jsonplaceholder.typicode.com/comments?_limit=10"
  );
  if (!response.ok) {
    throw new Error("Не удалось загрузить отзывы: " + response.status);
  }
  reviews = await response.json();
}

function renderReview(comment) {
  reviewContent.textContent = "";

  const bodyEl = document.createElement("p");
  bodyEl.textContent = comment.body;

  const authorEl = document.createElement("p");
  authorEl.textContent = "Автор: " + comment.email;
  authorEl.classList.add("review-author");

  reviewContent.append(bodyEl, authorEl);
}

loadReviewBtn.addEventListener("click", async () => {
  try {
    if (reviews.length === 0) {
      reviewContent.textContent = "Загрузка...";
      await fetchReviews();
      loadReviewBtn.textContent = "Следующий отзыв";
    }

    renderReview(reviews[reviewIndex]);
    reviewIndex = (reviewIndex + 1) % reviews.length;
  } catch (error) {
    reviewContent.textContent = "Ошибка: " + error.message;
  }
});

const liveClock = document.querySelector("#live-clock");

function updateClock() {
  const now = new Date();
  const time = now.toLocaleTimeString("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
  liveClock.textContent = time;
}

updateClock();
setInterval(updateClock, 1000);
```

Рисунок 1 - Результат выполнения программы task1.js


document.querySelectorAll() возвращает NodeList,
а document.getElementsByClassName() возвращает HTMLCollection.
зрения "живости" — какая коллекция автоматически обновляется, если
после её получения добавить/удалить элемент на странице, а какая
остаётся "снимком" на момент вызова?

`querySelectorAll()` возвращает `NodeList` – статический «снимок» на момент вызова, поэтому элемент, добавленный позже, в уже полученный список не попадёт. `getElementsByClassName()` возвращает `HTMLCollection`, которая «живая»: она сама обновляется при добавлении и удалении элементов на странице. Проверка на `null` важна потому, что `querySelector` возвращает `null` при отсутствии элемента, а `NodeList` удобно обходить методом `forEach`.

## Задание 2. Изменение содержимого и атрибутов

Изучены свойства содержимого и работа с атрибутами. Через `textContent` заменены заголовок `#username` и описание `#bio`, через `innerHTML` в `#notice` вставлена разметка `<strong>`, которая реально отрисовалась жирным. У изображения `#avatar` прочитан `alt` через `getAttribute` (получено «Изображение»), затем `setAttribute` изменены `src` и `alt`.

Для бейджика `#status-badge` через `classList.remove("badge-offline")` убран класс офлайн-статуса и через `classList.add("badge-online")` добавлен класс онлайн, после чего `classList.contains("badge-online")` вернул `true`.

На листинг 2 показано: `textContent`, `innerHTML`, `getAttribute`/`setAttribute`, `classList.add`/`remove`/`contains`.

**Листинг 2 - Задание №2 (task2.js)**

```js
// TODO 1: textContent

document.getElementById("username").textContent = "Спортивный клуб «Атлет»";

// TODO 2: textContent

document.getElementById("bio").textContent = "Спортивный клуб «Атлет» — учреждение для подготовки спортсменов";

// TODO 3: innerHTML

document.getElementById("notice").innerHTML = "<strong>Внимание!</strong> Информация обновлена.";

// TODO 4: setAttribute / getAttribute

const avatar = document.getElementById("avatar");
console.log(avatar.getAttribute("alt"));
avatar.setAttribute("src", "photo2.jpg");
avatar.setAttribute("alt", "Новое описание");

// TODO 5: classList.add/remove

const statusBadge = document.getElementById("status-badge");
statusBadge.classList.remove("badge-offline");
statusBadge.classList.add("badge-online");

// TODO 6: classList.contains

console.log(document.getElementById("status-badge").classList.contains("badge-online"));

// TODO 7: textContent vs innerHTML
```

Рисунок 2 - Результат выполнения программы task2.js


и innerHTML, и почему для вывода текста от пользователя (например,
комментария) обычно безопаснее использовать textContent, а не innerHTML?

`textContent` вставляет строку как обычный текст: теги не разбираются, поэтому `<script>` останется видимым текстом. `innerHTML` разбирает строку как HTML, и вставленный тег реально создаёт DOM-узел, а `<script>` или обработчик вроде `<img onerror=...>` может выполниться – это XSS-уязвимость. Для пользовательского ввода безопаснее `textContent`, потому что он не создаёт элементов и не выполняет код.

## Задание 3. Стили через JS

Изучено управление стилями через `element.style`. Элементу `#js-box` заданы фон `orange` через `style.backgroundColor`, ширина `150px` и высота `80px` через `style.width` и `style.height`, скругление `10px` и рамка `2px solid black`. Имена CSS-свойств записаны в camelCase, а размеры – строками с единицами `px`.

Для `#preset-box`, вид которого задан классом `.preset`, `element.style.backgroundColor` вернул пустую строку, так как инлайн-стилей у элемента нет. Реальный цвет и ширина получены через `getComputedStyle()`: фон `rgb(0, 128, 128)` (teal) и ширина `120px`.

На листинг 3 показано: инлайн-стили, единицы измерения, несколько свойств сразу, `element.style` и `getComputedStyle`.

**Листинг 3 - Задание №3 (task3.js)**

```js
// TODO 1: element.style

const jsBox = document.getElementById("js-box");
jsBox.style.backgroundColor = "orange";

// TODO 2: element.style + единицы измерения

jsBox.style.width = "150px";
jsBox.style.height = "80px";

// TODO 3: несколько свойств сразу

jsBox.style.borderRadius = "10px";
jsBox.style.border = "2px solid black"; 

// TODO 4: element.style на элементе со стилями из CSS-класса

const presetBox = document.querySelector("#preset-box");
console.log(presetBox.style.backgroundColor);

// TODO 5: getComputedStyle

console.log(getComputedStyle(presetBox).backgroundColor);

// TODO 6: getComputedStyle — ширина

console.log(getComputedStyle(presetBox).width);

// TODO 7: element.style vs getComputedStyle
```

Рисунок 3 - Результат выполнения программы task3.js


element.style.backgroundColor вывел пустую строку, а в TODO 5
getComputedStyle вывел реальный цвет? В чём принципиальная разница между
этими двумя способами?

`element.style` – это объект только инлайн-стилей: он содержит лишь то, что задано атрибутом `style` или через JS, поэтому для `#preset-box` он пуст. `getComputedStyle(el)` возвращает итоговые значения после применения всех источников – CSS-классов, наследования и стилей браузера, поэтому показывает реальный `teal` и ширину `120px`. Читать вычисленные значения нужно через `getComputedStyle`, а записывать – через `element.style`.

## Задание 4. События (addEventListener)

Изучены события и `addEventListener`. На `#like-btn` повешены два обработчика `click`: первый меняет текст на «Нравится», второй увеличивает число в `#counter` на единицу, причём оба срабатывают независимо. Счётчик последовательно принимает значения 1, 2, 3 и так далее.

Отправка формы `#demo-form` перехвачена `event.preventDefault()`, значение `#name-input` выведено в `#form-result`; событие `input` на этом же поле обновляет `#live-preview` на каждый введённый символ. На `#color-list` повешен один обработчик, который через `event.target` определяет кликнутый `li` и записывает его `data-color` в `#clicked-color` (`red`, `green` или `blue`). На `document` подписан `keydown`, и в консоль выводится `event.key`.

На листинг 4 показано: `click`, два обработчика на одном элементе, `submit` с `preventDefault`, `input`, делегирование через `event.target`, `keydown` на документе.

**Листинг 4 - Задание №4 (task4.js)**

```js
// TODO 1: клик меняет свой же текст

let likeBtn = document.querySelector("#like-btn")
likeBtn.addEventListener("click", function() {
    likeBtn.textContent = "❤️ Нравится"
})

// TODO 2: второй обработчик на ТОТ ЖЕ элемент

likeBtn.addEventListener("click", function() {
    const counter = document.getElementById("counter")
    counter.textContent = Number(counter.textContent) + 1
})

// TODO 3: submit формы + preventDefault

document.querySelector("#demo-form").addEventListener("submit", function(event){
    event.preventDefault()
    const nameInput = document.querySelector("#name-input")
    document.querySelector("#form-result").textContent = nameInput.value
})

// TODO 4: событие "input" — живой предпросмотр

document.querySelector("#name-input").addEventListener("input", function(){
    document.querySelector("#live-preview").textContent = this.value
})

// TODO 5: один обработчик на список — event.target

document.querySelector("#color-list").addEventListener("click", function(event){
    document.querySelector("#clicked-color").textContent = event.target.getAttribute("data-color")
})

// TODO 6: keydown на всём документе

document.addEventListener("keydown", function(event){
    console.log(event.key)
})

// TODO 7: ссылка на функцию vs вызов функции
```

Рисунок 4 - Результат выполнения программы task4.js


btn.addEventListener("click", doSomething()) — ошибка (сработает сразу,
один раз, при загрузке страницы), а
btn.addEventListener("click", doSomething) — правильно (сработает при клике)?
В чём разница между этими двумя записями?

`addEventListener("click", doSomething())` – это вызов функции: `doSomething` выполняется сразу при загрузке скрипта, а в обработчик передаётся её результат `undefined`, поэтому клик её уже не вызовет. Запись `addEventListener("click", doSomething)` передаёт ссылку на функцию, и браузер вызывает её при каждом клике. То есть в обработчик передаётся функция без скобок, а со скобками – её результат.

## Задание 5. Всплытие и делегирование событий

Изучено всплытие событий. На `#outer` и `#inner` повешены обработчики `click`: при клике по внутреннему блоку сначала выводится «клик по inner», затем «клик по outer», потому что событие поднимается от целевого элемента к родителям. В обработчике `#outer` выведены `event.target.id` (реально кликнутый элемент, он меняется) и `event.currentTarget.id` (элемент с обработчиком, всегда `outer`).

На `#todo-list` повешен один обработчик: если цель имеет класс `delete-btn`, ближайший `li` удаляется через `closest("li")` и `remove()`, иначе у `li` переключается класс `done`. Клик мимо задач защищён проверкой `closest("li") === null` с `return`. Для `#inner` добавлен второй обработчик с `event.stopPropagation()`, который останавливает всплытие, поэтому обработчик `#outer` при клике по внутреннему блоку больше не срабатывает.

На листинг 5 показано: всплытие, `event.target` и `event.currentTarget`, делегирование удаления и отметки задач, `stopPropagation`, защита от клика в пустоту.

**Листинг 5 - Задание №5 (task5.js)**

```js
// TODO 1: всплытие (bubbling)

document.querySelector("#outer").addEventListener("click", function(){
    console.log("клик по outer")
})

document.querySelector("#inner").addEventListener("click", function(){
    console.log("клик по inner")
})

// TODO 2: event.target vs event.currentTarget

document.querySelector("#outer").addEventListener("click", function(event){
    console.log(event.target.id)
    console.log(event.currentTarget.id)
})

// TODO 3: делегирование — удаление задачи

document.querySelector("#todo-list").addEventListener("click", function(event){
    const li = event.target.closest("li")
    if (li === null){          // TODO 6: клик в пустоту — выходим
        return
    }
    if (event.target.classList.contains("delete-btn")){
        li.remove()            // TODO 3: удаляем задачу
    } else {
        li.classList.toggle("done")   // TODO 4: отмечаем как выполненную
    }
})

// TODO 4: делегирование — отметить как выполненное

// TODO 5: stopPropagation

document.querySelector("#inner").addEventListener("click", function(event){
    event.stopPropagation()
})

// TODO 6: защита от клика "в пустоту"

document.querySelector("#todo-list").addEventListener("click", function(event){
    if (event.target.closest("li") === null){
        return
    }
})

// TODO 7: зачем нужно делегирование
```

Рисунок 5 - Результат выполнения программы task5.js


и event.currentTarget, и почему вешать ОДИН обработчик на родителя
(делегирование) часто лучше, чем вешать отдельный обработчик на каждый
дочерний элемент — особенно если элементы могут добавляться на страницу
позже?

`event.target` – элемент, где клик произошёл реально, он меняется от клика к клику; `event.currentTarget` – элемент, на котором висит обработчик, он постоянен. Делегирование (один обработчик на родителе) удобнее отдельного обработчика на каждом ребёнке: оно работает и для элементов, добавленных позже, ведь событие всплывает до родителя, и не требует заново назначать обработчики.

## Задание 6. Создание и удаление элементов

Изучено создание и удаление элементов. Через `document.createElement("p")` создан абзац с текстом «Привет!» и выведен в консоль: на странице он не появился, пока его не вставили. После `sandbox.appendChild()` абзац реально отобразился в `#sandbox`.

По кнопке `#add-task-btn` создаётся `li` с текстом из `#new-task-input`, добавляется в конец `#todo-list` через `appendChild`, после чего поле очищается (`value = ""`), а пустое поле запись не добавляет. Кнопка `#add-task-top-btn` вставляет такой же элемент в начало через `prepend`. Вместе с `li` создаётся кнопка `✕` с классом `delete-btn`; она вставляется внутрь `li`, и ей сразу назначается собственный обработчик, удаляющий элемент через `li.remove()`.

На листинг 6 показано: `createElement` без вставки, `appendChild`, полный рецепт по кнопке, очистка поля, `prepend`, прямое удаление созданного элемента.

**Листинг 6 - Задание №6 (task6.js)**

```js
// TODO 1: createElement без вставки

const sandbox = document.querySelector("#sandbox");
const newParagraph = document.createElement("p");
newParagraph.textContent = "Привет!";
console.log(newParagraph);

// TODO 2: appendChild

sandbox.appendChild(newParagraph);

// TODO 3: полный рецепт по кнопке

const addTaskBtn = document.querySelector("#add-task-btn");
const newTaskInput = document.querySelector("#new-task-input");
const todoList = document.querySelector("#todo-list");

addTaskBtn.addEventListener("click", function () {
    if (newTaskInput.value !== "") {
        const li = createTaskItem(newTaskInput.value);   // создать + настроить
        todoList.appendChild(li);                        // вставить
        newTaskInput.value = "";                         // TODO 4: очищаем поле
    }
});

// TODO 4: очистка поля после добавления

// TODO 5: prepend — добавление в начало

const addTaskTopBtn = document.querySelector("#add-task-top-btn");

addTaskTopBtn.addEventListener("click", function () {
    if (newTaskInput.value !== "") {
        const li = createTaskItem(newTaskInput.value);
        todoList.prepend(li);          // новая запись встаёт ПЕРВОЙ
        newTaskInput.value = "";
    }
});

// TODO 6: удаление созданного элемента напрямую (без делегирования)

function createTaskItem(text) {
    const li = document.createElement("li");
    li.textContent = text;

    const deleteBtn = document.createElement("button");
    deleteBtn.textContent = "✕";
    deleteBtn.classList.add("delete-btn");
    deleteBtn.addEventListener("click", function () {
        li.remove();                    // удаляем сам <li> с кнопкой
    });

    li.appendChild(deleteBtn);         // кнопка живёт ВНУТРИ <li>
    return li;
}

// TODO 7: прямой обработчик vs делегирование
```

Рисунок 6 - Результат выполнения программы task6.js


элемента (как в TODO 6) логично повесить обработчик прямо на него, а
делегирование (тема 5, один обработчик на родителя) больше подходит для
элементов, которые уже есть на странице в большом количестве или
добавляются откуда-то ещё?

Для свежесозданного элемента ссылка уже хранится в переменной, поэтому обработчик удобно вешать прямо на него – как у кнопки удаления. Делегирование больше подходит для большого числа существующих элементов или для элементов, которые добавляются из другого места: один обработчик на родителе покрывает их все. Выбор зависит от того, откуда берутся элементы и сколько их.

## Задание 7. Работа с формами

Изучена работа с формами. На форму `#reg-form` подписан обработчик `submit` с `event.preventDefault()`, поэтому страница не перезагружается. В одном обработчике выводятся значения полей: текст из `#username-field` через `.value`, состояние `#agree-checkbox` через `.checked`, выбранная радиокнопка через `document.querySelector('input[name="account-type"]:checked')` и значение `#country-select`.

В начале обработчика выполнена валидация: при пустом имени в `#error-message` записывается «Введите имя» и выполняется `return`; если не отмечен чекбокс – «Нужно согласиться с условиями» и `return`. Если обе проверки пройдены, сообщение очищается, и только тогда значения полей выводятся в консоль.

На листинг 7 показано: обработчик `submit`, `.value`, `.checked`, радиокнопка `:checked`, `select`, две проверки валидации.

**Листинг 7 - Задание №7 (task7.js)**

```js
const form = document.querySelector("#reg-form");

// TODO 1: текстовое поле

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

// TODO 2: чекбокс — .checked, а не .value!

// TODO 3: радиокнопки — :checked

// TODO 4: select

// TODO 5: валидация — обязательное поле

// TODO 6: валидация — согласие с условиями

// TODO 7: почему не .value у чекбокса
```

Рисунок 7 - Результат выполнения программы task7.js


.checked, а не .value, хотя у текстового поля именно .value? Что вообще
хранится в .value у чекбокса, и почему оно бесполезно для проверки
"отмечен/не отмечен"?

У чекбокса `.value` – это значение атрибута `value` (по умолчанию `"on"`), оно задаётся в разметке и не меняется при клике, поэтому о состоянии ничего не говорит. `.checked` возвращает текущее состояние `true`/`false`, и именно его нужно проверять. У текстового поля `.value` хранит введённый текст, поэтому там он и полезен.

## Задание 8. Таймеры

Изучены `setTimeout` и `setInterval`. По кнопке `#toast-btn` у `#toast` убирается класс `hidden`, а через `setTimeout` с задержкой 3000 мс класс возвращается – уведомление исчезает. Для `#clock` запущен `setInterval` с интервалом 1000 мс, который записывает `new Date().toLocaleTimeString()` на каждом тике.

Обратный отсчёт запускается по `#start-btn`: каждую секунду число в `#countdown` уменьшается на единицу (10 => 9 => ... => 0), а при достижении нуля интервал останавливается через `clearInterval`. Идентификатор интервала объявлен вне обработчиков, поэтому доступен кнопкам `#stop-btn` (остановка) и `#reset-btn` (остановка и возврат значения `10`). Перед новым запуском старый интервал также гасится, чтобы не работали два таймера одновременно.

На листинг 8 показано: `setTimeout`, `setInterval` для часов, обратный отсчёт, остановка на нуле, кнопки «Стоп» и «Сброс».

**Листинг 8 - Задание №8 (task8.js)**

```js
// TODO 1: setTimeout — уведомление само исчезает

const toastBtn = document.querySelector("#toast-btn");
const toast = document.querySelector("#toast");

toastBtn.addEventListener("click", function () {
    toast.classList.remove("hidden");            // показать
    setTimeout(function () {
        toast.classList.add("hidden");           // скрыть через 3 секунды
    }, 3000);
});

// TODO 2: setInterval — живые часы

const clock = document.querySelector("#clock");

setInterval(function () {
    clock.textContent = new Date().toLocaleTimeString();
}, 1000);

// TODO 3: setInterval — обратный отсчёт

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

// TODO 5: кнопка "Стоп"

const stopBtn = document.querySelector("#stop-btn");

stopBtn.addEventListener("click", function () {
    if (countdownIntervalId !== null) {
        clearInterval(countdownIntervalId);      // останавливаем отсчёт
        countdownIntervalId = null;
    }
});

// TODO 6: кнопка "Сброс"

const resetBtn = document.querySelector("#reset-btn");

resetBtn.addEventListener("click", function () {
    if (countdownIntervalId !== null) {
        clearInterval(countdownIntervalId);      // останавливаем интервал
        countdownIntervalId = null;
    }
    countdown.textContent = "10";                // возвращаем исходное значение
});

// TODO 7: зачем сохранять id таймера
```

Рисунок 8 - Результат выполнения программы task8.js


что вернул setInterval/setTimeout, в переменную? Что произойдёт, если
несколько раз запустить setInterval для одного и того же счётчика, не
останавливая предыдущие через clearInterval?

`setInterval` и `setTimeout` возвращают числовой идентификатор, и только он позволяет потом вызвать `clearInterval(id)` или `clearTimeout(id)`. Если идентификатор не сохранить, остановить таймер нечем. При повторном запуске без остановки предыдущего работают несколько интервалов сразу, поэтому счётчик уменьшается в несколько раз быстрее.

## Задание 9. Fetch API

Изучена работа с `Fetch API`. По кнопке `#load-users-btn` выполняется `fetch` к `https://jsonplaceholder.typicode.com/users`, ответ разбирается через `response.json()` во втором `.then()`, данные выводятся в консоль, а их количество записывается в `#users-result` в виде «Пользователей: 10». Ошибки сети обрабатываются `.catch()`.

Функция `loadSingleUser()` объявлена как `async` и получает одного пользователя через `await fetch(...)` и `await response.json()`, записывая `data.name` в `#single-result`; она обёрнута в `try/catch` и привязана к `#load-single-btn`. Функция `loadBadUser()` запрашивает несуществующий id `9999`, проверяет `response.ok` и при ошибке выбрасывает `new Error("Пользователь не найден: " + response.status)`, а в `catch` выводит `error.message` в `#error-result`; она привязана к `#load-bad-btn`.

На листинг 9 показано: цепочка `fetch`/`.then`/`.catch`, вывод количества, `async`/`await` с `try/catch`, обработка ответа `404` через `response.ok`, привязка функций к кнопкам.

**Листинг 9 - Задание №9 (task9.js)**

```js
// TODO 1: fetch + .then + .catch — список пользователей

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

// TODO 2: покажите количество на странице

// TODO 1: usersResult.textContent = "Пользователей: " + data.length;

// TODO 3: async/await — один пользователь

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

const loadSingleBtn = document.querySelector("#load-single-btn");

loadSingleBtn.addEventListener("click", loadSingleUser);

// TODO 5: обработка HTTP-ошибки (404)

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

const loadBadBtn = document.querySelector("#load-bad-btn");

loadBadBtn.addEventListener("click", loadBadUser);

// TODO 7: почему .catch() одного не хватает
```

Рисунок 9 - Результат выполнения программы task9.js


не поймает ситуацию "пользователь не найден" (404)? Что вообще считается
"ошибкой" с точки зрения fetch, а что — просто обычным (хоть и неудачным)
ответом сервера?

`fetch` переходит в `.catch()` только при сетевых ошибках, когда сервер недоступен или соединение оборвалось. Ответ `404` – это обычный ответ сервера: тело можно прочитать, просто статус не принадлежит диапазону 2xx, поэтому `.catch()` его не ловит. Проверять `response.ok` нужно вручную, иначе сообщение «пользователь не найден» попадёт в `.then` как обычные данные.

## Задание 10. JSON (JSON.stringify / JSON.parse)

Изучена работа с JSON. Объект `person` выведен в консоль сначала как объект, затем как строка `JSON.stringify(person)` с ключами в кавычках. Строка `raw` преобразована обратно в объект через `JSON.parse(raw)`, и `parsed.name` подтверждает, что это настоящий объект.

Профиль из полей `#name-field` и `#age-field` собирается в объект, сериализуется через `JSON.stringify` и сохраняется в `localStorage` по ключу `profile`. При загрузке `getItem` проверяется на `null`, затем `JSON.parse` восстанавливает объект, и в `#output` выводится имя и возраст. Для демонстрации ошибки объект `{ foo: "bar" }` сохранён без `JSON.stringify`, поэтому в `localStorage` попала строка `"[object Object]"`; её разбор обёрнут в `try/catch`, и при ошибке в `#bad-output` выводится «Не удалось разобрать JSON: испорченные данные».

На листинг 10 показано: `JSON.stringify`, `JSON.parse`, сохранение и загрузка профиля в `localStorage`, сохранение без `stringify`, разбор повреждённых данных через `try/catch`.

**Листинг 10 - Задание №10 (task10.js)**

```js
// TODO 1: JSON.stringify — объект в строку

const person = { name: "Аня", age: 20 };
console.log(person);
console.log(JSON.stringify(person));

// TODO 2: JSON.parse — строка в объект

const raw = '{"name":"Аня","age":20}';
const parsed = JSON.parse(raw);
console.log(parsed);
console.log(parsed.name);

// TODO 3: сохранение в localStorage

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

// TODO 5: демонстрация ошибки — сохранение БЕЗ stringify

const badSaveBtn = document.querySelector("#bad-save-btn");

badSaveBtn.addEventListener("click", function () {
    localStorage.setItem("bad", { foo: "bar" });
    console.log("Сохранено:", localStorage.getItem("bad"));
});

// TODO 6: попытка прочитать это же значение

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

// TODO 7: почему localStorage нужен именно JSON
```

Рисунок 10 - Результат выполнения программы task10.js


требует JSON.stringify/JSON.parse — что будет, если положить туда объект
напрямую, и почему localStorage в принципе не может хранить "живые"
JS-объекты?

`localStorage` хранит только строки, поэтому объект, сохранённый напрямую, превращается в `"[object Object]"`, и восстановить данные из этой строки нельзя. `JSON.stringify` переводит объект в строку, а `JSON.parse` разбирает её обратно – это сериализация и десериализация. «Живые» JS-объекты с методами и ссылками в `localStorage` сохранить невозможно: там может лежать только текст.

## Итоговое задание

Тема: одностраничный сайт в стиле «Игра» – GameZone, дополненный интерактивностью на JavaScript.

Реализована навигация через делегирование: один обработчик на `.nav` находит ближайшую ссылку, отменяет переход по `preventDefault()`, снимает класс `active` со всех ссылок и добавляет его выбранной. Карточки услуг также обрабатываются делегированием: клик по карточке переключает класс `selected`, а клик по кнопке «В избранное» не выделяет карточку, а создаёт в `#favorites-list` новый `li` с названием услуги и кнопкой «Удалить». У кнопки удаления свой прямой обработчик, который удаляет `li` и обновляет сообщение `#favorites-empty`.

Промо-таймер запускается через `setInterval` и каждую секунду уменьшает число секунд, выводя время в формате `мм:сс` (05:00 => 04:59 => ...). При достижении нуля интервал останавливается через `clearInterval`, а бейджик получает текст «Акция завершена». После успешной отправки формы таймер останавливается, бейджик меняется на «Купон получен», и в промо-карточку добавляется абзац с кодом скидки.

Форма `#request-form` обрабатывается через `FormData` и `event.preventDefault()`. Проверяются имя, email, пароль, телефон, пол и уровень игры: пустые поля и неверный формат email/телефона дают сообщения об ошибке в `#form-status`, а при успехе данные сериализуются через `JSON.stringify` и сохраняются в `localStorage` по ключу `gamezone-request`. При загрузке страницы сохранённые данные восстанавливаются в поля формы, включая радиокнопки. Отзыв загружается с сервера через `fetch` с проверкой `response.ok`, а живые часы обновляются через `setInterval` каждую секунду.

На листинг 11 показано: делегирование навигации и карточек, избранное с созданием и удалением элементов, промо-таймер, валидация формы, сохранение и восстановление данных, загрузка отзыва через `fetch`, живые часы.

**Листинг 11 - Итоговое задание (task_itog.js)**

```js
const nav = document.querySelector(".nav");

nav.addEventListener("click", (event) => {
  const link = event.target.closest("a");
  if (!link) return;

  event.preventDefault();

  nav.querySelectorAll("a").forEach((item) => item.classList.remove("active"));
  link.classList.add("active");
});

const featuresSection = document.querySelector(".features");

featuresSection.addEventListener("click", (event) => {
  if (event.target.closest(".add-fav-btn")) return;

  const card = event.target.closest(".feature-card");
  if (!card) return;

  card.classList.toggle("selected");
});

const favoritesList = document.querySelector("#favorites-list");
const favoritesEmpty = document.querySelector("#favorites-empty");

function updateFavoritesEmpty() {
  favoritesEmpty.hidden = favoritesList.children.length > 0;
}

featuresSection.addEventListener("click", (event) => {
  const favBtn = event.target.closest(".add-fav-btn");
  if (!favBtn) return;

  const card = favBtn.closest(".feature-card");
  const title = card.querySelector("h3").textContent;

  const listItem = document.createElement("li");
  listItem.textContent = title;

  const removeBtn = document.createElement("button");
  removeBtn.textContent = "Удалить";
  removeBtn.classList.add("remove-fav-btn");
  removeBtn.addEventListener("click", () => {
    listItem.remove();
    updateFavoritesEmpty();
  });

  listItem.appendChild(removeBtn);
  favoritesList.appendChild(listItem);
  updateFavoritesEmpty();
});

const promoTimer = document.querySelector("#promo-timer");
const promoBadge = document.querySelector(".promo .badge");

let promoSecondsLeft = 300;

function formatTime(totalSeconds) {
  const minutes = String(Math.floor(totalSeconds / 60)).padStart(2, "0");
  const seconds = String(totalSeconds % 60).padStart(2, "0");
  return `${minutes}:${seconds}`;
}

const promoIntervalId = setInterval(() => {
  promoSecondsLeft -= 1;
  promoTimer.textContent = formatTime(promoSecondsLeft);

  if (promoSecondsLeft <= 0) {
    clearInterval(promoIntervalId);
    promoBadge.textContent = "Акция завершена";
  }
}, 1000);

const requestForm = document.querySelector("#request-form");
const formStatus = document.querySelector("#form-status");
const STORAGE_KEY = "gamezone-request";

function setFormStatus(message, isSuccess) {
  formStatus.textContent = message;
  formStatus.className = isSuccess ? "status-success" : "status-error";
}

requestForm.addEventListener("submit", (event) => {
  event.preventDefault();

  const formData = new FormData(requestForm);
  const name = formData.get("name").trim();
  const email = formData.get("email").trim();
  const password = formData.get("password").trim();
  const phone = formData.get("phone").trim();
  const gender = formData.get("gender");
  const level = formData.get("level");

  const errors = [];
  if (!name) errors.push("Введите имя");
  if (!email) errors.push("Введите email");
  else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.push("Email имеет неверный формат");
  }
  if (!password) errors.push("Введите пароль");
  if (!phone) errors.push("Введите телефон");
  else if (!/^\+?[\d\s\-()]{10,}$/.test(phone)) {
    errors.push("Телефон имеет неверный формат");
  }
  if (!gender) errors.push("Выберите пол");
  if (!level) errors.push("Выберите уровень игры");

  if (errors.length > 0) {
    setFormStatus(errors.join(". ") + ".", false);
    return;
  }

  const data = { name, email, password, phone, gender, level };
  localStorage.setItem(STORAGE_KEY, JSON.stringify(data));

  setFormStatus("Заявка успешно отправлена!", true);
  requestForm.reset();

  clearInterval(promoIntervalId);
  promoBadge.textContent = "Купон получен";

  if (!document.querySelector("#promo-coupon")) {
    const coupon = document.createElement("p");
    coupon.id = "promo-coupon";
    coupon.textContent =
      "Ваш купон на скидку 20%: GAMEZONE20 — назовите его при оформлении!";
    document.querySelector(".promo-card").appendChild(coupon);
  }
});

const savedData = localStorage.getItem(STORAGE_KEY);
if (savedData) {
  const data = JSON.parse(savedData);
  requestForm.elements.name.value = data.name ?? "";
  requestForm.elements.email.value = data.email ?? "";
  requestForm.elements.password.value = data.password ?? "";
  requestForm.elements.phone.value = data.phone ?? "";
  if (data.gender) {
    requestForm.querySelector(`input[name="gender"][value="${data.gender}"]`).checked = true;
  }
  if (data.level) {
    requestForm.querySelector(`input[name="level"][value="${data.level}"]`).checked = true;
  }
}

const reviewContent = document.querySelector("#review-content");
const loadReviewBtn = document.querySelector("#load-review-btn");

let reviews = [];
let reviewIndex = 0;

async function fetchReviews() {
  const response = await fetch(
    "https://jsonplaceholder.typicode.com/comments?_limit=10"
  );
  if (!response.ok) {
    throw new Error("Не удалось загрузить отзывы: " + response.status);
  }
  reviews = await response.json();
}

function renderReview(comment) {
  reviewContent.textContent = "";

  const bodyEl = document.createElement("p");
  bodyEl.textContent = comment.body;

  const authorEl = document.createElement("p");
  authorEl.textContent = "Автор: " + comment.email;
  authorEl.classList.add("review-author");

  reviewContent.append(bodyEl, authorEl);
}

loadReviewBtn.addEventListener("click", async () => {
  try {
    if (reviews.length === 0) {
      reviewContent.textContent = "Загрузка...";
      await fetchReviews();
      loadReviewBtn.textContent = "Следующий отзыв";
    }

    renderReview(reviews[reviewIndex]);
    reviewIndex = (reviewIndex + 1) % reviews.length;
  } catch (error) {
    reviewContent.textContent = "Ошибка: " + error.message;
  }
});

const liveClock = document.querySelector("#live-clock");

function updateClock() {
  const now = new Date();
  const time = now.toLocaleTimeString("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
  liveClock.textContent = time;
}

updateClock();
setInterval(updateClock, 1000);
```

Рисунок 11 - Результат выполнения программы task_itog.js


В итоговом задании делегирование позволило повесить один обработчик на `.nav` и обрабатывать клики по всем ссылкам, включая те, что теоретически добавляются позже. События всплывают от целевого элемента к родителю, поэтому через `event.target.closest()` можно определить, по какому элементу кликнули, не назначая обработчики на каждую ссылку и каждую карточку отдельно.

Итоговое задание объединило изученные приёмы: выбор элементов и работу с классами, делегирование событий, создание и удаление узлов, таймеры, валидацию формы, `JSON` с `localStorage` и `Fetch API`. Делегирование позволило обслуживать все карточки и ссылки одним обработчиком, а сохранение формы в `localStorage` – восстановить заявку после перезагрузки страницы. Проверка `response.ok` в загрузке отзыва показывает, что `fetch` сам по себе не считает ответ с ошибкой исключением.

## Заключение

В ходе лабораторной работы изучены и применены на практике возможности JavaScript в браузере. Освоены выбор элементов через `getElementById`, `querySelector` и `querySelectorAll`, различие статического `NodeList` и «живой» `HTMLCollection`, перебор списков и защита от `null`. Изучены изменение содержимого через `textContent` и `innerHTML`, работа с атрибутами через `getAttribute`/`setAttribute`, управление классами через `classList`, а также различие инлайн-стилей `element.style` и вычисленных стилей `getComputedStyle`.

Рассмотрены события `click`, `submit`, `input` и `keydown`, несколько обработчиков на одном элементе, всплытие событий, `event.target` и `event.currentTarget`, делегирование и `stopPropagation`. Освоены создание и удаление элементов через `createElement`, `appendChild`, `prepend` и `remove`, работа с формами (`value`, `checked`, `:checked`, `select`) и валидация. Изучены таймеры `setTimeout` и `setInterval` с остановкой через `clearInterval`, запросы к серверу через `Fetch API` с обработкой HTTP-ошибок по `response.ok`, а также сериализация данных через `JSON.stringify` и `JSON.parse` для `localStorage`.

Все полученные знания применены в итоговом задании при доработке одностраничного сайта GameZone: добавлены делегирование навигации и карточек, избранное, промо-таймер, валидация формы с сохранением данных, загрузка отзыва с сервера и живые часы. Практика показала, что понимание модели событий, работы с DOM и асинхронных запросов необходимо для создания интерактивных веб-страниц.
