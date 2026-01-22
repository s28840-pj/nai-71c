{ fetchurl, runCommand }:
let
  roms = fetchurl {
    url = "https://gist.githubusercontent.com/jjshoots/61b22aefce4456920ba99f2c36906eda/raw/00046ac3403768bfe45857610a3d333b8e35e026/Roms.tar.gz.b64";
    hash = "sha256-Asp3fBZHanL6NmgKK6ePJMOsMbIVUDNUml83oGUxF94=";
  };
in
runCommand "atari-roms" { } ''
  base64 -d "${roms}" > roms.tar.gz
  mkdir -p "$out"
  tar -xf roms.tar.gz -C "$out" --strip-components=2
''
