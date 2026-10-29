import { useEffect, useState } from "react";
import Header from "./components/Header.jsx";
import Hero from "./components/Hero.jsx";
import Features from "./components/Features.jsx";
import Favorites from "./components/Favorites.jsx";
import Promo from "./components/Promo.jsx";
import Review from "./components/Review.jsx";
import RequestForm from "./components/RequestForm.jsx";
import Footer from "./components/Footer.jsx";
import Modal from "./components/Modal.jsx";
import AboutPage from "./components/AboutPage.jsx";
import "./task_itog.css";

const NAV_ITEMS = [
  { label: "Главная", target: "home" },
  { label: "О нас", target: "about" },
  { label: "Услуги", target: "features" },
  { label: "Контакты", target: "contacts" },
];

const SCROLL_TOP = "top";

const FEATURES = [
  {
    id: "quality",
    title: "Качество",
    text: "Мы тщательно следим за каждой деталью.",
  },
  {
    id: "speed",
    title: "Скорость",
    text: "Быстро решаем любые вопросы клиентов.",
  },
  {
    id: "care",
    title: "Забота о клиенте",
    text: "Индивидуальный подход к каждому.",
  },
];

function App() {
  const [page, setPage] = useState("home");
  const [activeItem, setActiveItem] = useState(NAV_ITEMS[0].label);

  const [selectedFeatureIds, setSelectedFeatureIds] = useState([]);
  const [favorites, setFavorites] = useState([]);

  const [couponGranted, setCouponGranted] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const [scrollRequest, setScrollRequest] = useState(null);

  useEffect(() => {
    if (!scrollRequest) return;

    if (scrollRequest.target === SCROLL_TOP) {
      window.scrollTo({ top: 0, behavior: "smooth" });
      return;
    }

    document
      .getElementById(scrollRequest.target)
      ?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, [scrollRequest]);

  function requestScroll(target) {
    setScrollRequest({ target });
  }

  function handleNavigate(item) {
    setActiveItem(item.label);

    if (item.target === "about") {
      setPage("about");
      requestScroll(SCROLL_TOP);
      return;
    }

    setPage("home");
    requestScroll(item.target === "home" ? SCROLL_TOP : item.target);
  }

  function handleShowAbout() {
    setActiveItem("О нас");
    setPage("about");
    requestScroll(SCROLL_TOP);
  }

  function handleBackHome() {
    setActiveItem("Главная");
    setPage("home");
    requestScroll(SCROLL_TOP);
  }

  function handleSelectFeature(id) {
    setSelectedFeatureIds((prev) =>
      prev.includes(id) ? prev.filter((item) => item !== id) : [...prev, id]
    );
  }

  function handleAddFavorite(title) {
    setFavorites((prev) =>
      prev.includes(title) ? prev : [...prev, title]
    );
  }

  function handleRemoveFavorite(title) {
    setFavorites((prev) => prev.filter((item) => item !== title));
  }

  function handleRequestSuccess() {
    setCouponGranted(true);
  }

  function openModal() {
    setIsModalOpen(true);
  }

  function closeModal() {
    setIsModalOpen(false);
  }

  return (
    <>
      <Header
        title="GameZone"
        navItems={NAV_ITEMS}
        activeItem={activeItem}
        onNavigate={handleNavigate}
      />

      {page === "home" ? (
        <main>
          <Hero onShowAbout={handleShowAbout} onOpenModal={openModal} />

          <Features
            features={FEATURES}
            selectedIds={selectedFeatureIds}
            favorites={favorites}
            onSelect={handleSelectFeature}
            onAddFavorite={handleAddFavorite}
          />

          <Favorites favorites={favorites} onRemove={handleRemoveFavorite} />

          <Promo
            couponGranted={couponGranted}
            onOpenModal={openModal}
          />

          <Review />

          <RequestForm onRequestSuccess={handleRequestSuccess} />
        </main>
      ) : (
        <AboutPage onBack={handleBackHome} onOpenModal={openModal} />
      )}

      <Footer />

      <Modal
        isOpen={isModalOpen}
        title="Правила акции"
        onClose={closeModal}
      >
        <p>
          Оставьте заявку на главной странице, пока идёт таймер, и получите
          купон.
        </p>
        <ul>
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
