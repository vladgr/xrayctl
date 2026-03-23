-include .env

build:
	clear
	cargo build --release

upload:
	clear
	scp -r $(shell pwd)/target/release/xrayctl ubuntu@$(SERVER_IP):/home/ubuntu/xrayctl
