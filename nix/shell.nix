{
  mkShell,
  rustc,
  cargo,
  rust-analyzer,
  clippy,
  rustfmt,
}:
mkShell {
  packages = [
    rustc
    cargo
    rust-analyzer
    clippy
    rustfmt
  ];

  shellHook = ''
    export REKOMENDACJE_DATA_DIR="$(readlink -f 03-rekomendacje)/clean"
  '';
}
