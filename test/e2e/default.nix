{ pkgs }:
let
  name = "sl-test-e2e";

  script = pkgs.writeShellScriptBin name
  ''
   sl-install && make test-e2e
  '';
in
{
 buildInputs = [ script ];
}
