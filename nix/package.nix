{
  lib,
  rustPlatform,
  pkgsStatic,
  path,
  runCommand,
  stdenv,
  overlays,
}:
let
  x86_64-darwin-pkgs = import path {
    system = "x86_64-darwin";
    inherit overlays;
  };
  linuxStatic = runCommand "nai-static" { } ''
    mkdir -p "$out"
    for f in "${pkgsStatic.nai}"/bin/*; do
      cp "$f" "$out"/"$(basename "$f")"-x86_64-linux
    done
  '';
  darwinStatic = runCommand "nai-static" { } ''
    mkdir -p "$out"
    for f in "${pkgsStatic.nai}"/bin/*; do
      cp "$f" "$out"/"$(basename "$f")"-aarch64-darwin
    done
    for f in "${x86_64-darwin-pkgs.pkgsStatic.nai}"/bin/*; do
      cp "$f" "$out"/"$(basename "$f")"-x86_64-darwin
    done
  '';
in
rustPlatform.buildRustPackage {
  pname = "nai";
  version = "0.0.1";

  cargoLock.lockFile = ../Cargo.lock;
  src =
    with lib.fileset;
    toSource {
      root = ../.;
      fileset = unions [
        ../Cargo.toml
        ../Cargo.lock
        ../01-warcaby/src
        ../01-warcaby/Cargo.toml
        ../01-warcaby/README.md
      ];
    };

  passthru.static = if stdenv.isLinux then linuxStatic else darwinStatic;
}
