up:
	docker-compose up -d
	sleep 5
	docker exec -ti miner-rust_dogecoind-beta_1 dogecoin-cli addnode dogecoind-alpha:18444 onetry
	docker exec -ti miner-rust_dogecoind-beta_1 dogecoin-cli generate 50
	docker exec -ti miner-rust_litecoind-beta_1 litecoin-cli addnode litecoind-alpha:19444 onetry
	docker exec -ti miner-rust_litecoind-beta_1 litecoin-cli generatetoaddress 50 QSCiV7ezoTYDfhtqURYUH9g4wY2Pzvmmm9

down:
	docker-compose down