import { memo } from "react";
import FeatureCard from "./FeatureCard.jsx";
import Reveal from "./Reveal.jsx";

function Features({
  features,
  selectedIds,
  favorites,
  onSelect,
  onToggleFavorite,
  loading,
  error,
  onRetry,
}) {
  return (
    <div className="section-block" id="features">
      <Reveal className="section-head">
        <span className="section-kicker">Почему мы</span>
        <h2>Наши услуги</h2>
        <p>
          Нажмите на карточку, чтобы отметить её, или добавьте в избранное.
          {selectedIds.length > 0 && (
            <span className="selected-count">
              {" "}
              Выбрано: {selectedIds.length}
            </span>
          )}
        </p>
      </Reveal>

      {loading && <p className="catalog-note">Загрузка услуг...</p>}

      {error && (
        <p className="status-error catalog-note">
          Не удалось загрузить услуги: {error}{" "}
          <button type="button" className="link-button" onClick={onRetry}>
            Повторить
          </button>
        </p>
      )}

      {!loading && !error && features.length === 0 && (
        <p className="catalog-note">Услуг пока нет. Добавьте их в разделе «Услуги (БД)».</p>
      )}

      <section className="features">
        {features.map((feature, index) => (
          <FeatureCard
            key={feature.id}
            index={index}
            feature={feature}
            isSelected={selectedIds.includes(feature.id)}
            isFavorite={favorites.includes(feature.title)}
            onSelect={onSelect}
            onToggleFavorite={onToggleFavorite}
          />
        ))}
      </section>
    </div>
  );
}

export default memo(Features);
