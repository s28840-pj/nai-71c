{
  mkShell,
  rustc,
  cargo,
  rust-analyzer,
  clippy,
  rustfmt, pnpm, nodejs, nodePackages,
}:
mkShell {
  packages = [
    rustc
    cargo
    rust-analyzer
    clippy
    rustfmt
    pnpm
    nodejs
    nodePackages.svelte-language-server
  ];

  shellHook = ''
    export REKOMENDACJE_DATA_DIR="$(readlink -f 03-rekomendacje)/clean"
  '';
}
