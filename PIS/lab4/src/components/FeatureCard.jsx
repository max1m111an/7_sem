// Тема 2 + 5 + 6: переиспользуемая карточка услуги.
// Один и тот же компонент рендерится три раза с разными props.
// Условный рендеринг: класс selected и текст кнопки зависят от state.

function FeatureCard({
  feature,
  isSelected,
  isFavorite,
  onSelect,
  onAddFavorite,
}) {
  return (
    <div
      className={isSelected ? "feature-card selected" : "feature-card"}
      onClick={() => onSelect(feature.id)}
    >
      <h3>{feature.title}</h3>
      <p>{feature.text}</p>

      <button
        type="button"
        className="add-fav-btn"
        onClick={(event) => {
          // Клик по кнопке не должен выбирать карточку.
          event.stopPropagation();
          onAddFavorite(feature.title);
        }}
      >
        {isFavorite ? "В избранном" : "В избранное"}
      </button>
    </div>
  );
}

export default FeatureCard;
