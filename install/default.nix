{ pkgs }:
let
  name = "sl-install";

  script = pkgs.writeShellScriptBin name
  ''
  make install
  '';
in
{
 buildInputs = [ script ];
}
