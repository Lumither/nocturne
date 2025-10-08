.PHONY: rebuild
rebuild:
	cargo clean
	cargo build

.PHONY: rebuild-release
rebuild-release:
	cargo clean
	cargo build -r

.PHONY: run
run:
	sudo ./target/debug/daemon
