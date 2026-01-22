{
  lib,
  sdl2-compat,
  cmake,
  ninja,
  fetchFromGitHub,
  stdenv,
}:
stdenv.mkDerivation (finalAttrs: {
  pname = "sdl12-compat";
  version = "1.2.72";

  src = fetchFromGitHub {
    owner = "libsdl-org";
    repo = "sdl12-compat";
    tag = "release-${finalAttrs.version}";
    hash = "sha256-dTBsbLJFQSaWWhn1+CCQopq7sBONxvlaAximmo3iYVM=";
  };

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [
    sdl2-compat
  ];

  # SDL3 is dlopened at runtime, leave it in runpath
  # dontPatchELF = true;

  cmakeFlags = [
    (lib.cmakeFeature "CMAKE_INSTALL_RPATH" (lib.makeLibraryPath [ sdl2-compat ]))
  ];

  postFixup = ''
    # allow as a drop in replacement for SDL1.2
    # Can be removed after treewide switch from pkg-config to pkgconf
    ln -s $out/lib/pkgconfig/sdl12_compat.pc $out/lib/pkgconfig/sdl12.pc
  '';
})
