with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "rs";
  nativeBuildInputs = [
    rustc
    cargo
    bacon
    rustfmt
    clippy
    rust-analyzer
  ];
  buildInputs = [
  ];

  RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
