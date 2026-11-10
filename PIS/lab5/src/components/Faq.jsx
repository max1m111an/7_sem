import { useState } from "react";
import Reveal from "./Reveal.jsx";
import Icon from "./Icon.jsx";

const QUESTIONS = [
  {
    id: "age",
    q: "С какого возраста можно прийти в клуб?",
    a: "С 14 лет — самостоятельно, младше — вместе со взрослым. Для турниров действуют отдельные правила, их публикуем в анонсе.",
  },
  {
    id: "own",
    q: "Можно ли играть со своей периферией?",
    a: "Да, приносите свою мышь, клавиатуру или геймпад — подключим за минуту. Наушники и коврики у нас тоже есть.",
  },
  {
    id: "booking",
    q: "Как забронировать место?",
    a: "Оставьте заявку в форме ниже — администратор перезвонит, подберёт время и зону: общий зал, VIP или консольную.",
  },
  {
    id: "coupon",
    q: "Как работает купон по акции?",
    a: "Отправьте заявку, пока идёт таймер. Купон появится в блоке акции — назовите его администратору при оплате.",
  },
];

function Faq() {
  const [openId, setOpenId] = useState(QUESTIONS[0].id);

  return (
    <section className="section-block faq" id="faq">
      <Reveal className="section-head">
        <span className="section-kicker">FAQ</span>
        <h2>Частые вопросы</h2>
      </Reveal>

      <div className="faq-list">
        {QUESTIONS.map((item, index) => {
          const isOpen = openId === item.id;
          return (
            <Reveal
              key={item.id}
              className={isOpen ? "faq-item is-open" : "faq-item"}
              delay={index * 90}
            >
              <button
                type="button"
                className="faq-question"
                aria-expanded={isOpen}
                aria-controls={`faq-${item.id}`}
                onClick={() => setOpenId(isOpen ? null : item.id)}
              >
                <span>{item.q}</span>
                <Icon name="chevron" size={20} className="faq-chevron" />
              </button>
              <div className="faq-answer" id={`faq-${item.id}`} role="region">
                <div>
                  <p>{item.a}</p>
                </div>
              </div>
            </Reveal>
          );
        })}
      </div>
    </section>
  );
}

export default Faq;
