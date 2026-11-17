import { useEffect, useRef, useState } from "react";
import Reveal from "./Reveal.jsx";
import Confetti from "./Confetti.jsx";
import Icon from "./Icon.jsx";

const PROMO_DURATION = 5 * 60;
const COUPON = "GAMEZONE20";

function formatTime(totalSeconds) {
  const minutes = String(Math.floor(totalSeconds / 60)).padStart(2, "0");
  const seconds = String(totalSeconds % 60).padStart(2, "0");
  return `${minutes}:${seconds}`;
}

function Promo({ couponGranted, onOpenModal, onNotify, onGoToForm }) {
  const [secondsLeft, setSecondsLeft] = useState(PROMO_DURATION);
  const [burst, setBurst] = useState(0);
  const [copied, setCopied] = useState(false);
  const wasGranted = useRef(couponGranted);
  const isFinished = secondsLeft <= 0;
  const isUrgent = !couponGranted && !isFinished && secondsLeft <= 60;

  useEffect(() => {
    if (couponGranted || isFinished) return undefined;

    const intervalId = setInterval(() => {
      setSecondsLeft((prev) => (prev > 0 ? prev - 1 : 0));
    }, 1000);

    return () => clearInterval(intervalId);
  }, [couponGranted, isFinished]);

  // Конфетти в момент получения купона
  useEffect(() => {
    if (couponGranted && !wasGranted.current) {
      setBurst((prev) => prev + 1);
    }
    wasGranted.current = couponGranted;
  }, [couponGranted]);

  useEffect(() => {
    if (!burst) return undefined;
    const timeoutId = setTimeout(() => setBurst(0), 3200);
    return () => clearTimeout(timeoutId);
  }, [burst]);

  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(COUPON);
      setCopied(true);
      onNotify?.(`Купон ${COUPON} скопирован`, "success");
      setTimeout(() => setCopied(false), 1800);
    } catch {
      onNotify?.("Не удалось скопировать — выделите код вручную", "error");
    }
  }

  function handleRestart() {
    setSecondsLeft(PROMO_DURATION);
    onNotify?.("Таймер акции перезапущен — успейте оставить заявку!", "info");
  }

  const badgeText = couponGranted
    ? "Купон получен"
    : isFinished
      ? "Акция завершена"
      : "Топ выбор";

  const progress = Math.round((secondsLeft / PROMO_DURATION) * 100);

  const cardClass = [
    "promo-card",
    couponGranted ? "is-granted" : "",
    isUrgent ? "is-urgent" : "",
    isFinished && !couponGranted ? "is-finished" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <Reveal as="section" className="promo" variant="zoom">
      <div className={cardClass}>
        <span className="promo-shine" aria-hidden="true" />
        <span className="badge">{badgeText}</span>
        <h2>Специальное предложение</h2>

        <div className="promo-row">
          <span id="promo-timer" key={isUrgent ? secondsLeft : "t"}>
            {formatTime(secondsLeft)}
          </span>
          <span className="promo-discount">−20%</span>
        </div>

        <div className="promo-progress" aria-hidden="true">
          <span style={{ width: `${progress}%` }} />
        </div>

        <p>
          {couponGranted
            ? "Заявка принята — скидка закреплена за вами!"
            : isFinished
              ? "Время вышло, но акцию можно запустить ещё раз."
              : "Оставьте заявку прямо сейчас — расскажем обо всех деталях."}
        </p>

        <div className="promo-actions">
          {!couponGranted && !isFinished && (
            <button type="button" className="btn promo-btn btn-glow" onClick={onGoToForm}>
              Оставить заявку
            </button>
          )}
          {isFinished && !couponGranted && (
            <button type="button" className="btn promo-btn" onClick={handleRestart}>
              Запустить снова
            </button>
          )}
          <button
            type="button"
            className="btn promo-btn btn-outline"
            onClick={onOpenModal}
          >
            Правила акции
          </button>
        </div>

        {couponGranted && (
          <p id="promo-coupon">
            <span>
              Ваш купон на скидку 20%: <strong>{COUPON}</strong> — назовите его
              при оформлении!
            </span>
            <button
              type="button"
              className={copied ? "copy-btn is-copied" : "copy-btn"}
              onClick={handleCopy}
            >
              <Icon name={copied ? "check" : "copy"} size={16} />
              {copied ? "Скопировано" : "Копировать"}
            </button>
          </p>
        )}

        <Confetti key={10} />
      </div>
    </Reveal>
  );
}

export default Promo;
