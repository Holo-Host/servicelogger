{ pkgs ? import ./pkgs.nix {}, shell ? false }:

with pkgs;

mkShell {
  inputsFrom = lib.attrValues  (import ./. {
    inherit pkgs;
    shell = true;
  });

  buildInputs = [
    # additional packages go here.  Paths to these will be available in the shell $buildInputs
    # environment variable.
    dnaPackages.holofuel
    dnaPackages.holo-hosting-app
  ];
}
