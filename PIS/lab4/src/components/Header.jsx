// Тема 1 + 2 + 4 + 8: шапка сайта.
// Получает пункты меню и активный пункт через props, а живые часы
// обновляет собственный useEffect с setInterval.

import { useEffect, useState } from "react";

function Header({ title, navItems, activeItem, onNavigate }) {
  const [now, setNow] = useState(() => new Date());

  useEffect(() => {
    const intervalId = setInterval(() => setNow(new Date()), 1000);
    return () => clearInterval(intervalId);
  }, []);

  const time = now.toLocaleTimeString("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });

  return (
    <header>
      <h1>{title}</h1>
      <span id="live-clock">{time}</span>
      <nav className="nav">
        {navItems.map((item) => (
          <a
            key={item.target}
            href={`#${item.target}`}
            className={item.label === activeItem ? "active" : ""}
            onClick={(event) => {
              event.preventDefault();
              onNavigate(item);
            }}
          >
            {item.label}
          </a>
        ))}
      </nav>
    </header>
  );
}

export default Header;
