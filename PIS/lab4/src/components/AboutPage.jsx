// Тема 1 + 5 + 6 + 11: вторая страница сайта.
// Показывается вместо главной по state в App, без перезагрузки.
// Список фактов рендерится через .map() с key.

const ABOUT_FACTS = [
  {
    id: "founded",
    title: "С 2018 года",
    text: "GameZone вырос из небольшого клуба в площадку для турниров.",
  },
  {
    id: "events",
    title: "120+ турниров",
    text: "Каждый месяц проводим соревнования по популярным дисциплинам.",
  },
  {
    id: "community",
    title: "Дружное комьюнити",
    text: "Игроки, тренеры и болельщики — все на одной волне.",
  },
];

function AboutPage({ onBack, onOpenModal }) {
  return (
    <main>
      <section className="hero">
        <p>GameZone — клуб, где игры становятся событиями.</p>
      </section>

      <section className="features">
        {ABOUT_FACTS.map((fact) => (
          <article className="feature-card" key={fact.id}>
            <h3>{fact.title}</h3>
            <p>{fact.text}</p>
          </article>
        ))}
      </section>

      <section className="favorites about-cta">
        <h2>Как к нам попасть</h2>
        <p>
          Оставьте заявку на главной странице — мы свяжемся с вами, подберём
          формат игры и расскажем про ближайшие турниры.
        </p>

        <div className="hero-actions">
          <button type="button" className="btn" onClick={onBack}>
            На главную
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
    </main>
  );
}

export default AboutPage;
