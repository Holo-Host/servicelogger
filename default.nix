{ pkgs ? import ./pkgs.nix {} }: with pkgs;

{
  servicelogger = buildDNA {
    name = "servicelogger";
    src = gitignoreSource ./.;
  };
}
