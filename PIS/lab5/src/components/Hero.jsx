import { useEffect, useState } from "react";

const PHRASES = [
  "Игры, турниры и мерч для геймеров.",
  "Турниры каждую неделю.",
  "Мощные ПК и консоли нового поколения.",
  "Комьюнити, которое играет вместе.",
];

function useTypewriter(phrases, { typeSpeed = 55, deleteSpeed = 26, pause = 1900 } = {}) {
  const [index, setIndex] = useState(0);
  const [text, setText] = useState("");
  const [isDeleting, setIsDeleting] = useState(false);

  const reduceMotion =
    typeof window !== "undefined" &&
    window.matchMedia?.("(prefers-reduced-motion: reduce)").matches;

  useEffect(() => {
    if (reduceMotion) return undefined;

    const full = phrases[index];
    let delay = isDeleting ? deleteSpeed : typeSpeed;
    let step;

    if (!isDeleting && text === full) {
      delay = pause;
      step = () => setIsDeleting(true);
    } else if (isDeleting && text === "") {
      delay = 350;
      step = () => {
        setIsDeleting(false);
        setIndex((prev) => (prev + 1) % phrases.length);
      };
    } else {
      step = () =>
        setText(full.slice(0, text.length + (isDeleting ? -1 : 1)));
    }

    const timeoutId = setTimeout(step, delay);
    return () => clearTimeout(timeoutId);
  }, [text, isDeleting, index, phrases, typeSpeed, deleteSpeed, pause, reduceMotion]);

  return reduceMotion ? phrases[0] : text;
}

// «Живой» счётчик игроков онлайн — небольшое случайное блуждание
function useOnlineCounter(start = 137) {
  const [online, setOnline] = useState(start);

  useEffect(() => {
    const intervalId = setInterval(() => {
      setOnline((prev) => {
        const next = prev + Math.round((Math.random() - 0.45) * 6);
        return Math.min(Math.max(next, 96), 214);
      });
    }, 2200);
    return () => clearInterval(intervalId);
  }, []);

  return online;
}

function Hero({ onShowAbout, onOpenModal, onExplore }) {
  const typed = useTypewriter(PHRASES);
  const online = useOnlineCounter();

  return (
    <section className="hero" id="home">
      <div className="hero-glow" aria-hidden="true" />

      <span className="hero-kicker">
        <span className="pulse-dot" aria-hidden="true" />
        Сейчас в клубе: <strong key={online}>{online}</strong> игроков
      </span>

      <h2 className="hero-title">
        Добро пожаловать <span className="gradient-text">на новый уровень</span>
      </h2>

      <p aria-label={PHRASES[0]}>
        <span aria-hidden="true">{typed}</span>
        <span className="caret" aria-hidden="true" />
      </p>

      <div className="hero-actions">
        <button type="button" className="btn btn-glow" onClick={onExplore}>
          Выбрать игру
        </button>
        <button type="button" className="btn" onClick={onShowAbout}>
          О нас
        </button>
        <button
          type="button"
          className="btn btn-outline"
          onClick={onOpenModal}
        >
          Правила акции
        </button>
      </div>

      <div className="hero-scroll" aria-hidden="true">
        <span />
      </div>
    </section>
  );
}

export default Hero;
