with import <nixpkgs> {};
  mkShell {
    packages = [
      cargo
      rustc
      rustfmt
      clippy
      rust-analyzer
      taplo
    ];
  }
