function Hero({ onShowAbout, onOpenModal }) {
  return (
    <section className="hero">
      <p>Игры, турниры и мерч для геймеров.</p>

      <div className="hero-actions">
        <button type="button" className="btn" onClick={onShowAbout}>
          О нас
        </button>
        <button
          type="button"
          className="btn btn-outline"
          onClick={onOpenModal}
        >
          Правила акции
        </button>
      </div>
    </section>
  );
}

export default Hero;
