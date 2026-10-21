// ЛАБОРАТОРНАЯ РАБОТА №3
// ИТОГОВОЕ ЗАДАНИЕ — Вариант 04
// Стиль: «Игра» — GameZone
//
// 1. Навигация (делегирование)
// 2. Карточки услуг (делегирование + классы)
// 3. Избранное (создание и удаление элементов)
// 4. Промо-таймер (setInterval)
// 5. Валидация формы (события + формы)
// 6. Сохранение данных формы (JSON + localStorage)
// 7. Отзыв с сервера (Fetch API)
// 8. Улучшения: живые часы + проверка формата email/телефона

// === 1. Навигация (делегирование) ===
const nav = document.querySelector(".nav");

nav.addEventListener("click", (event) => {
  const link = event.target.closest("a");
  if (!link) return;

  event.preventDefault();

  nav.querySelectorAll("a").forEach((item) => item.classList.remove("active"));
  link.classList.add("active");
});

// === 2. Карточки услуг (делегирование + классы) ===
const featuresSection = document.querySelector(".features");

featuresSection.addEventListener("click", (event) => {
  if (event.target.closest(".add-fav-btn")) return;

  const card = event.target.closest(".feature-card");
  if (!card) return;

  card.classList.toggle("selected");
});

// === 3. Избранное (создание и удаление элементов) ===
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

// === 4. Промо-таймер (setInterval) ===
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

// === 5. Валидация формы (события + формы) ===
// === 6. Сохранение данных формы (JSON + localStorage) ===
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

// === 7. Отзыв с сервера (Fetch API) ===
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

// === 8. Улучшения: живые часы ===
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
