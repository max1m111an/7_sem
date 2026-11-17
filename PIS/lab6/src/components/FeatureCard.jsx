import { useRef } from "react";
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
  const cardRef = useRef(null);

  function setRefs(node) {
    cardRef.current = node;
    revealRef.current = node;
  }

  // 3D-наклон карточки за курсором
  function handlePointerMove(event) {
    const card = cardRef.current;
    if (!card || event.pointerType === "touch") return;
    const rect = card.getBoundingClientRect();
    const x = (event.clientX - rect.left) / rect.width;
    const y = (event.clientY - rect.top) / rect.height;
    card.style.setProperty("--rx", `${(0.5 - y) * 10}deg`);
    card.style.setProperty("--ry", `${(x - 0.5) * 12}deg`);
    card.style.setProperty("--gx", `${x * 100}%`);
    card.style.setProperty("--gy", `${y * 100}%`);
  }

  function handlePointerLeave() {
    const card = cardRef.current;
    if (!card) return;
    card.style.setProperty("--rx", "0deg");
    card.style.setProperty("--ry", "0deg");
  }

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
      ref={setRefs}
      className={classes}
      style={{ "--reveal-delay": `${index * 120}ms` }}
      onClick={() => onSelect(feature.id)}
      onPointerMove={handlePointerMove}
      onPointerLeave={handlePointerLeave}
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
