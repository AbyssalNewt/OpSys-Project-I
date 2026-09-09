with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "rs";
  naitiveBuildInputs = [
    rustc
    cargo
  ];
  buildInputs = [
  ];

  # Tell compiler where to look for shared libraries
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
