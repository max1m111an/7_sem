// ЛАБОРАТОРНАЯ РАБОТА №1
// ЗАДАНИЕ №6: Замыкания (closures)
// Вариант 04
//
// Выполняйте задания строго по порядку. Запуск файла: node var04.js
// (или кнопкой запуска расширения Code Runner в VS Code)

// --- Лёгкий уровень ---

// TODO 1: Счётчик через замыкание
// Напишите функцию createCounter(), которая объявляет внутри
// себя let count = 0 и возвращает функцию БЕЗ аргументов, которая при
// каждом вызове увеличивает count на 1 и возвращает новое значение.
// Создайте переменную counter = createCounter() и вызовите её 3 раза,
// выводя результат каждого вызова.

function createCounter(){
  let count = 0
  return function() {
    count++
    return count
  }
}
let counter = createCounter()
console.log(counter())
console.log(counter())
console.log(counter())

// --- Средний уровень ---

// TODO 2: Независимые счётчики
// Создайте ДВА независимых счётчика через createCounter():
// counterA и counterB. Вызовите counterA() два раза, затем counterB()
// один раз. В выводах результата в отчёте объясните, почему их значения не совпадают.

let counterA = createCounter()
let counterB = createCounter()

counterA()
counterA()
counterB()
counterB()
counterB()
console.log(counterA())
console.log(counterB())

// TODO 3: Функция-приветствие (makeGreeter)
// Напишите функцию makeGreeter(name), которая возвращает функцию
// без аргументов. Эта внутренняя функция при вызове должна выводить
// в консоль "Привет, <name>!" (используйте шаблонную строку с обратными
// кавычками). Проверьте работу на имени "Dmitry".

function makeGreater(name){
  return function() {
    console.log("Привет, " + name + "!")
  }
}
makeGreater("Dmitry")()

// --- Сложный уровень ---

// TODO 4: Приватное состояние (банковский счёт)
// Напишите функцию createBankAccount(initialBalance), которая:
// - хранит внутри себя переменную balance = initialBalance
// - возвращает ОБЪЕКТ с двумя методами:
//   - deposit(amount) — увеличивает balance на amount
//   - getBalance() — возвращает текущий balance
// Проверьте: создайте account = createBankAccount(900),
// вызовите account.deposit(140) и выведите account.getBalance().

function createBankAccount(initialBalance){
  let balance = initialBalance
  return {
    deposit: function(amount) { balance += amount },
    getBalance: function() { console.log(balance) }
  }
}
let account = createBankAccount(900)
account.deposit(140)
account.getBalance()

// --- Задача уровня собеседования ---

// TODO 5: Замыкание и приватные переменные — предсказание
// Что выведет следующий код? Запишите прогноз в выводах результата в отчёте
// ДО запуска (для ОБЕИХ строк), затем раскомментируйте и проверьте.

function outer() {
  let secret = "42";
  function inner() {
    console.log(secret);
  }
  return inner;
}
const revealSecret = outer();
//console.log(secret); // err
revealSecret();
