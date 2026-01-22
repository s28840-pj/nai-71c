let
  pins = import ../npins;
  pkgs = import pins.nixpkgs {
    overlays = [
      (final: prev: {
        nai = final.callPackage ./package.nix { };
        ad-watcher = final.callPackage ./web.nix { };
        sdl12-compat = final.callPackage ./sdl12.nix { };
        ale-roms = final.callPackage ./roms.nix { };
      })
    ];
  };
in
{
  shell = pkgs.callPackage ./shell.nix { };
  doc = pkgs.callPackage ./doc.nix { };
  bins = pkgs.nai;
  inherit (pkgs) sdl12-compat ale-roms;
}
