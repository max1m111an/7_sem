import useReveal from "../hooks/useReveal.js";

// Обёртка, которая плавно «проявляет» содержимое при прокрутке.
function Reveal({
  as: Tag = "div",
  className = "",
  delay = 0,
  variant = "up",
  style,
  children,
  ...rest
}) {
  const [ref, isVisible] = useReveal();

  const classes = [
    "reveal",
    `reveal--${variant}`,
    isVisible ? "is-visible" : "",
    className,
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <Tag
      ref={ref}
      className={classes}
      style={{ ...style, "--reveal-delay": `${delay}ms` }}
      {...rest}
    >
      {children}
    </Tag>
  );
}

export default Reveal;
