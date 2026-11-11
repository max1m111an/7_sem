import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { Link, Route, Routes, useLocation, useNavigate } from "react-router-dom";
import Header from "./components/Header.jsx";
import Hero from "./components/Hero.jsx";
import Features from "./components/Features.jsx";
import Catalog, { GAMES } from "./components/Catalog.jsx";
import Favorites from "./components/Favorites.jsx";
import Stats from "./components/Stats.jsx";
import Promo from "./components/Promo.jsx";
import Review from "./components/Review.jsx";
import Faq from "./components/Faq.jsx";
import RequestForm from "./components/RequestForm.jsx";
import Footer from "./components/Footer.jsx";
import Modal from "./components/Modal.jsx";
import AboutPage from "./components/AboutPage.jsx";
import Background from "./components/Background.jsx";
import ScrollUI from "./components/ScrollUI.jsx";
import Toasts from "./components/Toasts.jsx";
import Confetti from "./components/Confetti.jsx";
import useLocalStorage from "./hooks/useLocalStorage.js";
import "./task_itog.css";

const NAV_ITEMS = [
  { label: "Главная", target: "home", to: "/" },
  { label: "О нас", target: "about", to: "/about" },
  { label: "Услуги", target: "features", to: "/#features" },
  { label: "Игры", target: "catalog", to: "/#catalog" },
  { label: "Контакты", target: "contacts", to: "/#contacts" },
];

const SCROLL_TOP = "top";

// Секции главной страницы, за которыми следит «scroll spy»
const SPY_TARGETS = ["home", "features", "catalog", "contacts"];

const FEATURES = [
  {
    id: "quality",
    icon: "star",
    title: "Качество",
    text: "Мы тщательно следим за каждой деталью.",
  },
  {
    id: "speed",
    icon: "bolt",
    title: "Скорость",
    text: "Быстро решаем любые вопросы клиентов.",
  },
  {
    id: "care",
    icon: "heart",
    title: "Забота о клиенте",
    text: "Индивидуальный подход к каждому.",
  },
];

const KONAMI = [
  "ArrowUp", "ArrowUp", "ArrowDown", "ArrowDown",
  "ArrowLeft", "ArrowRight", "ArrowLeft", "ArrowRight", "KeyB", "KeyA",
];

const TOAST_MS = 3200;

function NotFoundPage() {
  return (
    <main className="page-enter">
      <section className="hero hero--compact">
        <div className="hero-glow" aria-hidden="true" />
        <h2 className="hero-title">
          404 — <span className="gradient-text">страница не найдена</span>
        </h2>
        <p>Похоже, такой страницы в GameZone нет.</p>
        <div className="hero-actions">
          <Link to="/" className="btn">
            Вернуться на главную
          </Link>
        </div>
      </section>
    </main>
  );
}

