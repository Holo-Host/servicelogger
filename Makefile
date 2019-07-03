SHELL		= /bin/bash

HOLOCHAIN	= ~/src/holochain-rust
CARGO = cargo -Z config-profile

# Holochain Rust has specific requirements for v8 of NodeJS
export PATH := /usr/local/opt/node@8/bin:$(PATH)

# When a local Holochain Rust is being used, its hc/holochain are built into its .cargo/bin
# when using the nix-shell build targets.
export PATH := $(HOLOCHAIN)/.cargo/bin:$(PATH)

all: test

version:
	@if [[ $$( rustc --version )						\
		!= "rustc 1.33.0-nightly (19f8958f8 2019-01-23)" ]]; then	\
	    rustup override set nightly-2019-01-24;				\
	fi
	@if ! rustc --print target-list 					\
		| grep wasm32-unknown-unknown >/dev/null; then			\
	    rustup target add wasm32-unknown-unknown;				\
	fi

build: version
	#$(CARGO) build           --manifest-path zomes/transactions/code/Cargo.toml --target wasm32-unknown-unknown || true
	#cargo build --release --manifest-path zomes/transactions/code/Cargo.toml --target wasm32-unknown-unknown || true

test-cargo: version
	#RUST_BACKTRACE=1 cargo test --manifest-path zomes/transactions/code/Cargo.toml -- --nocapture || true

install: build
	nix-shell --run hf-install

test: test-cargo # install
	#RUST_STACKTRACE=1 hc test # | test/node_modules/faucet/bin/cmd.js # for pretty output w/ no logging



