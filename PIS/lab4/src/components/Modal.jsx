// Тема 5 + 8 + 12: модальный контейнер.
// Открывается и закрывается через state в App, рендерится условно.
// Фон затемняется и размывается через CSS (backdrop-filter: blur).
// useEffect закрывает окно по Escape и блокирует прокрутку страницы.

import { useEffect } from "react";

function Modal({ isOpen, title, onClose, children }) {
  useEffect(() => {
    if (!isOpen) return undefined;

    function handleKeyDown(event) {
      if (event.key === "Escape") onClose();
    }

    document.addEventListener("keydown", handleKeyDown);
    document.body.style.overflow = "hidden";

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.body.style.overflow = "";
    };
  }, [isOpen, onClose]);

  // Тема 5: ранний return — пока окно закрыто, ничего не рендерим.
  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
        onClick={(event) => event.stopPropagation()}
      >
        <button
          type="button"
          className="modal-close"
          onClick={onClose}
          aria-label="Закрыть"
        >
          &times;
        </button>

        <h2 id="modal-title">{title}</h2>
        {children}
      </div>
    </div>
  );
}

export default Modal;
