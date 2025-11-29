{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.pkg-config
    pkgs.openssl
    pkgs.cmake
    pkgs.alsa-oss
    pkgs.alsa-lib
    pkgs.alsa-utils
    pkgs.rustc
    pkgs.cargo
  ];
}
