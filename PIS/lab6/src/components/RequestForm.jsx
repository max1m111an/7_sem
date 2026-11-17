import { useEffect, useState } from "react";
import Reveal from "./Reveal.jsx";
import Icon from "./Icon.jsx";

const STORAGE_KEY = "gamezone-request";

const EMPTY_FORM = {
  name: "",
  email: "",
  password: "",
  phone: "",
  gender: "",
  level: "",
};

const LEVELS = [
  { value: "novice", label: "Новичок", emoji: "🌱" },
  { value: "middle", label: "Средний", emoji: "⚔️" },
  { value: "pro", label: "Профи", emoji: "🏆" },
];

const STRENGTH = [
  { label: "", className: "" },
  { label: "Слабый", className: "weak" },
  { label: "Средний", className: "fair" },
  { label: "Хороший", className: "good" },
  { label: "Надёжный", className: "strong" },
];

// Маска телефона: +7 (999) 123-45-67
function formatPhone(raw) {
  let digits = raw.replace(/\D/g, "");
  if (!digits) return "";
  if (digits[0] === "8") digits = "7" + digits.slice(1);
  if (digits[0] !== "7") digits = "7" + digits;
  digits = digits.slice(0, 11);

  const parts = [digits.slice(1, 4), digits.slice(4, 7), digits.slice(7, 9), digits.slice(9, 11)];
  let result = "+7";
  if (parts[0]) result += ` (${parts[0]}`;
  if (parts[0].length === 3) result += ")";
  if (parts[1]) result += ` ${parts[1]}`;
  if (parts[2]) result += `-${parts[2]}`;
  if (parts[3]) result += `-${parts[3]}`;
  return result;
}

function passwordScore(password) {
  if (!password) return 0;
  let score = 0;
  if (password.length >= 6) score++;
  if (password.length >= 10) score++;
  if (/[A-ZА-Я]/.test(password) && /[a-zа-я]/.test(password)) score++;
  if (/\d/.test(password) && /[^A-Za-zА-Яа-я0-9]/.test(password)) score++;
  return Math.max(1, Math.min(score, 4));
}

function validate(data) {
  const errors = {};
  if (!data.name) errors.name = "Введите имя";
  if (!data.email) errors.email = "Введите email";
  else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(data.email)) {
    errors.email = "Email имеет неверный формат";
  }
  if (!data.password) errors.password = "Введите пароль";
  if (!data.phone) errors.phone = "Введите телефон";
  else if (!/^\+?[\d\s\-()]{10,}$/.test(data.phone)) {
    errors.phone = "Телефон имеет неверный формат";
  }
  if (!data.gender) errors.gender = "Выберите пол";
  if (!data.level) errors.level = "Выберите уровень игры";
  return errors;
}

function trimForm(form) {
  return {
    name: form.name.trim(),
    email: form.email.trim(),
    password: form.password.trim(),
    phone: form.phone.trim(),
    gender: form.gender,
    level: form.level,
  };
}

function Field({ name, label, error, children }) {
  return (
    <div className={error ? "field has-error" : "field"}>
      {children}
      <label htmlFor={`field-${name}`}>{label}</label>
      <span className="field-error" role={error ? "alert" : undefined}>
        {error}
      </span>
    </div>
  );
}

