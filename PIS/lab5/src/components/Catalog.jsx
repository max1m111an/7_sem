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

export const GAMES = [
  { id: "cs2", title: "Counter-Strike 2", genre: "shooter", price: 250, rating: 4.9, format: "5 × 5", hot: true },
  { id: "dota2", title: "Dota 2", genre: "moba", price: 220, rating: 4.8, format: "5 × 5", hot: true },
  { id: "valorant", title: "Valorant", genre: "shooter", price: 230, rating: 4.7, format: "5 × 5" },
  { id: "lol", title: "League of Legends", genre: "moba", price: 200, rating: 4.5, format: "5 × 5" },
  { id: "apex", title: "Apex Legends", genre: "shooter", price: 210, rating: 4.4, format: "3 × 3" },
  { id: "tekken8", title: "Tekken 8", genre: "fighting", price: 180, rating: 4.6, format: "1 × 1" },
  { id: "sf6", title: "Street Fighter 6", genre: "fighting", price: 180, rating: 4.5, format: "1 × 1" },
  { id: "forza", title: "Forza Horizon 5", genre: "racing", price: 190, rating: 4.6, format: "до 12" },
  { id: "f1", title: "F1 24", genre: "racing", price: 200, rating: 4.2, format: "до 20" },
  { id: "fc", title: "EA SPORTS FC", genre: "sport", price: 170, rating: 4.3, format: "1 × 1", hot: true },
  { id: "nba", title: "NBA 2K", genre: "sport", price: 170, rating: 4.1, format: "1 × 1" },
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
        <span className="game-genre">{GENRE_LABEL[game.genre]}</span>
        <h3>{game.title}</h3>
        <div className="game-meta">
          <span className="game-rating" title="Рейтинг">
            <Icon name="star" size={15} /> {game.rating.toFixed(1)}
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

// Игры показываем по рейтингу — от самых популярных
const SORTED_GAMES = [...GAMES].sort((a, b) => b.rating - a.rating);

function Catalog({ favorites, onToggleFavorite }) {
  return (
    <div className="section-block catalog" id="catalog">
      <Reveal className="section-head">
        <span className="section-kicker">Каталог</span>
        <h2>Во что сыграем?</h2>
        <p>Отмечайте сердечком игры, которые хотите попробовать.</p>
      </Reveal>

      <div className="games-grid">
        {SORTED_GAMES.map((game, index) => (
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

export default Catalog;
