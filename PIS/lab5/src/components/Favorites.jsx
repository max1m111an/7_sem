function Favorites({ favorites, onRemove }) {
  return (
    <section className="favorites">
      <h2>Избранное</h2>

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
