{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
    pkg-config
    libx11
    libxi
    libxkbcommon
    libGL
    alsa-lib
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.libx11
    pkgs.libxi
    pkgs.libxkbcommon
    pkgs.libGL
    pkgs.alsa-lib
  ];
}
