{ pkgs }:
let
  name = "hf-test-unit";

  script = pkgs.writeShellScriptBin name
  ''
  RUST_BACKTRACE=1 cargo test \
      --manifest-path zomes/service/code/Cargo.toml \
      -- --nocapture
  '';
in
{
 buildInputs = [ script ];
}