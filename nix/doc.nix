{ nai }:

nai.overrideAttrs {
  pname = "nai-doc";
  dontCargoCheck = true;

  buildPhase = ''
    cargo doc --no-deps --workspace --frozen
  '';

  installPhase = ''
    cp -r target/doc "$out"
    echo "<meta http-equiv=\"refresh\" content=\"0; url=warcaby\">" > "$out/index.html"
  '';
}
