import { useEffect, useRef, useState } from "react";

const CLOSE_MS = 220;

function Modal({ isOpen, title, onClose, children }) {
  // Модалка остаётся в DOM ещё немного после закрытия — для анимации ухода
  const [isRendered, setIsRendered] = useState(isOpen);
  const [isClosing, setIsClosing] = useState(false);
  const closeRef = useRef(null);

  useEffect(() => {
    if (isOpen) {
      setIsRendered(true);
      setIsClosing(false);
      return undefined;
    }
    if (!isRendered) return undefined;

    setIsClosing(true);
    const timeoutId = setTimeout(() => {
      setIsRendered(false);
      setIsClosing(false);
    }, CLOSE_MS);
    return () => clearTimeout(timeoutId);
  }, [isOpen]);

  useEffect(() => {
    if (!isOpen) return undefined;

    function handleKeyDown(event) {
      if (event.key === "Escape") onClose();
    }

    document.addEventListener("keydown", handleKeyDown);
    document.body.style.overflow = "hidden";
    // Пока модалка открыта, страницу под ней не нужно рисовать:
    // экономим полный ре-рендер всех карточек на слабом iGPU.
    document.body.classList.add("modal-open");

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.body.style.overflow = "";
      document.body.classList.remove("modal-open");
    };
  }, [isOpen, onClose]);

  // Модалка монтируется только следующим рендером после isOpen,
  // поэтому фокус нужно наводить уже когда она реально в DOM.
  useEffect(() => {
    if (isOpen && isRendered) closeRef.current?.focus();
  }, [isOpen, isRendered]);

  if (!isRendered) return null;

  return (
    <div
      className={isClosing ? "modal-overlay is-closing" : "modal-overlay"}
      onClick={onClose}
    >
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
          ref={closeRef}
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
