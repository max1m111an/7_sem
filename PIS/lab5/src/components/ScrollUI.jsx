import { useEffect, useState } from "react";
import Icon from "./Icon.jsx";

// Полоса прогресса прокрутки сверху и кнопка «Наверх».
function ScrollUI() {
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    let frame = 0;

    function update() {
      frame = 0;
      const max =
        document.documentElement.scrollHeight - window.innerHeight;
      setProgress(max > 0 ? Math.min(window.scrollY / max, 1) : 0);
    }

    function handleScroll() {
      if (!frame) frame = requestAnimationFrame(update);
    }

    update();
    window.addEventListener("scroll", handleScroll, { passive: true });
    window.addEventListener("resize", handleScroll);
    return () => {
      window.removeEventListener("scroll", handleScroll);
      window.removeEventListener("resize", handleScroll);
      cancelAnimationFrame(frame);
    };
  }, []);

  return (
    <>
      <div
        className="scroll-progress"
        style={{ transform: `scaleX(${progress})` }}
        aria-hidden="true"
      />
      <button
        type="button"
        className={progress > 0.12 ? "to-top is-shown" : "to-top"}
        onClick={() => window.scrollTo({ top: 0, behavior: "smooth" })}
        aria-label="Наверх"
        tabIndex={progress > 0.12 ? 0 : -1}
      >
        <Icon name="arrowUp" size={20} />
        <svg className="to-top-ring" viewBox="0 0 48 48" aria-hidden="true">
          <circle
            cx="24"
            cy="24"
            r="22"
            pathLength="100"
            style={{ strokeDashoffset: 100 - progress * 100 }}
          />
        </svg>
      </button>
    </>
  );
}

export default ScrollUI;
