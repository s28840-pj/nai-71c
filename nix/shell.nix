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
}
