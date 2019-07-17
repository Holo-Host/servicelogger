{ pkgs }:
let
  name = "sl-install";

  script = pkgs.writeShellScriptBin name
  ''
  rm -rf dist
  hc package --strip-meta
  '';
in
{
 buildInputs = [ script ];
}
