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
