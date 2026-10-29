// Тема 3 + 5 + 8 + 9: промо-блок с живым таймером.
// Таймер — собственный useEffect с setInterval и очисткой.
// couponGranted приходит из App: форма и промо-блок — соседи,
// синхронизированные через общий state родителя.

import { useEffect, useState } from "react";

const PROMO_DURATION = 5 * 60;

function formatTime(totalSeconds) {
  const minutes = String(Math.floor(totalSeconds / 60)).padStart(2, "0");
  const seconds = String(totalSeconds % 60).padStart(2, "0");
  return `${minutes}:${seconds}`;
}

function Promo({ couponGranted, onOpenModal }) {
  const [secondsLeft, setSecondsLeft] = useState(PROMO_DURATION);
  const isFinished = secondsLeft <= 0;

  useEffect(() => {
    if (couponGranted || isFinished) return undefined;

    const intervalId = setInterval(() => {
      setSecondsLeft((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);

    return () => clearInterval(intervalId);
  }, [couponGranted, isFinished]);

  // Тема 5: вложенный тернарник выбирает подпись бейджа.
  const badgeText = couponGranted
    ? "Купон получен"
    : isFinished
      ? "Акция завершена"
      : "Топ выбор";

  // Динамическое значение для style={{}}: полоса прогресса таймера.
  const progress = Math.round((secondsLeft / PROMO_DURATION) * 100);

  return (
    <section className="promo">
      <div className="promo-card">
        <span className="badge">{badgeText}</span>
        <h2>Специальное предложение</h2>
        <span id="promo-timer">{formatTime(secondsLeft)}</span>

        <div className="promo-progress" aria-hidden="true">
          <span style={{ width: `${progress}%` }} />
        </div>

        <p>Оставьте заявку прямо сейчас — расскажем обо всех деталях.</p>

        <button
          type="button"
          className="btn promo-btn"
          onClick={onOpenModal}
        >
          Правила акции
        </button>

        {/* Тема 5: купон появляется только после успешной заявки */}
        {couponGranted && (
          <p id="promo-coupon">
            Ваш купон на скидку 20%: GAMEZONE20 — назовите его при оформлении!
          </p>
        )}
      </div>
    </section>
  );
}

export default Promo;
