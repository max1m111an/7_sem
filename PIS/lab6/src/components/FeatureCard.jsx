import useReveal from "../hooks/useReveal.js";
import Icon from "./Icon.jsx";

function FeatureCard({
  feature,
  index = 0,
  isSelected,
  isFavorite,
  onSelect,
  onToggleFavorite,
}) {
  const [revealRef, isVisible] = useReveal();

  const classes = [
    "feature-card",
    "tilt",
    "reveal",
    "reveal--up",
    isVisible ? "is-visible" : "",
    isSelected ? "selected" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div
      ref={revealRef}
      className={classes}
      style={{ "--reveal-delay": `${index * 120}ms` }}
      onClick={() => onSelect(feature.id)}
      role="button"
      tabIndex={0}
      aria-pressed={isSelected}
      onKeyDown={(event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          onSelect(feature.id);
        }
      }}
    >
      <span className="tilt-glare" aria-hidden="true" />
      {feature.icon && (
        <span className="feature-icon" aria-hidden="true">
          <Icon name={feature.icon} size={26} />
        </span>
      )}
      <h3>{feature.title}</h3>
      <p>{feature.text}</p>

      {onToggleFavorite && (
        <button
          type="button"
          className={isFavorite ? "add-fav-btn is-active" : "add-fav-btn"}
          onClick={(event) => {
            event.stopPropagation();
            onToggleFavorite(feature.title);
          }}
        >
          <Icon name={isFavorite ? "check" : "heart"} size={16} />
          {isFavorite ? "В избранном" : "В избранное"}
        </button>
      )}

      {isSelected && <span className="selected-mark" aria-hidden="true">✓</span>}
    </div>
  );
}

export default FeatureCard;
