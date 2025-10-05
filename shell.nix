let
  pins = import ./npins;
  pkgs = import pins.nixpkgs {};
in pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rust-analyzer
    clippy
    rustfmt
  ];
}
