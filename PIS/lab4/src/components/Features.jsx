// Тема 6: список карточек услуг через .map() с корректным key.
// Тема 9: состояние выбранных карточек и избранного живёт в App,
// а этот компонент только передаёт его вниз в FeatureCard.

import FeatureCard from "./FeatureCard.jsx";

function Features({
  features,
  selectedIds,
  favorites,
  onSelect,
  onAddFavorite,
}) {
  return (
    <section className="features" id="features">
      {features.map((feature) => (
        <FeatureCard
          key={feature.id}
          feature={feature}
          isSelected={selectedIds.includes(feature.id)}
          isFavorite={favorites.includes(feature.title)}
          onSelect={onSelect}
          onAddFavorite={onAddFavorite}
        />
      ))}
    </section>
  );
}

export default Features;
