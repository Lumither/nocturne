ARCH := $(shell uname -m)
ifeq ($(ARCH), arm64)
    ARCH := aarch64
endif

ifneq (,$(wildcard ./.env))
    include ./.env
    export
endif
ifneq (,$(wildcard ./.env.local))
    include ./.env.local
    export
endif

USE_CR ?= false
CHANNEL ?= release
ifeq ($(USE_CR), true)
    COMPOSE_FILE := prod.cr.compose.yaml
else
    COMPOSE_FILE := prod.compose.yaml
endif


.PHONY: all
all:up

.PHONY: build
build:
	ARCH=$(ARCH) TAG=$(CHANNEL) docker compose -f $(COMPOSE_FILE) pull
	ARCH=$(ARCH) TAG=$(CHANNEL) docker compose -f $(COMPOSE_FILE) build

.PHONY: up
up:
	ARCH=$(ARCH) docker compose -f $(COMPOSE_FILE) up -d

.PHONY: down
down:
	ARCH=$(ARCH) docker compose -f $(COMPOSE_FILE) down

.PHONY: clean
clean:
	ARCH=$(ARCH) docker compose -f $(COMPOSE_FILE) down --rmi all --volumes