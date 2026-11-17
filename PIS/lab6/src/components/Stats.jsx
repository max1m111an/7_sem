import { useEffect, useState } from "react";
import useReveal from "../hooks/useReveal.js";
import Icon from "./Icon.jsx";

const DEFAULT_STATS = [
  { id: "events", icon: "trophy", value: 120, suffix: "+", label: "турниров в год" },
  { id: "players", icon: "users", value: 5400, suffix: "+", label: "игроков в комьюнити" },
  { id: "pcs", icon: "gamepad", value: 48, suffix: "", label: "игровых мест" },
  { id: "hours", icon: "clock", value: 24, suffix: "/7", label: "мы открыты" },
];

function prefersReducedMotion() {
  return window.matchMedia?.("(prefers-reduced-motion: reduce)").matches;
}

// Число, которое «досчитывает» до значения, когда блок становится видимым.
function CountUp({ value, start, duration = 1600 }) {
  const [current, setCurrent] = useState(0);

  useEffect(() => {
    if (!start) return undefined;
    if (prefersReducedMotion()) {
      setCurrent(value);
      return undefined;
    }

    let frame = 0;
    const startTime = performance.now();

    function tick(now) {
      const progress = Math.min((now - startTime) / duration, 1);
      const eased = 1 - Math.pow(1 - progress, 3);
      setCurrent(Math.round(value * eased));
      if (progress < 1) frame = requestAnimationFrame(tick);
    }

    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, [value, start, duration]);

  return current.toLocaleString("ru-RU");
}

function Stats({ stats = DEFAULT_STATS }) {
  const [ref, isVisible] = useReveal({ threshold: 0.3 });

  return (
    <section
      ref={ref}
      className={isVisible ? "stats is-visible" : "stats"}
      aria-label="GameZone в цифрах"
    >
      {stats.map((stat, index) => (
        <div
          className="stat"
          key={stat.id}
          style={{ "--reveal-delay": `${index * 110}ms` }}
        >
          <span className="stat-icon">
            <Icon name={stat.icon} size={22} />
          </span>
          <span className="stat-value">
            <CountUp value={stat.value} start={isVisible} />
            {stat.suffix}
          </span>
          <span className="stat-label">{stat.label}</span>
        </div>
      ))}
    </section>
  );
}

export default Stats;
