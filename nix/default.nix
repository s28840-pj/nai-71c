let
  pins = import ../npins;
  pkgs = import pins.nixpkgs { };
in
{
  shell = pkgs.callPackage ./shell.nix { };
  doc = pkgs.callPackage ./doc.nix { };
}
