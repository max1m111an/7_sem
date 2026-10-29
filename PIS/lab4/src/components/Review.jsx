// Тема 3 + 4 + 5: отзыв с сервера (Fetch API).
// Первый клик загружает данные, следующие клики листают отзывы.
// Загрузка/ошибка/готовый отзыв показываются условно.

import { useState } from "react";

const REVIEWS_URL = "https://jsonplaceholder.typicode.com/comments?_limit=10";

function Review() {
  const [reviews, setReviews] = useState([]);
  const [reviewIndex, setReviewIndex] = useState(0);
  const [currentReview, setCurrentReview] = useState(null);
  const [status, setStatus] = useState("idle");
  const [error, setError] = useState("");

  async function handleLoadReview() {
    try {
      let loadedReviews = reviews;

      if (loadedReviews.length === 0) {
        setStatus("loading");
        const response = await fetch(REVIEWS_URL);
        if (!response.ok) {
          throw new Error("Не удалось загрузить отзывы: " + response.status);
        }
        loadedReviews = await response.json();
        setReviews(loadedReviews);
      }

      setCurrentReview(loadedReviews[reviewIndex % loadedReviews.length]);
      setReviewIndex((prev) => prev + 1);
      setStatus("ready");
      setError("");
    } catch (err) {
      setError(err.message);
      setStatus("error");
    }
  }

  return (
    <section className="review">
      <h2>Отзыв клиента</h2>

      <div id="review-content">
        {status === "loading" ? (
          "Загрузка..."
        ) : status === "error" ? (
          <p className="status-error">Ошибка: {error}</p>
        ) : currentReview ? (
          <>
            <p>{currentReview.body}</p>
            <p className="review-author">Автор: {currentReview.email}</p>
          </>
        ) : (
          "Нажмите кнопку, чтобы загрузить отзыв"
        )}
      </div>

      <button
        type="button"
        id="load-review-btn"
        className="btn"
        onClick={handleLoadReview}
        disabled={status === "loading"}
      >
        {currentReview ? "Следующий отзыв" : "Показать отзыв"}
      </button>
    </section>
  );
}

export default Review;
