// task.js — сервер варианта 04: спортивный клуб «Атлет»
//
// Два независимых объекта:
//   trains   — тренировки (name, day, duration_min, level, max_participants, hall);
//   trainers — тренеры (name, specialty, experience_years, phone, rating, photo_url).
// Дополнительно реализована связывающая таблица trains_trainers (расписание)
// с двумя внешними ключами и JOIN-запросами.
//
// Все обращения к PostgreSQL идут только через параметры $1, $2, ... —
// склейка строк в SQL не используется.

import express from "express";
import cors from "cors";
import pg from "pg";

const { Pool } = pg;

const PORT = 3000;

const pool = new Pool({
  host: "localhost",
  port: 5432,
  database: "postgres",
  user: "postgres",
  password: "s31nhl3r",
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

// Express 5 сам ловит отклонённые промисы, но обёртка делает код явным
// и сохраняет совместимость с прежними версиями Express.
const asyncHandler = (handler) => (req, res, next) =>
  Promise.resolve(handler(req, res, next)).catch(next);

const TRAIN_FIELDS = [
  "name",
  "day",
  "duration_min",
  "level",
  "max_participants",
  "hall",
];
const TRAINER_FIELDS = [
  "name",
  "specialty",
  "experience_years",
  "phone",
  "rating",
  "photo_url",
];
const SCHEDULE_FIELDS = ["workout_id", "trainer_id", "time", "room"];

const INT_FIELDS = new Set([
  "duration_min",
  "max_participants",
  "experience_years",
  "rating",
  "workout_id",
  "trainer_id",
]);

// Возвращает значения полей в том порядке, в каком они пойдут в $1, $2, ...
function valuesFromBody(body, fields) {
  // body: db obj, fields: string[]
  return fields.map((field) => {
    const raw = body?.[field];
    const value = INT_FIELDS.has(field) ? Number(raw) : String(raw ?? "").trim();

    if (INT_FIELDS.has(field) && !Number.isInteger(value)) {
      throw new HttpError(400, `Поле "${field}" должно быть целым числом`);
    }
    if (!INT_FIELDS.has(field) && !value) {
      throw new HttpError(400, `Поле "${field}" обязательно`);
    }

    return value;
  });
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
    CREATE TABLE IF NOT EXISTS trains (
     id SERIAL PRIMARY KEY,
     name varchar(255) NOT NULL,
     day varchar(255) NOT NULL,
     duration_min integer NOT NULL,
     level varchar(255) NOT NULL,
     max_participants integer NOT NULL,
     hall varchar(255) NOT NULL
    );
  `);

  await pool.query(`
    CREATE TABLE IF NOT EXISTS trainers (
      id SERIAL PRIMARY KEY,
      name VARCHAR(50) NOT NULL,
      specialty VARCHAR(50) NOT NULL,
      experience_years INTEGER NOT NULL,
      phone VARCHAR(20) NOT NULL,
      rating INTEGER NOT NULL,
      photo_url VARCHAR(255) NOT NULL,
      CONSTRAINT "phone-photo" UNIQUE (phone, photo_url)
    );
  `);

  await pool.query(`
    CREATE TABLE IF NOT EXISTS trains_trainers (
      id SERIAL PRIMARY KEY,
      workout_id INTEGER NOT NULL,
      trainer_id INTEGER NOT NULL,
      "time" time NOT NULL,
      room VARCHAR(255) NOT NULL,
      CONSTRAINT workout_id FOREIGN KEY (workout_id) REFERENCES trains(id),
      CONSTRAINT trainer_id FOREIGN KEY (trainer_id) REFERENCES trainers(id)
    );

  `);
}

// ---------- ОБЪЕКТ 1: тренировки (/trains) ----------

const trainsRouter = express.Router();

trainsRouter.get(
  "/",
  asyncHandler(async (_req, res) => {
    const { rows } = await pool.query("SELECT * FROM trains ORDER BY id");
    res.json(rows);
  })
);

trainsRouter.post(
  "/",
  asyncHandler(async (req, res) => {
    const values = valuesFromBody(req.body, TRAIN_FIELDS);
    const { rows } = await pool.query(
      `INSERT INTO trains (name, day, duration_min, level, max_participants, hall)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      values
    );
    res.status(201).json(rows[0]);
  })
);

trainsRouter.put(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const values = valuesFromBody(req.body, TRAIN_FIELDS);
    const { rows } = await pool.query(
      `UPDATE trains
       SET name = $1, day = $2, duration_min = $3,
           level = $4, max_participants = $5, hall = $6
       WHERE id = $7
       RETURNING *`,
      [...values, id]
    );
    if (!rows[0]) throw new HttpError(404, `Тренировка с id=${id} не найдена`);
    res.json(rows[0]);
  })
);

trainsRouter.delete(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const { rows } = await pool.query(
      "DELETE FROM trains WHERE id = $1 RETURNING *",
      [id]
    );
    if (!rows[0]) throw new HttpError(404, `Тренировка с id=${id} не найдена`);
    res.json(rows[0]);
  })
);

// ---------- ОБЪЕКТ 2: тренеры (/trainers) ----------

