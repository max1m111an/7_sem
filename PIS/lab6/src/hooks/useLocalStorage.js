import { useEffect, useState } from "react";

// Состояние, которое переживает перезагрузку страницы.
function useLocalStorage(key, initialValue) {
  const [value, setValue] = useState(() => {
    try {
      const raw = localStorage.getItem(key);
      return raw ? JSON.parse(raw) : initialValue;
    } catch {
      return initialValue;
    }
  });

  useEffect(() => {
    try {
      localStorage.setItem(key, JSON.stringify(value));
    } catch {
      // хранилище недоступно (приватный режим) — просто работаем без него
    }
  }, [key, value]);

  return [value, setValue];
}

export default useLocalStorage;
