with import <nixpkgs> {};
mkShell {
  buildInputs = [
    cargo
    rustc
    rls
    unzip
    curl
  ];

  RUST_BACKTRACE=1;
}
