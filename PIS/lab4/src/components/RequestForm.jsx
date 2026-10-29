// Тема 3 + 4 + 7 + 8: controlled-форма заявки.
// Все поля хранятся в state (value + onChange), отправка — onSubmit
// с e.preventDefault(). Валидация повторяет ЛР3, данные сохраняются
// в localStorage и восстанавливаются при монтировании через useEffect.

import { useEffect, useState } from "react";

const STORAGE_KEY = "gamezone-request";

const EMPTY_FORM = {
  name: "",
  email: "",
  password: "",
  phone: "",
  gender: "",
  level: "",
};

function RequestForm({ onRequestSuccess }) {
  const [form, setForm] = useState(EMPTY_FORM);
  const [status, setStatus] = useState({ type: "", message: "" });

  // Тема 8: восстановление сохранённой заявки при открытии страницы.
  useEffect(() => {
    const savedData = localStorage.getItem(STORAGE_KEY);
    if (!savedData) return;

    try {
      const data = JSON.parse(savedData);
      setForm((prev) => ({ ...prev, ...data }));
    } catch {
      // Повреждённые данные просто игнорируем.
    }
  }, []);

  function handleChange(event) {
    const { name, value } = event.target;
    setForm((prev) => ({ ...prev, [name]: value }));

    if (status.message) {
      setStatus({ type: "", message: "" });
    }
  }

  function handleSubmit(event) {
    event.preventDefault();

    const data = {
      name: form.name.trim(),
      email: form.email.trim(),
      password: form.password.trim(),
      phone: form.phone.trim(),
      gender: form.gender,
      level: form.level,
    };

    const errors = [];
    if (!data.name) errors.push("Введите имя");
    if (!data.email) errors.push("Введите email");
    else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(data.email)) {
      errors.push("Email имеет неверный формат");
    }
    if (!data.password) errors.push("Введите пароль");
    if (!data.phone) errors.push("Введите телефон");
    else if (!/^\+?[\d\s\-()]{10,}$/.test(data.phone)) {
      errors.push("Телефон имеет неверный формат");
    }
    if (!data.gender) errors.push("Выберите пол");
    if (!data.level) errors.push("Выберите уровень игры");

    if (errors.length > 0) {
      setStatus({ type: "error", message: errors.join(". ") + "." });
      return;
    }

    localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
    setStatus({ type: "success", message: "Заявка успешно отправлена!" });
    setForm(EMPTY_FORM);

    // Тема 9: сообщаем родителю — промо-блок должен выдать купон.
    onRequestSuccess();
  }

  return (
    <section className="form-section" id="contacts">
      <h2>Оставить заявку</h2>

      <form
        id="request-form"
        action="/submit"
        method="POST"
        noValidate
        onSubmit={handleSubmit}
      >
        <input
          type="text"
          name="name"
          placeholder="Ваше имя"
          required
          value={form.name}
          onChange={handleChange}
        />
        <input
          type="email"
          name="email"
          placeholder="you@example.com"
          required
          value={form.email}
          onChange={handleChange}
        />
        <input
          type="password"
          name="password"
          placeholder="Ваш пароль"
          required
          autoComplete="new-password"
          value={form.password}
          onChange={handleChange}
        />
        <input
          type="tel"
          name="phone"
          placeholder="+7 (___) ___-__-__"
          required
          value={form.phone}
          onChange={handleChange}
        />

        <fieldset className="radio-group">
          <legend>Пол</legend>
          <label>
            <input
              type="radio"
              name="gender"
              value="male"
              checked={form.gender === "male"}
              onChange={handleChange}
            />{" "}
            Мужской
          </label>
          <label>
            <input
              type="radio"
              name="gender"
              value="female"
              checked={form.gender === "female"}
              onChange={handleChange}
            />{" "}
            Женский
          </label>
        </fieldset>

        <fieldset className="radio-group">
          <legend>Уровень игры</legend>
          <label>
            <input
              type="radio"
              name="level"
              value="novice"
              checked={form.level === "novice"}
              onChange={handleChange}
            />{" "}
            Новичок
          </label>
          <label>
            <input
              type="radio"
              name="level"
              value="middle"
              checked={form.level === "middle"}
              onChange={handleChange}
            />{" "}
            Средний
          </label>
          <label>
            <input
              type="radio"
              name="level"
              value="pro"
              checked={form.level === "pro"}
              onChange={handleChange}
            />{" "}
            Профи
          </label>
        </fieldset>

        <button type="submit" className="btn">
          Отправить заявку
        </button>
      </form>

      {/* Тема 5: сообщение о валидации рендерится условно */}
      {status.message && (
        <div
          id="form-status"
          className={
            status.type === "success" ? "status-success" : "status-error"
          }
        >
          {status.message}
        </div>
      )}
    </section>
  );
}

export default RequestForm;
