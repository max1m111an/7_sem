import { useEffect, useRef, useState } from "react";
import Reveal from "./Reveal.jsx";
import Icon from "./Icon.jsx";

const LEAVE_MS = 320;

function Favorites({ favorites, kinds = {}, onRemove, onClear }) {
  const [leaving, setLeaving] = useState([]);
  const timers = useRef([]);

  useEffect(() => {
    const pending = timers.current;
    return () => pending.forEach(clearTimeout);
  }, []);

  // Сначала проигрываем анимацию исчезновения, потом удаляем из состояния
  function handleRemove(title) {
    if (leaving.includes(title)) return;
    setLeaving((prev) => [...prev, title]);
    timers.current.push(
      setTimeout(() => {
        onRemove(title);
        setLeaving((prev) => prev.filter((item) => item !== title));
      }, LEAVE_MS)
    );
  }

  function handleClear() {
    setLeaving(favorites);
    timers.current.push(
      setTimeout(() => {
        onClear();
        setLeaving([]);
      }, LEAVE_MS)
    );
  }

  return (
    <Reveal as="section" className="favorites" id="favorites">
      <div className="favorites-head">
        <h2>
          Избранное
          <span className="count-pill" key={favorites.length}>
            {favorites.length}
          </span>
        </h2>
        {favorites.length > 1 && (
          <button type="button" className="clear-btn" onClick={handleClear}>
            <Icon name="trash" size={16} />
            Очистить всё
          </button>
        )}
      </div>

      {favorites.length === 0 && (
        <p id="favorites-empty">
          <span className="empty-heart" aria-hidden="true">♡</span>
          Пока пусто — нажмите на сердечко у игры или услуги.
        </p>
      )}

      <ul id="favorites-list">
        {favorites.map((title) => (
          <li
            key={title}
            data-title={title}
            className={leaving.includes(title) ? "is-leaving" : ""}
          >
            <span className="fav-title">
              <Icon name="heart" size={16} className="fav-heart" />
              {title}
              {kinds[title] && <span className="fav-kind">{kinds[title]}</span>}
            </span>
            <button
              type="button"
              className="remove-fav-btn"
              onClick={() => handleRemove(title)}
            >
              Удалить
            </button>
          </li>
        ))}
      </ul>
    </Reveal>
  );
}

export default Favorites;
