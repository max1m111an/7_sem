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
    closeRef.current?.focus();

    return () => {
      document.removeEventListener("keydown", handleKeyDown);
      document.body.style.overflow = "";
    };
  }, [isOpen, onClose]);

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
