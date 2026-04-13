up:
	@echo "   ___                         _              _             ";
	@echo "  / _ \\___  ___ ____ _______  (_)__    __ _  (_)__  ___ ____";
	@echo " / // / _ \\/ _ \`/ -_) __/ _ \\/ / _ \\  /  ' \\/ / _ \\/ -_) __/";
	@echo "/____/\\___/\\_, /\\__/\\__/\\___/_/_//_/ /_/_/_/_/_//_/\\__/_/   ";
	@echo "          /___/                                             ";
	tmux new-session -d -s dev \; \
		send-keys 'docker compose up' Enter \; \
		run-shell 'sleep 10' \; \
		split-window -h \; \
		send-keys 'docker exec dogecoind-beta dogecoin-cli generate 50 && docker exec litecoind-beta litecoin-cli generatetoaddress 50 QSCiV7ezoTYDfhtqURYUH9g4wY2Pzvmmm9' Enter \; \
		attach-session -t dev

kill:
	docker compose down
	tmux kill-session -t dev