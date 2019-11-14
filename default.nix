{ pkgs ? import ./pkgs.nix {} }:

with pkgs;

let
  inherit (darwin.apple_sdk.frameworks) CoreServices Security;
in

{
  servicelogger = buildDNA {
    name = "servicelogger";
    src = gitignoreSource ./.;

    nativeBuildInputs = []
    ++ (callPackage ./dynamodb {}).buildInputs
    ++ lib.optionals stdenv.isDarwin [ CoreServices ];
  };
}
