// server.js — API итогового приложения GameZone (ЛР6).
//
// Два независимых объекта: games (каталог игр) и services (услуги клуба).
// Дополнительно — game_services (связка «игра входит в услугу») с двумя
// внешними ключами и JOIN-запросами.
//
// Настройки подключения — как заготовка в task.js: параметры PG берутся
// из .env, но имя базы для itog фиксированное (gamezone_db).
// Все запросы к PostgreSQL параметризованы: $1, $2, ...

import express from "express";
import cors from "cors";
import pg from "pg";
import { existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// Загружаем .env из папки с server.js (Node 20.12+ / 21.7+).
// Переменные, уже заданные в окружении, имеют приоритет над файлом.
const envPath = resolve(dirname(fileURLToPath(import.meta.url)), ".env");
if (existsSync(envPath)) {
  process.loadEnvFile(envPath);
}

const { Pool } = pg;

const PORT = Number(process.env.PORT) || 3000;

const pool = new Pool({
  host: process.env.PGHOST || "localhost",
  port: Number(process.env.PGPORT) || 5432,
  database: process.env.PGDATABASE_ITOG,
  user: process.env.PGUSER || "postgres",
  password: process.env.PGPASSWORD || "",
});

const app = express();
app.use(cors());
app.use(express.json());

// ---------- вспомогательные функции ----------

class HttpError extends Error {
  constructor(status, message) {
    super(message);
    this.status = status;
  }
}

const asyncHandler = (handler) => (req, res, next) =>
  Promise.resolve(handler(req, res, next)).catch(next);

const GAME_FIELDS = [
  { name: "title", type: "text" },
  { name: "genre", type: "text" },
  { name: "price", type: "int" },
  { name: "rating", type: "decimal" },
  { name: "match_format", type: "text" },
  { name: "is_hot", type: "bool" },
];

const SERVICE_FIELDS = [
  { name: "title", type: "text" },
  { name: "icon", type: "text" },
  { name: "description", type: "text" },
  { name: "price", type: "int" },
  { name: "duration_min", type: "int" },
  { name: "is_active", type: "bool" },
];

const GAME_SERVICE_FIELDS = [
  { name: "game_id", type: "int" },
  { name: "service_id", type: "int" },
  { name: "session_time", type: "time" },
  { name: "room", type: "text" },
];

function parseValue(type, raw, field) {
  if (type === "text") {
    const value = String(raw ?? "").trim();
    if (!value) throw new HttpError(400, `Поле "${field}" обязательно`);
    return value;
  }

  if (type === "int") {
    if (raw === "" || raw === null || raw === undefined) {
      throw new HttpError(400, `Поле "${field}" обязательно`);
    }
    const value = Number(raw);
    if (!Number.isInteger(value)) {
      throw new HttpError(400, `Поле "${field}" должно быть целым числом`);
    }
    return value;
  }

  if (type === "decimal") {
    if (raw === "" || raw === null || raw === undefined) {
      throw new HttpError(400, `Поле "${field}" обязательно`);
    }
    const value = Number(raw);
    if (!Number.isFinite(value)) {
      throw new HttpError(400, `Поле "${field}" должно быть числом`);
    }
    return value;
  }

  if (type === "bool") {
    if (typeof raw === "boolean") return raw;
    const value = String(raw ?? "").trim().toLowerCase();
    if (["true", "1", "on", "yes"].includes(value)) return true;
    if (["false", "0", "off", "no", ""].includes(value)) return false;
    throw new HttpError(400, `Поле "${field}" должно быть логическим значением`);
  }

  if (type === "time") {
    const value = String(raw ?? "").trim();
    if (!/^([01]\d|2[0-3]):[0-5]\d(:[0-5]\d)?$/.test(value)) {
      throw new HttpError(400, `Поле "${field}" должно содержать время (ЧЧ:ММ)`);
    }
    return value.length === 5 ? `${value}:00` : value;
  }

  throw new HttpError(500, `Неизвестный тип поля "${type}"`);
}

function valuesFromBody(body, fields) {
  return fields.map(({ name, type }) => parseValue(type, body?.[name], name));
}

function parseId(req) {
  const id = Number(req.params.id);
  if (!Number.isInteger(id) || id <= 0) {
    throw new HttpError(400, "Некорректный id");
  }
  return id;
}

// ---------- создание таблиц ----------

async function initDb() {
  await pool.query(`
    CREATE TABLE IF NOT EXISTS games (
      id SERIAL PRIMARY KEY,
      title VARCHAR(100) NOT NULL UNIQUE,
      genre VARCHAR(30) NOT NULL,
      price INTEGER NOT NULL CHECK (price >= 0),
      rating NUMERIC(3,1) NOT NULL CHECK (rating BETWEEN 0 AND 5),
      match_format VARCHAR(20) NOT NULL,
      is_hot BOOLEAN NOT NULL DEFAULT FALSE
    );
  `);

  await pool.query(`
    CREATE TABLE IF NOT EXISTS services (
      id SERIAL PRIMARY KEY,
      title VARCHAR(100) NOT NULL,
      icon VARCHAR(30) NOT NULL,
      description TEXT NOT NULL,
      price INTEGER NOT NULL CHECK (price >= 0),
      duration_min INTEGER NOT NULL,
      is_active BOOLEAN NOT NULL DEFAULT TRUE
    );
  `);

  await pool.query(`
    CREATE TABLE IF NOT EXISTS game_services (
      id SERIAL PRIMARY KEY,
      game_id INTEGER NOT NULL REFERENCES games(id) ON DELETE CASCADE,
      service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
      session_time TIME NOT NULL,
      room VARCHAR(100) NOT NULL
    );
  `);
}

// ---------- универсальный CRUD-роутер ----------
// Имена таблиц/колонок берутся только из констант этого файла,
// поэтому их подстановка в SQL безопасна. Значения идут через $1, $2, ...

function createCrudRouter({ table, fields, listSql, getSql }) {
  const router = express.Router();
  const names = fields.map((field) => field.name);
  const columns = names.join(", ");
  const placeholders = names.map((_, index) => `$${index + 1}`).join(", ");
  const assignments = names
    .map((name, index) => `${name} = $${index + 1}`)
    .join(", ");

  router.get(
    "/",
    asyncHandler(async (_req, res) => {
      const { rows } = await pool.query(listSql ?? `SELECT * FROM ${table} ORDER BY id`);
      res.json(rows);
    })
  );

  router.get(
    "/:id",
    asyncHandler(async (req, res) => {
      const id = parseId(req);
      const { rows } = await pool.query(
        getSql ?? `SELECT * FROM ${table} WHERE id = $1`,
        [id]
      );
      if (!rows[0]) throw new HttpError(404, `Запись с id=${id} не найдена`);
      res.json(rows[0]);
    })
  );

  router.post(
    "/",
    asyncHandler(async (req, res) => {
      const values = valuesFromBody(req.body, fields);
      const { rows } = await pool.query(
        `INSERT INTO ${table} (${columns}) VALUES (${placeholders}) RETURNING *`,
        values
      );
      res.status(201).json(rows[0]);
    })
  );

  router.put(
    "/:id",
    asyncHandler(async (req, res) => {
      const id = parseId(req);
      const values = valuesFromBody(req.body, fields);
      const { rows } = await pool.query(
        `UPDATE ${table} SET ${assignments} WHERE id = $${fields.length + 1} RETURNING *`,
        [...values, id]
      );
      if (!rows[0]) throw new HttpError(404, `Запись с id=${id} не найдена`);
      res.json(rows[0]);
    })
  );

  router.delete(
    "/:id",
    asyncHandler(async (req, res) => {
      const id = parseId(req);
      const { rows } = await pool.query(
        `DELETE FROM ${table} WHERE id = $1 RETURNING *`,
        [id]
      );
      if (!rows[0]) throw new HttpError(404, `Запись с id=${id} не найдена`);
      res.json(rows[0]);
    })
  );

  return router;
}

// ---------- объект 1: игры ----------

const gamesRouter = createCrudRouter({ table: "games", fields: GAME_FIELDS });

// ---------- объект 2: услуги ----------

const servicesRouter = createCrudRouter({ table: "services", fields: SERVICE_FIELDS });

// ---------- бонус: связка игр и услуг ----------

const GAME_SERVICE_SELECT = `
  SELECT gs.id,
         gs.game_id,
         g.title AS game_title,
         gs.service_id,
         s.title AS service_title,
         gs.session_time,
         gs.room
  FROM game_services AS gs
  JOIN games AS g ON g.id = gs.game_id
  JOIN services AS s ON s.id = gs.service_id
`;

const gameServicesRouter = createCrudRouter({
  table: "game_services",
  fields: GAME_SERVICE_FIELDS,
  listSql: `${GAME_SERVICE_SELECT} ORDER BY gs.id`,
  getSql: `${GAME_SERVICE_SELECT} WHERE gs.id = $1`,
});

// ---------- подключение маршрутов ----------

// Одинаковые адреса доступны и с префиксом /api, и без него:
// vite проксирует /api, а напрямую можно обращаться и к /games и т.д.
for (const path of ["/games", "/api/games"]) app.use(path, gamesRouter);
for (const path of ["/services", "/api/services"]) app.use(path, servicesRouter);
for (const path of ["/game-services", "/api/game-services"]) {
  app.use(path, gameServicesRouter);
}

app.get("/", (_req, res) => {
  res.json({
    message: "API GameZone работает",
    routes: ["/games", "/services", "/game-services"],
  });
});

app.use((_req, res) => {
  res.status(404).json({ error: "Маршрут не найден" });
});

app.use((err, _req, res, _next) => {
  let status = err.status || 500;

  // типичные ошибки PostgreSQL: нарушение UNIQUE / FK / CHECK / неверный тип
  if (!err.status && ["23505", "23503", "23514", "22P02", "22007"].includes(err.code)) {
    status = err.code === "23505" ? 409 : 400;
  }
  if (status === 500) console.error(err);

  res.status(status).json({
    error: status === 500 ? "Внутренняя ошибка сервера" : err.message,
  });
});

// ---------- запуск ----------

initDb()
  .then(() => {
    app.listen(PORT, () => {
      console.log(`Сервер GameZone запущен на http://localhost:${PORT}`);
    });
  })
  .catch((err) => {
    console.error("Ошибка инициализации базы данных:", err);
    process.exit(1);
  });
