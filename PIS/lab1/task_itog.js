// ЛАБОРАТОРНАЯ РАБОТА №1
// ИТОГОВОЕ ЗАДАНИЕ (по всем темам: переменные, типы, массивы, объекты,
// циклы, область видимости, замыкания)
// Вариант 04
//
// Задание выполняется САМОСТОЯТЕЛЬНО, без подсказок и без готового
// кода — только текстовое описание требований. Комментарии-подсказки
// и заготовки кода в файле намеренно отсутствуют.
//
// Тема варианта: учёт объектов категории «велосипед» в системе
// (например, склад, каталог, список объектов).

// Требования к итоговому заданию:
//
// 1. Создайте массив из не менее 4 объектов, где каждый объект
//    относится к категории «велосипед» и содержит минимум 3 поля
//    (например: название, категория, числовое значение — цена, год,
//    количество и т.п.; конкретные значения придумайте сами).
//
// 2. Через цикл (for или for...of) переберите массив и выведите
//    в консоль краткую информацию по каждому объекту в формате
//    "Название — значение" (используйте шаблонную строку).
//
// 3. Через цикл посчитайте сумму (или среднее) числового поля по всем
//    объектам массива и выведите результат.
//
// 4. Напишите функцию, которая принимает массив объектов и пороговое
//    число и возвращает НОВЫЙ массив только с теми объектами, у которых
//    числовое поле больше порога (используйте цикл и обычный if,
//    без встроенных методов массива вроде filter).
//
// 5. Напишите функцию-фабрику (по аналогии с createCounter или
//    createBankAccount), которая через ЗАМЫКАНИЕ хранит счётчик
//    операций (например, сколько раз вызывали функцию добавления
//    объекта) и предоставляет метод для получения текущего значения
//    счётчика. Продемонстрируйте работу на 2-3 вызовах.
//
// 6. Продемонстрируйте понимание области видимости: объявите одну
//    переменную внутри блока { } с помощью let и покажите (в выводах результата
//    в отчёте), что она недоступна снаружи этого блока.
//
// Критерии оценки: корректная работа кода, чистота и читаемость,
// использование правильных конструкций (let/const, строгое сравнение
// ===, шаблонные строки), отсутствие незадекларированных
// (случайных) глобальных переменных.

// Ваш код ниже:

// #1
const bicycles = [
  { name: "Stels Navigator", category: "горный", price: 14500 },
  { name: "Forward Apache", category: "городской", price: 15200 },
  { name: "Merida Big Nine", category: "горный", price: 28900 },
  { name: "Cube Aim Race", category: "шоссейный", price: 32500 },
  { name: "Aist Velo", category: "детский", price: 8900 }
]

// #2
for (const bike of bicycles) {
  for (const field in bike) {
    console.log(`${field} — ${bike[field]}`)
  }
}

// #3
let total = 0
for (const bike of bicycles) {
  total += bike.price
}
console.log("Средняя цена = " + total / bicycles.length)

// #4
function filterByPrice(arr, threshold) {
  let newArr = []
  for (const bike of arr) {
    if (bike.price > threshold) {
      newArr.push(bike)
    }
  }
  return newArr
}

let expensive = filterByPrice(bicycles, 15000)
console.log(expensive)

// #5
function createCatalog() {
  let callsCount = 0
  let items = []
  return {
    getCallsCount: function (isCall = false) {
      if (isCall) callsCount++
      return callsCount
    },
    getItems: function () {
      callsCount++
      return items
    },
    addBicycle: function (name, category, price) {
      callsCount++
      items.push({ name, category, price })
    }
  }
}

let catalog = createCatalog()
catalog.addBicycle("Trek Marlin", "горный", 26800)
console.log(catalog.getItems())
console.log(catalog.getCallsCount())
console.log(catalog.getCallsCount(true))

// #6
function scopeCheck() {
  const specificScope = 144
  console.log("End of scope")
}
scopeCheck()
// console.log(specificScope) // err