function RequestForm({ onRequestSuccess, onNotify }) {
  const [form, setForm] = useState(EMPTY_FORM);
  const [status, setStatus] = useState({ type: "", message: "" });
  const [errors, setErrors] = useState({});
  const [touched, setTouched] = useState({});
  const [showPassword, setShowPassword] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isShaking, setIsShaking] = useState(false);

  useEffect(() => {
    const savedData = localStorage.getItem(STORAGE_KEY);
    if (!savedData) return;

    try {
      const data = JSON.parse(savedData);
      setForm((prev) => ({ ...prev, ...data }));
    } catch {
      // повреждённые данные просто игнорируем
    }
  }, []);

  function handleChange(event) {
    const { name } = event.target;
    const value =
      name === "phone" ? formatPhone(event.target.value) : event.target.value;
    const next = { ...form, [name]: value };
    setForm(next);

    // Ошибку конкретного поля пересчитываем «на лету», если она уже показана
    if (errors[name] || touched[name]) {
      const fieldError = validate(trimForm(next))[name];
      setErrors((prev) => ({ ...prev, [name]: fieldError }));
    }

    if (status.message) {
      setStatus({ type: "", message: "" });
    }
  }

  function handleBlur(event) {
    const { name } = event.target;
    setTouched((prev) => ({ ...prev, [name]: true }));
    if (form[name]) {
      const fieldError = validate(trimForm(form))[name];
      setErrors((prev) => ({ ...prev, [name]: fieldError }));
    }
  }

  function handleSubmit(event) {
    event.preventDefault();
    if (isSubmitting) return;

    const data = trimForm(form);
    const nextErrors = validate(data);
    const messages = Object.values(nextErrors);

    if (messages.length > 0) {
      setErrors(nextErrors);
      setIsShaking(false);
      requestAnimationFrame(() => setIsShaking(true));
      setTimeout(() => setIsShaking(false), 600);
      setStatus({ type: "error", message: messages.join(". ") + "." });
      onNotify?.("Проверьте поля формы", "error");
      return;
    }

    // Имитация отправки на сервер
    setIsSubmitting(true);
    setTimeout(() => {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(data));
      setStatus({ type: "success", message: "Заявка успешно отправлена!" });
      setForm(EMPTY_FORM);
      setErrors({});
      setTouched({});
      setIsSubmitting(false);
      onNotify?.(`Спасибо, ${data.name}! Заявка отправлена`, "success");
      onRequestSuccess();
    }, 1100);
  }

  const score = passwordScore(form.password);
  const strength = STRENGTH[score];

  return (
    <Reveal as="section" className="form-section" id="contacts">
      <h2>Оставить заявку</h2>
      <p className="form-subtitle">
        Ответим в течение 15 минут и подберём удобное время.
      </p>

      <form
        id="request-form"
        action="/submit"
        method="POST"
        noValidate
        className={isShaking ? "is-shaking" : ""}
        onSubmit={handleSubmit}
      >
        <Field name="name" label="Ваше имя" error={errors.name}>
          <input
            id="field-name"
            type="text"
            name="name"
            placeholder=" "
            required
            autoComplete="name"
            value={form.name}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-invalid={Boolean(errors.name)}
          />
        </Field>

        <Field name="email" label="Email" error={errors.email}>
          <input
            id="field-email"
            type="email"
            name="email"
            placeholder=" "
            required
            autoComplete="email"
            value={form.email}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-invalid={Boolean(errors.email)}
          />
        </Field>

        <Field name="password" label="Пароль" error={errors.password}>
          <input
            id="field-password"
            type={showPassword ? "text" : "password"}
            name="password"
            placeholder=" "
            required
            autoComplete="new-password"
            value={form.password}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-invalid={Boolean(errors.password)}
          />
          <button
            type="button"
            className="field-action"
            onClick={() => setShowPassword((prev) => !prev)}
            aria-label={showPassword ? "Скрыть пароль" : "Показать пароль"}
          >
            <Icon name={showPassword ? "eyeOff" : "eye"} size={18} />
          </button>
        </Field>

        {form.password && (
          <div className={`strength strength--${strength.className}`}>
            <div className="strength-bars">
              {[1, 2, 3, 4].map((i) => (
                <span key={i} className={i <= score ? "is-on" : ""} />
              ))}
            </div>
            <span className="strength-label">{strength.label}</span>
          </div>
        )}

        <Field name="phone" label="Телефон" error={errors.phone}>
          <input
            id="field-phone"
            type="tel"
            name="phone"
            placeholder=" "
            required
            autoComplete="tel"
            inputMode="tel"
            value={form.phone}
            onChange={handleChange}
            onBlur={handleBlur}
            aria-invalid={Boolean(errors.phone)}
          />
        </Field>

        <fieldset
          className={errors.gender ? "radio-group chips-group has-error" : "radio-group chips-group"}
        >
          <legend>Пол</legend>
          <label>
            <input
              type="radio"
              name="gender"
              value="male"
              checked={form.gender === "male"}
              onChange={handleChange}
            />
            <span>Мужской</span>
          </label>
          <label>
            <input
              type="radio"
              name="gender"
              value="female"
              checked={form.gender === "female"}
              onChange={handleChange}
            />
            <span>Женский</span>
          </label>
        </fieldset>

        <fieldset
          className={errors.level ? "radio-group chips-group has-error" : "radio-group chips-group"}
        >
          <legend>Уровень игры</legend>
          {LEVELS.map((level) => (
            <label key={level.value}>
              <input
                type="radio"
                name="level"
                value={level.value}
                checked={form.level === level.value}
                onChange={handleChange}
              />
              <span>
                <span className="chip-emoji" aria-hidden="true">{level.emoji}</span>
                {level.label}
              </span>
            </label>
          ))}
        </fieldset>

        <button
          type="submit"
          className={isSubmitting ? "btn btn-glow is-loading" : "btn btn-glow"}
          disabled={isSubmitting}
        >
          {isSubmitting && <span className="spinner" aria-hidden="true" />}
          {isSubmitting ? "Отправляем..." : "Отправить заявку"}
        </button>
      </form>

      {status.message && (
        <div
          id="form-status"
          key={status.message}
          className={
            status.type === "success" ? "status-success" : "status-error"
          }
        >
          {status.type === "success" && (
            <Icon name="check" size={18} className="status-icon" />
          )}
          {status.message}
        </div>
      )}
    </Reveal>
  );
}

export default RequestForm;
