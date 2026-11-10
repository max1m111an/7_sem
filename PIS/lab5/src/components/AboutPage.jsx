import FeatureCard from "./FeatureCard.jsx";
import Reveal from "./Reveal.jsx";
import Stats from "./Stats.jsx";

const ABOUT_FACTS = [
  {
    id: "founded",
    icon: "clock",
    title: "С 2018 года",
    text: "GameZone вырос из небольшого клуба в площадку для турниров.",
  },
  {
    id: "events",
    icon: "trophy",
    title: "120+ турниров",
    text: "Каждый месяц проводим соревнования по популярным дисциплинам.",
  },
  {
    id: "community",
    icon: "users",
    title: "Дружное комьюнити",
    text: "Игроки, тренеры и болельщики — все на одной волне.",
  },
];

const TIMELINE = [
  { year: "2018", text: "Открыли первый зал на 12 мест." },
  { year: "2020", text: "Запустили онлайн-лигу и стримы турниров." },
  { year: "2023", text: "Переехали в новый зал: 48 мест, VIP и консольная зона." },
  { year: "2026", text: "Более 5 000 игроков в комьюнити и турниры каждую неделю." },
];

function AboutPage({ onBack, onOpenModal }) {
  return (
    <main className="page-enter">
      <section className="hero hero--compact">
        <div className="hero-glow" aria-hidden="true" />
        <h2 className="hero-title">
          О <span className="gradient-text">GameZone</span>
        </h2>
        <p>GameZone — клуб, где игры становятся событиями.</p>
      </section>

      <section className="features">
        {ABOUT_FACTS.map((fact, index) => (
          <FeatureCard
            key={fact.id}
            index={index}
            feature={fact}
            isSelected={false}
            onSelect={() => {}}
          />
        ))}
      </section>

      <Stats />

      <Reveal as="section" className="favorites timeline-section">
        <h2>Наша история</h2>
        <ol className="timeline">
          {TIMELINE.map((item, index) => (
            <Reveal as="li" key={item.year} delay={index * 120} variant="left">
              <span className="timeline-year">{item.year}</span>
              <p>{item.text}</p>
            </Reveal>
          ))}
        </ol>
      </Reveal>

      <Reveal as="section" className="favorites about-cta">
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
      </Reveal>
    </main>
  );
}

export default AboutPage;
