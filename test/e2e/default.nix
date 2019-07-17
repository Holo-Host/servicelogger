{ pkgs }:
let
  name = "sl-test-e2e";

  script = pkgs.writeShellScriptBin name
  ''
  make test-e2e
  '';
in
{
 buildInputs = [ script ];
}
