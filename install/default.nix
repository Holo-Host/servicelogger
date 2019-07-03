{ pkgs }:
let
  name = "hf-install";

  script = pkgs.writeShellScriptBin name
  ''
  rm -f dist/holofuel.dna.json
  mkdir -p dist
  hc package --output dist/holofuel.dna.json --strip-meta
  '';
in
{
 buildInputs = [ script ];
}