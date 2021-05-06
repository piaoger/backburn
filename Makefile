
ifeq ($(RELEASE),1)
        PROFILE ?= release
        CARGO_ARGS = --release
else
        PROFILE ?= debug
        CARGO_ARGS =
endif

.PHONY: build
build: $(units)
	cargo build ${CARGO_ARGS}

