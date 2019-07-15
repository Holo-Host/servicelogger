.PHONY: all sl-test sl-test-unit sl-test-e2e install clean

# External targets; Uses a nix-shell environment to obtain Holochain runtime, run tests

all: sl-test

sl-test:
	nix-shell --run sl-test

sl-test-%:
	nix-shell --run sl-test-$*

sl-install:
	nix-shell --run sl-install

# Internal targets; require a Nix environment in order to be deterministic.  Uses the version
# of `hc` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell` from within hApp dir
# - If you establish a Nix environment (eg. run `nix-shell` from within holochain-rust, to
#   gain access to a certain development branch), then you can run these targets to build
#   and test the 'Zome under that version of holochain-rust
.PHONY: rebuild build test-unit test-e2e
rebuild: clean build
build:
	rm -rf dist
	hc package --strip-meta

test-unit:
	RUST_BACKTRACE=1 cargo test \
	    --manifest-path zomes/service/code/Cargo.toml \
	    -- --nocapture
test-e2e:
	( cd test && npm install ) \
	&& RUST_BACKTRACE=1 hc test \
	    | test/node_modules/faucet/bin/cmd.js

# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf dist test/node_modules .cargo # cleans up artifacts


