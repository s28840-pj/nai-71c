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
  ale-roms,
  openssl,
  lib,
  stdenv,
  xorg,
  wayland,
  libxkbcommon,
  rustPlatform,
  libGL,
  vulkan-headers,
  vulkan-loader,
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
  ]
  ++ lib.optionals stdenv.hostPlatform.isLinux [
    openssl
    wayland
    libxkbcommon
    libGL
    vulkan-headers
    vulkan-loader
    rustPlatform.bindgenHook
    xorg.libX11
    xorg.libxcb
    xorg.libXScrnSaver
    xorg.libXcursor
    xorg.libXext
    xorg.libXfixes
    xorg.libXi
    xorg.libXrandr
  ];

  shellHook = ''
    export REKOMENDACJE_DATA_DIR="$(readlink -f 03-rekomendacje)/clean"
  '';

  LIBTORCH = libtorch-bin;
  LIBTORCH_INCLUDE = libtorch-bin.dev;

  ATARI_ROM_DIR = ale-roms;

  LD_LIBRARY_PATH = lib.makeLibraryPath [
    libGL
    vulkan-loader
  ];
}
