{ self, pkgs, lib, config, inputs, ... }:

{
  name = "bevy_brickbreaker";
  cachix.enable = false;
  starship.enable = true;

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };

  packages = with pkgs; [
    git
    gitui

    # unstable.jetbrains.jdk
    # unstable.jetbrains.rust-rover

    alsa-lib
    libudev-zero

    libxkbcommon
    vulkan-loader
    
    # WINIT_UNIX_BACKEND=wayland
    wayland

    # WINIT_UNIX_BACKEND=x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11
  ];

  env.LIB_PATH = with pkgs; lib.makeLibraryPath [
      alsa-lib
      libudev-zero

      libxkbcommon
      vulkan-loader
  ];

  enterShell = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$LIB_PATH:$RUST_SRC_PATH";
  '';
}

