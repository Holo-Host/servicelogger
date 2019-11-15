.PHONY: all

# External targets; Uses a nix-shell environment to obtain Holochain runtime, run tests

.PHONY: all
all: nix-test

# nix-install, nix-test, nix-test-unit, ...
nix-%:
	nix-shell --pure --run "make $*"

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
# However, if the name of the directory within which `hc` is run matches the
# DNA's name, then this name is used by default, and the output directory is
# created automatically.
$(DNA):
	hc package

test: 		test-unit test-e2e

test-unit:
	RUST_BACKTRACE=1 cargo test \
	    --manifest-path zomes/service/code/Cargo.toml \
	    -- --nocapture

test-e2e-sim1h: export AWS_ACCESS_KEY_ID     ?= HoloCentral
test-e2e-sim1h: export AWS_SECRET_ACCESS_KEY ?= ... 
test-e2e-sim1h:	$(DNA)
	export |grep AWS
	@echo "Setting up Scenario test Javascript..."; \
	    ( cd test && npm install );
	@echo "Starting dynamodb-memory..."; \
	    dynamodb-memory &
	@echo "Starting HoloFuel Scenario tests..."; \
	    RUST_BACKTRACE=1 APP_SPEC_NETWORK_TYPE=sim1h hc test \
	    | test/node_modules/faucet/bin/cmd.js

# End-to-end test of DNA.  Runs a sim2h_server on localhost:9000; the default expected by `hc test`
test-e2e:	$(DNA)
	@echo "Setting up Scenario test Javascript..."; \
	    ( cd test && npm install )
	@echo "Starting sim2h_server on localhost:9000 (may already be running)..."; \
	    sim2h_server -p 9000 &
	@echo "Starting HoloFuel Scenario tests..."; \
	    RUST_BACKTRACE=1 hc test 2>test.out~ \
	        | test/node_modules/faucet/bin/cmd.js
	@echo "*** If tests failed, see debug output in test.out~ ***"


# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    test/node_modules \
	    .cargo \
	    target \
	    zomes/service/code/target


