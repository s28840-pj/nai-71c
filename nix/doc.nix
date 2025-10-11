{ lib, rustPlatform }:

rustPlatform.buildRustPackage {
  pname = "nai-doc";
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

  dontCargoCheck = true;

  buildPhase = ''
    cargo doc --no-deps --workspace --frozen
  '';

  installPhase = ''
    cp -r target/doc "$out"
    echo "<meta http-equiv=\"refresh\" content=\"0; url=warcaby\">" > "$out/index.html"
  '';
}
