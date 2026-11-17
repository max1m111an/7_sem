// server.js
import express from "express";
const app = express();

const PORT = 3000;

app.get("/", function (req, res) {
  res.send("Сервер работает!");
});

app.listen(PORT, function () {
  console.log(`Сервер запущен на http://localhost:${PORT}`);
});