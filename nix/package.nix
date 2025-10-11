{
  lib,
  rustPlatform,
  pkgsStatic,
}:

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
      ];
    };

  passthru.static = pkgsStatic.nai;
}
