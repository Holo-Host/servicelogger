{ pkgs ? import ./pkgs.nix {}, shell ? false }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  servicelogger = buildDNA {
    inherit shell;

    name = "servicelogger";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
