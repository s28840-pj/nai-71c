{
  stdenvNoCC,
  nodejs,
  pnpm,
}:

stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "ad-watcher";
  version = "0.0.1";

  src = ../06-ad-watcher;

  nativeBuildInputs = [
    nodejs
    pnpm.configHook
  ];

  pnpmDeps = pnpm.fetchDeps {
    inherit (finalAttrs) pname version src;
    fetcherVersion = 2;
    hash = "sha256-PMvLZ+MO1EI6Bm/Gh+b30S3q5CsYu9FBz6BPxNHUZ+k=";
  };

  buildPhase = "pnpm run build";

  installPhase = "cp -r build $out";
})
