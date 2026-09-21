with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "rs";
  nativeBuildInputs = [
    rustc
    cargo
    rust-analyzer
  ];
  buildInputs = [
  ];

  # Tell compiler where to look for shared libraries
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
