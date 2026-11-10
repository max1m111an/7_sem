import { useEffect, useState } from "react";
import Reveal from "./Reveal.jsx";
import Icon from "./Icon.jsx";

const REVIEWS_URL = "https://jsonplaceholder.typicode.com/comments?_limit=10";
const AUTOPLAY_MS = 7000;

function starsFor(review) {
  return 4 + (review.id % 2);
}

function Review() {
  const [reviews, setReviews] = useState([]);
  const [current, setCurrent] = useState(-1);
  const [direction, setDirection] = useState("next");
  const [status, setStatus] = useState("idle");
  const [error, setError] = useState("");
  const [isPaused, setIsPaused] = useState(false);

  const currentReview = current >= 0 ? reviews[current] : null;

  async function ensureLoaded() {
    if (reviews.length > 0) return reviews;

    setStatus("loading");
    const response = await fetch(REVIEWS_URL);
    if (!response.ok) {
      throw new Error("Не удалось загрузить отзывы: " + response.status);
    }
    const loaded = await response.json();
    setReviews(loaded);
    return loaded;
  }

  async function go(step) {
    try {
      const list = await ensureLoaded();
      setDirection(step < 0 ? "prev" : "next");
      setCurrent((prev) => (prev + step + list.length) % list.length);
      setStatus("ready");
      setError("");
    } catch (err) {
      setError(err.message);
      setStatus("error");
    }
  }

  function goTo(index) {
    setDirection(index < current ? "prev" : "next");
    setCurrent(index);
  }

  // Автопрокрутка, пока пользователь не навёл курсор
  useEffect(() => {
    if (status !== "ready" || isPaused || reviews.length < 2) return undefined;
    const timeoutId = setTimeout(() => {
      setDirection("next");
      setCurrent((prev) => (prev + 1) % reviews.length);
    }, AUTOPLAY_MS);
    return () => clearTimeout(timeoutId);
  }, [status, isPaused, current, reviews.length]);

  return (
    <Reveal
      as="section"
      className="review"
      onMouseEnter={() => setIsPaused(true)}
      onMouseLeave={() => setIsPaused(false)}
    >
      <div className="review-head">
        <h2>Отзыв клиента</h2>
        {currentReview && (
          <div className="review-nav">
            <button type="button" className="icon-btn" onClick={() => go(-1)} aria-label="Предыдущий отзыв">
              <Icon name="arrowLeft" size={18} />
            </button>
            <span className="review-counter">
              {current + 1} / {reviews.length}
            </span>
            <button type="button" className="icon-btn" onClick={() => go(1)} aria-label="Следующий отзыв">
              <Icon name="arrowRight" size={18} />
            </button>
          </div>
        )}
      </div>

      <div id="review-content" className={status === "loading" ? "is-loading" : ""}>
        {status === "loading" ? (
          <div className="skeleton" aria-label="Загрузка...">
            <span className="skeleton-circle" />
            <span className="skeleton-line" />
            <span className="skeleton-line" />
            <span className="skeleton-line short" />
          </div>
        ) : status === "error" ? (
          <p className="status-error">Ошибка: {error}</p>
        ) : currentReview ? (
          <div
            className={`review-slide review-slide--${direction}`}
            key={currentReview.id}
          >
            <div className="review-author-row">
              <span
                className="avatar"
                style={{ "--hue": (currentReview.id * 47) % 360 }}
              >
                {currentReview.email[0].toUpperCase()}
              </span>
              <div>
                <p className="review-author">Автор: {currentReview.email}</p>
                <span className="stars" aria-label={`Оценка ${starsFor(currentReview)} из 5`}>
                  {Array.from({ length: 5 }, (_, i) => (
                    <span
                      key={i}
                      className={i < starsFor(currentReview) ? "star is-on" : "star"}
                      style={{ "--i": i }}
                    >
                      ★
                    </span>
                  ))}
                </span>
              </div>
            </div>
            <p className="review-body">
              <span className="quote-mark" aria-hidden="true">“</span>
              {currentReview.body}
            </p>
          </div>
        ) : (
          <p className="review-placeholder">
            Нажмите кнопку, чтобы загрузить отзыв
          </p>
        )}
      </div>

      {reviews.length > 1 && status === "ready" && (
        <div className="dots" role="tablist" aria-label="Отзывы">
          {reviews.map((review, index) => (
            <button
              key={review.id}
              type="button"
              role="tab"
              aria-selected={index === current}
              aria-label={`Отзыв ${index + 1}`}
              className={index === current ? "dot is-active" : "dot"}
              onClick={() => goTo(index)}
            >
              {index === current && !isPaused && (
                <span
                  className="dot-progress"
                  style={{ animationDuration: `${AUTOPLAY_MS}ms` }}
                />
              )}
            </button>
          ))}
        </div>
      )}

      <button
        type="button"
        id="load-review-btn"
        className="btn"
        onClick={() => go(1)}
        disabled={status === "loading"}
      >
        {status === "loading"
          ? "Загрузка..."
          : currentReview
            ? "Следующий отзыв"
            : "Показать отзыв"}
      </button>
    </Reveal>
  );
}

export default Review;
