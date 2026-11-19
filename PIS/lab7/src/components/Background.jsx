import { memo } from "react";

// Статичный фон: неоновые «сферы», перспективная сетка и скан-линии.
// Никаких анимаций и обработчиков движения мыши — всё рисуется один раз.
function Background() {
  return (
    <div className="bg-fx" aria-hidden="true">
      <div className="bg-orb bg-orb--cyan" />
      <div className="bg-orb bg-orb--pink" />
      <div className="bg-orb bg-orb--violet" />
      <div className="bg-grid" />
      <div className="bg-scanlines" />
    </div>
  );
}

export default memo(Background);
