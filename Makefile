ARCH := $(shell uname -m)
ifeq ($(ARCH), arm64)
    ARCH := aarch64
endif

all: release

release:
	docker compose -f prod.compose.yaml build --build-arg ARCH=$(ARCH)

dev:
	docker-compose -f dev.compose.yaml up --build

clean:
	docker-compose -f prod.compose.yaml -f dev.compose.yaml rm -fsv

