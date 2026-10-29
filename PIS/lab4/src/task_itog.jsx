// ЛАБОРАТОРНАЯ РАБОТА №4
// ИТОГОВОЕ ЗАДАНИЕ — Вариант 04
// Тема варианта: «Игра» — GameZone
//
// Сайт из ЛР2/ЛР3 пересобран на React-компонентах с той же вёрсткой
// и тем же CSS. Корневой компонент хранит общее состояние страницы:
// - переключение главной и страницы «О нас» (state + условный рендеринг);
// - избранное и выбранные карточки (подъём состояния между соседями);
// - выдача купона после успешной заявки (форма + промо-блок);
// - открытие и закрытие модального окна.

import { useState } from "react";
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
  // Тема 3 + 11: текущая страница и активный пункт меню.
  const [page, setPage] = useState("home");
  const [activeItem, setActiveItem] = useState(NAV_ITEMS[0].label);

  // Тема 3 + 9: общее состояние карточек и списка избранного.
  const [selectedFeatureIds, setSelectedFeatureIds] = useState([]);
  const [favorites, setFavorites] = useState([]);

  // Тема 9 + 12: купон связывает форму и промо-блок, модалка — отдельный state.
  const [couponGranted, setCouponGranted] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);

  function handleNavigate(item) {
    setActiveItem(item.label);

    if (item.target === "about") {
      setPage("about");
      window.scrollTo({ top: 0, behavior: "smooth" });
      return;
    }

    setPage("home");

    if (item.target === "home") {
      window.scrollTo({ top: 0, behavior: "smooth" });
      return;
    }

    // Секция появится после переключения страницы — прокручиваем следом.
    setTimeout(() => {
      document
        .getElementById(item.target)
        ?.scrollIntoView({ behavior: "smooth", block: "start" });
    }, 0);
  }

  function handleShowAbout() {
    setActiveItem("О нас");
    setPage("about");
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

  function handleBackHome() {
    setActiveItem("Главная");
    setPage("home");
    window.scrollTo({ top: 0, behavior: "smooth" });
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

      {/* Тема 11: вторая страница через state + условный рендеринг */}
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

      {/* Тема 12: модальный контейнер поверх сайта */}
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
