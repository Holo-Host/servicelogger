.PHONY: all

# External targets; Uses a nix-shell environment to obtain Holochain runtime, run tests

all: sl-test

# sl-install, sl-test, sl-test-unit, ...
sl-%:
	nix-shell --run sl-$*

# Internal targets; require a Nix environment in order to be deterministic.  Uses the version
# of `hc` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell` from within hApp dir
# - If you establish a Nix environment (eg. run `nix-shell` from within holochain-rust, to
#   gain access to a certain development branch), then you can run these targets to build
#   and test the 'Zome under that version of holochain-rust
.PHONY:		rebuild build test test-unit test-e2e

rebuild: 	clean build

install: 	build

DNA = dist/servicelogger.dna.json

build:		$(DNA)

# Build the DNA; Specifying a custom --output requires the path to exist
$(DNA):
	mkdir -p $(dir $(@))
	hc package --output $@ --strip-meta
	ln $(DNA) dist/$$( hc hash | sed -ne 's/DNA Hash: \(.*\)/\1/p' ).servicelogger.dna.json

test: 		test-unit test-e2e

test-unit:
	RUST_BACKTRACE=1 cargo test \
	    --manifest-path zomes/service/code/Cargo.toml \
	    -- --nocapture

test-e2e:	$(DNA)
	( cd test && npm install ) \
	&& RUST_BACKTRACE=1 hc test \
	    | test/node_modules/faucet/bin/cmd.js

# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    test/node_modules \
	    .cargo \
	    target \
	    zomes/service/code/target


