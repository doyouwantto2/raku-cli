{ pkgs ? import <nixpkgs> { } }:

let
  runtimeLibs = with pkgs; [
    libxkbcommon
    wayland
    wayland-protocols

    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr

    alsa-lib
    udev

    vulkan-loader

    openssl
  ];
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    cargo-edit
    cargo-watch

    clang
    lld
    pkg-config

    vulkan-tools
    vulkan-headers
    vulkan-validation-layers
  ] ++ runtimeLibs;

  shellHook = ''
    # Runtime dynamic libraries (winit / bevy)
    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}"

    # OpenSSL (common Rust crates expect these)
    export OPENSSL_DIR=${pkgs.openssl.dev}
    export OPENSSL_LIB_DIR=${pkgs.openssl.out}/lib
    export OPENSSL_INCLUDE_DIR=${pkgs.openssl.dev}/include
  '';
}
