function Footer({ onSecret }) {
  return (
    <footer>
      <div className="footer-brand">
        <span className="gradient-text">GameZone</span>
        <span className="footer-tag">Играй. Побеждай. Повторяй.</span>
      </div>
      <p>&copy; 2026 GameZone. Учебный проект — лабораторная работа №5.</p>
      <button
        type="button"
        className="footer-secret"
        onClick={onSecret}
        title="Нажмите или наберите на клавиатуре"
      >
        ↑ ↑ ↓ ↓ ← → ← → B A
      </button>
    </footer>
  );
}

export default Footer;
