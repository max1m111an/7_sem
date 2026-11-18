import { memo, useMemo } from "react";
import Reveal from "./Reveal.jsx";
import Icon from "./Icon.jsx";

export const GENRES = [
  { id: "all", label: "Все" },
  { id: "shooter", label: "Шутеры" },
  { id: "moba", label: "MOBA" },
  { id: "fighting", label: "Файтинги" },
  { id: "racing", label: "Гонки" },
  { id: "sport", label: "Спорт" },
];

const GENRE_LABEL = Object.fromEntries(GENRES.map((g) => [g.id, g.label]));

function initials(title) {
  return title
    .split(/\s+/)
    .filter((word) => /[A-Za-zА-Яа-я0-9]/.test(word[0]))
    .slice(0, 2)
    .map((word) => word[0])
    .join("")
    .toUpperCase();
}

function GameCard({ game, index, isFavorite, onToggleFavorite }) {
  return (
    <article
      className={`game-card genre-${game.genre}`}
      style={{ "--i": index }}
    >
      <div className="game-cover">
        <span className="game-initials">{initials(game.title)}</span>
        {game.hot && <span className="game-hot">🔥 Хит</span>}
        <button
          type="button"
          className={isFavorite ? "game-fav is-active" : "game-fav"}
          onClick={() => onToggleFavorite(game.title)}
          aria-label={isFavorite ? "Убрать из избранного" : "Добавить в избранное"}
          aria-pressed={isFavorite}
        >
          <Icon name="heart" size={18} />
        </button>
      </div>

      <div className="game-body">
        <span className="game-genre">{GENRE_LABEL[game.genre] ?? game.genre}</span>
        <h3>{game.title}</h3>
        <div className="game-meta">
          <span className="game-rating" title="Рейтинг">
            <Icon name="star" size={15} /> {Number(game.rating).toFixed(1)}
          </span>
          <span title="Формат матча">{game.format}</span>
        </div>
        <div className="game-footer">
          <span className="game-price">
            {game.price} ₽<small>/час</small>
          </span>
        </div>
      </div>
    </article>
  );
}

// Игры приходят с сервера; сортируем по рейтингу — от самых популярных.
function Catalog({ games = [], favorites, onToggleFavorite, loading, error, onRetry }) {
  const sortedGames = useMemo(
    () => [...games].sort((a, b) => Number(b.rating) - Number(a.rating)),
    [games]
  );

  return (
    <div className="section-block catalog" id="catalog">
      <Reveal className="section-head">
        <span className="section-kicker">Каталог</span>
        <h2>Во что сыграем?</h2>
        <p>Отмечайте сердечком игры, которые хотите попробовать.</p>
      </Reveal>

      {loading && <p className="catalog-note">Загрузка каталога...</p>}

      {error && (
        <p className="status-error catalog-note">
          Не удалось загрузить игры: {error}{" "}
          <button type="button" className="link-button" onClick={onRetry}>
            Повторить
          </button>
        </p>
      )}

      {!loading && !error && sortedGames.length === 0 && (
        <p className="catalog-note">В каталоге пока нет игр. Добавьте их в разделе «Игры (БД)».</p>
      )}

      <div className="games-grid">
        {sortedGames.map((game, index) => (
          <GameCard
            key={game.id}
            game={game}
            index={index}
            isFavorite={favorites.includes(game.title)}
            onToggleFavorite={onToggleFavorite}
          />
        ))}
      </div>
    </div>
  );
}

export default memo(Catalog);
