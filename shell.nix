with import <nixpkgs> {};
mkShell {
  buildInputs = [
    cargo
    rustc
    rls
  ];

  RUST_BACKTRACE=1;
}