const trainersRouter = express.Router();

trainersRouter.get(
  "/",
  asyncHandler(async (_req, res) => {
    const { rows } = await pool.query("SELECT * FROM trainers ORDER BY id");
    res.json(rows);
  })
);

trainersRouter.post(
  "/",
  asyncHandler(async (req, res) => {
    const values = valuesFromBody(req.body, TRAINER_FIELDS);
    const { rows } = await pool.query(
      `INSERT INTO trainers
         (name, specialty, experience_years, phone, rating, photo_url)
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING *`,
      values
    );
    res.status(201).json(rows[0]);
  })
);

trainersRouter.put(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const values = valuesFromBody(req.body, TRAINER_FIELDS);
    const { rows } = await pool.query(
      `UPDATE trainers
       SET name = $1, specialty = $2, experience_years = $3,
           phone = $4, rating = $5, photo_url = $6
       WHERE id = $7
       RETURNING *`,
      [...values, id]
    );
    if (!rows[0]) throw new HttpError(404, `Тренер с id=${id} не найден`);
    res.json(rows[0]);
  })
);

trainersRouter.delete(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const { rows } = await pool.query(
      "DELETE FROM trainers WHERE id = $1 RETURNING *",
      [id]
    );
    if (!rows[0]) throw new HttpError(404, `Тренер с id=${id} не найден`);
    res.json(rows[0]);
  })
);

// ---------- БОНУС: расписание (/schedule) ----------

// В списке вместо "голых" внешних ключей подставляем читаемые названия.
const SCHEDULE_SELECT = `
  SELECT tt.id,
         tt.workout_id,
         t.name AS workout_name,
         tt.trainer_id,
         tr.name AS trainer_name,
         tt."time" AS time,
         tt.room
  FROM trains_trainers AS tt
  JOIN trains AS t ON t.id = tt.workout_id
  JOIN trainers AS tr ON tr.id = tt.trainer_id
`;

const scheduleRouter = express.Router();

scheduleRouter.get(
  "/",
  asyncHandler(async (_req, res) => {
    const { rows } = await pool.query(`${SCHEDULE_SELECT} ORDER BY tt.id`);
    res.json(rows);
  })
);

scheduleRouter.get(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const { rows } = await pool.query(`${SCHEDULE_SELECT} WHERE tt.id = $1`, [id]);
    if (!rows[0]) throw new HttpError(404, `Запись расписания с id=${id} не найдена`);
    res.json(rows[0]);
  })
);

scheduleRouter.post(
  "/",
  asyncHandler(async (req, res) => {
    const [workout_id, trainer_id, time, room] = valuesFromBody(
      req.body,
      SCHEDULE_FIELDS
    );
    const { rows } = await pool.query(
      `INSERT INTO trains_trainers (workout_id, trainer_id, "time", room)
       VALUES ($1, $2, $3, $4)
       RETURNING *`,
      [workout_id, trainer_id, time, room]
    );
    res.status(201).json(rows[0]);
  })
);

scheduleRouter.put(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const [workout_id, trainer_id, time, room] = valuesFromBody(
      req.body,
      SCHEDULE_FIELDS
    );
    const { rows } = await pool.query(
      `UPDATE trains_trainers
       SET workout_id = $1, trainer_id = $2, "time" = $3, room = $4
       WHERE id = $5
       RETURNING *`,
      [workout_id, trainer_id, time, room, id]
    );
    if (!rows[0]) throw new HttpError(404, `Запись расписания с id=${id} не найдена`);
    res.json(rows[0]);
  })
);

scheduleRouter.delete(
  "/:id",
  asyncHandler(async (req, res) => {
    const id = parseId(req);
    const { rows } = await pool.query(
      "DELETE FROM trains_trainers WHERE id = $1 RETURNING *",
      [id]
    );
    if (!rows[0]) throw new HttpError(404, `Запись расписания с id=${id} не найдена`);
    res.json(rows[0]);
  })
);

// ---------- подключение маршрутов ----------

// Одинаковые роутеры доступны и без префикса /api, и с ним —
// чтобы фронтенд на React мог использовать любой из этих адресов.
for (const path of ["/trains", "/api/trains"]) app.use(path, trainsRouter);
for (const path of ["/trainers", "/api/trainers"]) app.use(path, trainersRouter);
for (const path of ["/schedule", "/api/schedule"]) app.use(path, scheduleRouter);

app.get("/", (_req, res) => {
  res.json({
    message: 'Сервер спортивного клуба «Атлет» работает',
    routes: ["/trains", "/trainers", "/schedule"],
  });
});

app.use((_req, res) => {
  res.status(404).json({ error: "Маршрут не найден" });
});

app.use((err, _req, res, _next) => {
  let status = err.status || 500;
  // типичные ошибки PostgreSQL: нарушение UNIQUE / FK / неверный тип
  if (!err.status && ["23505", "23503", "22P02", "22007"].includes(err.code)) {
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
      console.log(`Сервер запущен на http://localhost:${PORT}`);
    });
  })
  .catch((err) => {
    console.error("Ошибка инициализации базы данных:", err);
    process.exit(1);
  });
