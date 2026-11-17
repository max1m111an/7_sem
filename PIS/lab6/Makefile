.PHONY: client server task client-task client-itog

# Запуск клиентской части по умолчанию
client:
	npm run dev

# Запуск Express-сервера
server:
	npx nodemon server.js

# Запуск серверной части задания
task:
	npx nodemon task.js

# Клиентская часть для варианта task
client-task:
	npx vite --mode task

# Клиентская часть для варианта itog
client-itog:
	npx vite --mode itog