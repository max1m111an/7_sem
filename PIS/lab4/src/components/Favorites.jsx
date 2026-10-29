// Тема 5 + 6 + 9: список избранного.
// Данные приходят из общего state в App, поэтому список синхронизирован
// с карточками услуг — это подъём состояния между соседними блоками.

function Favorites({ favorites, onRemove }) {
  return (
    <section className="favorites">
      <h2>Избранное</h2>

      {/* Тема 5: условный рендеринг через && */}
      {favorites.length === 0 && <p id="favorites-empty">Пока пусто</p>}

      <ul id="favorites-list">
        {favorites.map((title) => (
          <li key={title} data-title={title}>
            {title}
            <button
              type="button"
              className="remove-fav-btn"
              onClick={() => onRemove(title)}
            >
              Удалить
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}

export default Favorites;
