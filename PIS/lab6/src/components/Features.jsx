import FeatureCard from "./FeatureCard.jsx";
import Reveal from "./Reveal.jsx";

function Features({
  features,
  selectedIds,
  favorites,
  onSelect,
  onToggleFavorite,
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

export default Features;
