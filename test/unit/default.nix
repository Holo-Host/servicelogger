{ pkgs }:
let
  name = "sl-test-unit";

  script = pkgs.writeShellScriptBin name
  ''
  make test-unit
  '';
in
{
 buildInputs = [ script ];
}
