# 
# Test and build ServiceLogger Project
# 
# This Makefile is primarily instructional; you can simply enter the Nix environment for
# holochain-rust development (supplied by holo=nixpkgs; see pkgs.nix) via `nix-shell` and run `hc
# test` directly, or build a target directly (see default.nix), eg. `nix-build -A servicelogger`.
#
SHELL		= bash
DNANAME		= servicelogger
DNA		= dist/$(DNANAME).dna.json

# External targets; Uses a nix-shell environment to obtain Holochain runtime, run tests
.PHONY: all
all: nix-test

# nix-test, nix-install, ...
# 
# Provides a nix-shell environment, and runs the desired Makefile target.  It is recommended that
# you add `substituters = ...` and `trusted-public-keys = ...` to your nix.conf (see README.md), to
# take advantage of cached Nix and Holo build assets.
nix-%:
	nix-shell --pure --run "make $*"

# Internal targets; require a Nix environment in order to be deterministic.
# - Uses the version of `hc`, `holochain` on the system PATH.
# - Normally called from within a Nix environment, eg. run `nix-shell`
.PHONY:		rebuild build
rebuild: 	clean build

install: 	build

build:		$(DNA)

# Build the DNA; Specifying a custom --output requires the path to exist
# However, if the name of the directory within which `hc` is run matches the
# DNA's name, then this name is used by default, and the output directory is
# created automatically.
$(DNA):
	hc package

.PHONY: test test-unit test-e2e test-stress test-sim2h test-node
test: 		test-unit test-e2e

test-unit:
	RUST_BACKTRACE=1 cargo test \
	    -- --nocapture

# Ensure all DNAs are available.  If a nix-shell environment is used, then `shell.nix` may
# define additional DNAs required; paths to these should be available in the $buildInputs
# environment variable, eg.:
# 
#    /nix/store/r5n15mv3zkh158wz4q06dprlw6six1hn-holofuel
# 
# Otherwise, copy/link to the desired .dna.json files into dist/.
test-dnas:	$(DNA) \
		dist/holofuel.dna.json \
		dist/holo-hosting-app.dna.json

dist/%.dna.json:
	@for p in $$buildInputs; do \
	    if [[ "$${p#*-}" == "$*" ]]; then \
		echo "Linking $${p} to $@"; \
		ln -fs $${p}/$*.dna.json $@; \
	    fi \
	done \

# End-to-end test of DNA.  Runs a sim2h_server on localhost:9000; the default expected by `hc test`
test-e2e:	test-dnas test-sim2h test-node test-dnas
	@echo "Starting Scenario tests..."; \
	    RUST_BACKTRACE=1 hc test \
	        | node test/node_modules/faucet/bin/cmd.js

test-node:
	@echo "Setting up Scenario/Stress test Javascript..."; \
	    cd test && [ -d node_modules ] || npm install

test-sim2h:
	@echo "Starting sim2h_server on localhost:9000 (may already be running)..."; \
	    sim2h_server -p 9000 >sim2h_server.log 2>&1 &

# Generic targets; does not require a Nix environment
.PHONY: clean
clean:
	rm -rf \
	    dist \
	    test/node_modules \
	    .cargo \
	    target



# Create a conductor configuration using an agent # (1, 2, ...) and a keystore-agent-#.key link (which must link to 
# a filename ending in the public key, eg.
#     $ ls -l keystore-agent-3.key
#     lrwxrwxr-x  1 perry  staff  125 18 Apr 13:58 keystore-agent-3.key -> /Users/p.../keys/HcSCiGvvwq63Yyqa46H3C8OgioEkyt9ye3UF6J9PmINHuyrpUUh7Oisbfrr49da

#$(MAKE) keystore-%.key conductor-0.out conductor.toml.master

.PRECIOUS: conductor-%.toml
conductor-%.toml: keystore-%.key
	echo "Creating Holochain conductor config for Agent $*...";			\
	AGENT=$*;									\
	PUBKEY=$$( ls -l $< ); PUBKEY=$${PUBKEY##*/};					\
	KEYFILE=$<;									\
	WSPORT=$$(( 3000 + $* ));							\
	SLUIPORT=$$(( 8800 + $* ));							\
	HFUIPORT=$$(( 8300 + $* ));							\
	HHUIPORT=$$(( 8400 + $* ));							\
	S2HURI=wss://127.0.0.1:9000;							\
	sed -e "s/PUBKEY/$$PUBKEY/g"							\
	    -e "s/KEYFILE/$$KEYFILE/g"							\
	    -e "s/WSPORT/$$WSPORT/g"							\
	    -e "s/SLUIPORT/$$SLUIPORT/g"						\
	    -e "s/HFUIPORT/$$HFUIPORT/g"						\
	    -e "s/HHUIPORT/$$HHUIPORT/g"						\
	    -e "s|IPCURI|$$IPCURI|g"							\
	    -e "s|BSNODES|$$BSNODES|g"							\
	    -e "s|AGENT|$$AGENT|g"							\
	    < conductor.toml.master									\
	    > $@;									\
	echo "	Wrote new $@ (from conductor.toml.master and $<)"

# If the target doesn't exist, create a new key. The last segment of the link.  Don't delete it, if
# we do had to create one as an intermediate target.
.PRECIOUS: keystore-%.key
keystore-%.key:
	@echo -n "Creating Holochain key for Agent $*...";				\
	eval $$( hc keygen --nullpass --quiet						\
	  | python -c "import sys;						\
	      print('\n'.join('%s=%s' % ( k, v.strip() )			\
		for (k, v) in zip(['KEY','KEYFILE'], sys.stdin.readlines())))"	\
	);										\
	echo " $@ -> $$KEYFILE";							\
	ln -fs $$KEYFILE $@
