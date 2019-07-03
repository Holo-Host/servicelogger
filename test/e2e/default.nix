{ pkgs }:
let
  name = "hf-test-e2e";

  script = pkgs.writeShellScriptBin name
  ''
   # Build HoloFuel, install test JS dependencies, and run Diorama tests
   hf-install \
   && ( cd test && npm install ) \
   && RUST_BACKTRACE=1 hc test \
       | test/node_modules/faucet/bin/cmd.js
  '';
in
{
 buildInputs = [ script ];
}