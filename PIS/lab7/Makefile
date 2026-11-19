.PHONY: client server task all-task all-itog client-task client-itog

client:
	npm run dev

server:
	npx nodemon server.js

task:
	npx nodemon task.js

all-task:
	@$(MAKE) -j2 client-task task

all-itog:
	@$(MAKE) -j2 server client-itog

# Клиентская часть для варианта task
client-task:
	npx vite --mode task

# Клиентская часть для варианта itog
client-itog:
	npx vite --mode itog