function App() {
  const navigate = useNavigate();
  const location = useLocation();

  const [activeItem, setActiveItem] = useState(NAV_ITEMS[0].label);

  const [selectedFeatureIds, setSelectedFeatureIds] = useState([]);
  const [favorites, setFavorites] = useLocalStorage("gamezone-favorites", []);

  const [couponGranted, setCouponGranted] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const [scrollRequest, setScrollRequest] = useState(null);
  const [toasts, setToasts] = useState([]);
  const [partyMode, setPartyMode] = useState(false);
  const [konamiBurst, setKonamiBurst] = useState(0);

  const toastId = useRef(0);
  const isAutoScrolling = useRef(false);
  const partyRef = useRef(false);

  // ---------- уведомления ----------
  const dismissToast = useCallback((id) => {
    setToasts((prev) =>
      prev.map((toast) => (toast.id === id ? { ...toast, leaving: true } : toast))
    );
    setTimeout(() => {
      setToasts((prev) => prev.filter((toast) => toast.id !== id));
    }, 300);
  }, []);

  const notify = useCallback(
    (message, type = "info") => {
      toastId.current += 1;
      const id = toastId.current;
      setToasts((prev) => [...prev.slice(-3), { id, message, type }]);
      setTimeout(() => dismissToast(id), TOAST_MS);
    },
    [dismissToast]
  );

  // ---------- прокрутка ----------
  useEffect(() => {
    if (!scrollRequest) return undefined;

    isAutoScrolling.current = true;
    const timeoutId = setTimeout(() => {
      isAutoScrolling.current = false;
    }, 900);

    if (scrollRequest.target === SCROLL_TOP) {
      window.scrollTo({ top: 0, behavior: "smooth" });
    } else {
      document
        .getElementById(scrollRequest.target)
        ?.scrollIntoView({ behavior: "smooth", block: "start" });
    }

    return () => clearTimeout(timeoutId);
  }, [scrollRequest]);

  // Подсвечиваем пункт меню той секции, которая сейчас на экране
  useEffect(() => {
    if (location.pathname !== "/" || !("IntersectionObserver" in window)) return undefined;

    const observer = new IntersectionObserver(
      (entries) => {
        if (isAutoScrolling.current) return;
        const visible = entries
          .filter((entry) => entry.isIntersecting)
          .sort((a, b) => b.intersectionRatio - a.intersectionRatio)[0];
        if (!visible) return;
        const item = NAV_ITEMS.find((nav) => nav.target === visible.target.id);
        if (item) setActiveItem(item.label);
      },
      { rootMargin: "-35% 0px -55% 0px", threshold: [0, 0.25, 0.5] }
    );

    SPY_TARGETS.forEach((id) => {
      const node = document.getElementById(id);
      if (node) observer.observe(node);
    });

    return () => observer.disconnect();
  }, [location.pathname]);

  // Синхронизируем активный пункт меню с адресом (прямой заход,
  // кнопки браузера «назад/вперёд», переход по хэшу).
  useEffect(() => {
    if (location.pathname === "/about") {
      setActiveItem("О нас");
      return;
    }

    if (location.pathname === "/") {
      const hash = location.hash.replace("#", "");
      const item = NAV_ITEMS.find((nav) => nav.target === hash);
      setActiveItem(item ? item.label : "Главная");
    }
  }, [location.pathname, location.hash]);

  // Если открыли адрес с хэшем (например, /#catalog), прокручиваем к секции.
  useEffect(() => {
    if (location.pathname !== "/" || !location.hash) return;
    const target = location.hash.replace("#", "");
    if (document.getElementById(target)) {
      setScrollRequest({ target });
    }
  }, [location.pathname, location.hash]);

  // ---------- пасхалка: код Konami ----------
  // Общая функция: вызывается и с клавиатуры, и по клику на стрелки в подвале
  const triggerSecret = useCallback(() => {
    setKonamiBurst((prev) => prev + 1);
    partyRef.current = !partyRef.current;
    setPartyMode(partyRef.current);
    notify(
      partyRef.current ? "🎉 Секретный режим вечеринки!" : "Режим вечеринки выключен",
      "info"
    );
  }, [notify]);

  useEffect(() => {
    // Последние нажатые клавиши — сравниваем их «хвост» с кодом
    let history = [];

    function handleKey(event) {
      // event.code — физическая клавиша, поэтому код работает и на русской раскладке
      if (event.repeat) return;
      history = [...history, event.code].slice(-KONAMI.length);

      if (history.join() === KONAMI.join()) {
        history = [];
        triggerSecret();
      }
    }

    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [triggerSecret]);

  useEffect(() => {
    document.body.classList.toggle("party-mode", partyMode);
  }, [partyMode]);

  // Конфетти на весь экран после ввода кода — само исчезает через 4 секунды
  useEffect(() => {
    if (!konamiBurst) return undefined;
    const timeoutId = setTimeout(() => setKonamiBurst(0), 4000);
    return () => clearTimeout(timeoutId);
  }, [konamiBurst]);

  function requestScroll(target) {
    setScrollRequest({ target });
  }

  // Ссылки в Header сами меняют маршрут через <Link>; здесь только
  // обновляем подсветку и прокручиваем к нужной секции.
  function handleNavigate(item) {
    setActiveItem(item.label);

    if (item.target === "about") {
      requestScroll(SCROLL_TOP);
      return;
    }

    requestScroll(item.target === "home" ? SCROLL_TOP : item.target);
  }

  function handleShowAbout() {
    setActiveItem("О нас");
    requestScroll(SCROLL_TOP);
    navigate("/about");
  }

  function handleBackHome() {
    setActiveItem("Главная");
    requestScroll(SCROLL_TOP);
    navigate("/");
  }

  function goToSection(target) {
    const item = NAV_ITEMS.find((nav) => nav.target === target);
    if (item) setActiveItem(item.label);

    if (location.pathname !== "/") {
      navigate(`/#${target}`);
      return;
    }

    requestScroll(target);
  }

  // ---------- услуги и избранное ----------
  function handleSelectFeature(id) {
    setSelectedFeatureIds((prev) =>
      prev.includes(id) ? prev.filter((item) => item !== id) : [...prev, id]
    );
  }

  function handleToggleFavorite(title) {
    if (favorites.includes(title)) {
      setFavorites((prev) => prev.filter((item) => item !== title));
      notify(`«${title}» убрано из избранного`, "info");
    } else {
      setFavorites((prev) => (prev.includes(title) ? prev : [...prev, title]));
      notify(`«${title}» добавлено в избранное`, "fav");
    }
  }

  function handleRemoveFavorite(title) {
    setFavorites((prev) => prev.filter((item) => item !== title));
  }

  function handleClearFavorites() {
    setFavorites([]);
    notify("Избранное очищено", "info");
  }

  // Подпись типа элемента в списке избранного
  const favoriteKinds = useMemo(() => {
    const kinds = {};
    FEATURES.forEach((feature) => {
      kinds[feature.title] = "услуга";
    });
    GAMES.forEach((game) => {
      kinds[game.title] = "игра";
    });
    return kinds;
  }, []);

  function handleRequestSuccess() {
    setCouponGranted(true);
  }

  const openModal = useCallback(() => setIsModalOpen(true), []);
  const closeModal = useCallback(() => setIsModalOpen(false), []);

  return (
    <>
      <Background />
      <ScrollUI />

      <Header
        title="GameZone"
        navItems={NAV_ITEMS}
        activeItem={activeItem}
        onNavigate={handleNavigate}
        favoritesCount={favorites.length}
        onFavoritesClick={() => goToSection("favorites")}
      />

      <Routes>
        <Route
          path="/"
          element={
            <main className="page-enter" key="home">
              <Hero
                onShowAbout={handleShowAbout}
                onOpenModal={openModal}
                onExplore={() => goToSection("catalog")}
              />

              <Stats />

              <Features
                features={FEATURES}
                selectedIds={selectedFeatureIds}
                favorites={favorites}
                onSelect={handleSelectFeature}
                onToggleFavorite={handleToggleFavorite}
              />

              <Catalog
                favorites={favorites}
                onToggleFavorite={handleToggleFavorite}
              />

              <Favorites
                favorites={favorites}
                kinds={favoriteKinds}
                onRemove={handleRemoveFavorite}
                onClear={handleClearFavorites}
              />

              <Promo
                couponGranted={couponGranted}
                onOpenModal={openModal}
                onNotify={notify}
                onGoToForm={() => goToSection("contacts")}
              />

              <Review />

              <Faq />

              <RequestForm
                onRequestSuccess={handleRequestSuccess}
                onNotify={notify}
              />
            </main>
          }
        />

        <Route
          path="/about"
          element={
            <AboutPage
              key="about"
              onBack={handleBackHome}
              onOpenModal={openModal}
            />
          }
        />

        <Route path="*" element={<NotFoundPage />} />
      </Routes>

      <Footer onSecret={triggerSecret} />

      <Toasts toasts={toasts} onDismiss={dismissToast} />

      {konamiBurst > 0 && (
        <div className="confetti-screen" aria-hidden="true">
          <Confetti key={konamiBurst} count={140} />
        </div>
      )}

      <Modal
        isOpen={isModalOpen}
        title="Правила акции"
        onClose={closeModal}
      >
        <p>
          Оставьте заявку на главной странице, пока идёт таймер, и получите
          купон.
        </p>
        <ul className="modal-steps">
          <li>Заполните все поля формы заявки.</li>
          <li>Дождитесь сообщения об успешной отправке.</li>
          <li>Назовите купон при оформлении заказа.</li>
        </ul>
        <p>Одно нажатие на «Понятно» — и можно играть дальше.</p>
        <button type="button" className="btn" onClick={closeModal}>
          Понятно
        </button>
      </Modal>
    </>
  );
}

export default App;
