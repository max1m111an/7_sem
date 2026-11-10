import { useEffect, useRef } from "react";

// Анимированный фон: неоновые «сферы», перспективная сетка, скан-линии
// и мягкое свечение, которое следует за курсором.
function Background() {
  const ref = useRef(null);

  useEffect(() => {
    const node = ref.current;
    if (!node) return undefined;

    let frame = 0;
    let x = window.innerWidth / 2;
    let y = window.innerHeight / 3;

    function apply() {
      frame = 0;
      node.style.setProperty("--cursor-x", `${x}px`);
      node.style.setProperty("--cursor-y", `${y}px`);
    }

    function handleMove(event) {
      x = event.clientX;
      y = event.clientY;
      if (!frame) frame = requestAnimationFrame(apply);
    }

    apply();
    window.addEventListener("pointermove", handleMove, { passive: true });
    return () => {
      window.removeEventListener("pointermove", handleMove);
      cancelAnimationFrame(frame);
    };
  }, []);

  return (
    <div className="bg-fx" ref={ref} aria-hidden="true">
      <div className="bg-orb bg-orb--cyan" />
      <div className="bg-orb bg-orb--pink" />
      <div className="bg-orb bg-orb--violet" />
      <div className="bg-grid" />
      <div className="bg-spotlight" />
      <div className="bg-scanlines" />
    </div>
  );
}

export default Background;
