{pkgs ? import (import ./nix/sources.nix {}).nixpkgs {} }:
pkgs.stdenv.mkDerivation{
  name = "rust-lib";
  src = ./.;
  buildInputs = with pkgs;[
    rustc
    cargo
  ];
  installPhase = ''
    cargo check
    cargo test
  '';
  buildPhase = ''
    cargo build
    mv target $out
  '';
  
}
