{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    clippy
    rustc
    rustfmt

    rust-analyzer
  ];
}
