let
  pins = import ../npins;
  pkgs = import pins.nixpkgs {
    overlays = [
      (final: prev: {
        nai = final.callPackage ./package.nix { };
      })
    ];
  };
in
{
  shell = pkgs.callPackage ./shell.nix { };
  doc = pkgs.callPackage ./doc.nix { };
  bins = pkgs.nai;
}
