{ pkgs }:
let
  name = "sl-test";

  script = pkgs.writeShellScriptBin name
  ''
  sl-test-unit
  sl-test-e2e
  '';
in
{
 buildInputs = [
  script
 ]
 ++ (pkgs.callPackage ./unit { }).buildInputs
 ++ (pkgs.callPackage ./e2e { }).buildInputs
 ;
}
