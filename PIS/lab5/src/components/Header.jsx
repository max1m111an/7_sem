import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { Link } from "react-router-dom";
import Icon from "./Icon.jsx";

function Header({
  title,
  navItems,
  activeItem,
  onNavigate,
  favoritesCount,
  onFavoritesClick,
}) {
  const [now, setNow] = useState(() => new Date());
  const [isScrolled, setIsScrolled] = useState(false);
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [indicator, setIndicator] = useState({ left: 0, width: 0, ready: false });
  const [bump, setBump] = useState(false);

  const navRef = useRef(null);
  const prevCount = useRef(favoritesCount);

  useEffect(() => {
    const intervalId = setInterval(() => setNow(new Date()), 1000);
    return () => clearInterval(intervalId);
  }, []);

  useEffect(() => {
    function handleScroll() {
      setIsScrolled(window.scrollY > 24);
    }
    handleScroll();
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  // «Подпрыгивание» счётчика избранного, когда добавили новый элемент
  useEffect(() => {
    if (favoritesCount > prevCount.current) {
      setBump(true);
      const timeoutId = setTimeout(() => setBump(false), 450);
      prevCount.current = favoritesCount;
      return () => clearTimeout(timeoutId);
    }
    prevCount.current = favoritesCount;
    return undefined;
  }, [favoritesCount]);

  // Подсветка-«ползунок» под активным пунктом меню
  useLayoutEffect(() => {
    function measure() {
      const nav = navRef.current;
      const link = nav?.querySelector("a.active");
      if (!nav || !link) {
        setIndicator((prev) => ({ ...prev, ready: false }));
        return;
      }
      setIndicator({ left: link.offsetLeft, width: link.offsetWidth, ready: true });
    }

    measure();
    window.addEventListener("resize", measure);
    return () => window.removeEventListener("resize", measure);
  }, [activeItem, isMenuOpen]);

  useEffect(() => {
    if (!isMenuOpen) return undefined;
    function handleKey(event) {
      if (event.key === "Escape") setIsMenuOpen(false);
    }
    document.addEventListener("keydown", handleKey);
    return () => document.removeEventListener("keydown", handleKey);
  }, [isMenuOpen]);

  const time = now.toLocaleTimeString("ru-RU", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });

  const headerClass = [
    isScrolled ? "is-scrolled" : "",
    isMenuOpen ? "menu-open" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <header className={headerClass}>
      <Link
        to={navItems[0].to}
        className="brand"
        onClick={() => {
          setIsMenuOpen(false);
          onNavigate(navItems[0]);
        }}
      >
        <span className="brand-logo" aria-hidden="true">
          <Icon name="gamepad" size={26} />
        </span>
        <h1 className="glitch" data-text={title}>
          {title}
        </h1>
      </Link>

      <span id="live-clock">
        <span className="clock-dot" aria-hidden="true" />
        {time}
      </span>

      <nav className="nav" ref={navRef} aria-label="Основное меню">
        <span
          className={indicator.ready ? "nav-indicator is-ready" : "nav-indicator"}
          style={{
            transform: `translateX(${indicator.left}px)`,
            width: indicator.width,
          }}
          aria-hidden="true"
        />
        {navItems.map((item) => (
          <Link
            key={item.target}
            to={item.to}
            className={item.label === activeItem ? "active" : ""}
            onClick={() => {
              setIsMenuOpen(false);
              onNavigate(item);
            }}
          >
            {item.label}
          </Link>
        ))}
      </nav>

      <div className="header-actions">
        <button
          type="button"
          className={bump ? "fav-counter is-bumping" : "fav-counter"}
          onClick={() => {
            setIsMenuOpen(false);
            onFavoritesClick();
          }}
          aria-label={`Избранное: ${favoritesCount}`}
        >
          <Icon name="heart" size={18} />
          <span>{favoritesCount}</span>
        </button>

        <button
          type="button"
          className="burger"
          aria-label={isMenuOpen ? "Закрыть меню" : "Открыть меню"}
          aria-expanded={isMenuOpen}
          onClick={() => setIsMenuOpen((open) => !open)}
        >
          <span />
          <span />
          <span />
        </button>
      </div>
    </header>
  );
}

export default Header;
