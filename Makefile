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

DOCKER_ENV := ARCH=$(ARCH) TAG=$(CHANNEL)
DOCKER_COMPOSE := $(DOCKER_ENV) docker compose -f $(COMPOSE_FILE)

.PHONY: all
all:up

.PHONY: build
build:
	$(DOCKER_COMPOSE) pull
	$(DOCKER_COMPOSE) build

.PHONY: up
up:
	$(DOCKER_COMPOSE) up -d

.PHONY: down
down:
	$(DOCKER_COMPOSE) down

.PHONY: clean
clean:
	$(DOCKER_COMPOSE) down --rmi all --volumes