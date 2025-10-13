ARCH := $(shell uname -m)
ifeq ($(ARCH), arm64)
    ARCH := aarch64
endif

.PHONY: all
all:up

.PHONY: build
build:
	ARCH=$(ARCH) docker compose -f prod.compose.yaml build

.PHONY: up
up:
	ARCH=$(ARCH) docker compose -f prod.compose.yaml up -d

.PHONY: down
down:
	ARCH=$(ARCH) docker compose -f prod.compose.yaml down

.PHONY: rm
rm:
	ARCH=$(ARCH) docker compose -f prod.compose.yaml rm
