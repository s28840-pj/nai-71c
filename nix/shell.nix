{
  mkShell,
  rustc,
  cargo,
  rust-analyzer,
  clippy,
  rustfmt,
  pnpm,
  nodejs,
  nodePackages,
  cmake,
  pkg-config,
  sdl12-compat,
  zlib,
  libtorch-bin,
  npins,
  stdenv,
  ale-roms,
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
    cmake
    pkg-config
    sdl12-compat
    zlib
    libtorch-bin
    npins
    stdenv.cc.cc.lib
  ];

  shellHook = ''
    export REKOMENDACJE_DATA_DIR="$(readlink -f 03-rekomendacje)/clean"
  '';

  LIBTORCH = libtorch-bin;
  LIBTORCH_INCLUDE = libtorch-bin.dev;

  ATARI_ROM_DIR = ale-roms;
}
