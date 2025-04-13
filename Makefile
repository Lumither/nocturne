ARCH := $(shell uname -m)
ifeq ($(ARCH), arm64)
    ARCH := aarch64
endif

.PHONY: up
clean:
	echo "todo"

