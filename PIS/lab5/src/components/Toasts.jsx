import Icon from "./Icon.jsx";

const ICONS = {
  success: "check",
  error: "close",
  info: "info",
  fav: "heart",
};

// Всплывающие уведомления в правом нижнем углу.
function Toasts({ toasts, onDismiss }) {
  return (
    <div className="toasts" role="status" aria-live="polite">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`toast toast--${toast.type}${
            toast.leaving ? " is-leaving" : ""
          }`}
          onClick={() => onDismiss(toast.id)}
        >
          <span className="toast-icon">
            <Icon name={ICONS[toast.type] || "info"} size={18} />
          </span>
          <span className="toast-text">{toast.message}</span>
          <span className="toast-timer" />
        </div>
      ))}
    </div>
  );
}

export default Toasts;
