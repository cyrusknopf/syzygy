{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=release-23.11";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in with pkgs; rec {
        devShell = mkShell {
          buildInputs = [
            rustc
            cargo
            openssl
            pkg-config
            libxkbcommon
            libGL

            # Wayland
            wayland

            # X11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            xorg.libX11
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$(echo ${lib.makeLibraryPath [
              rustc
              cargo
              openssl
              pkg-config
              libxkbcommon
              libGL
              wayland
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
            ]})"
            export WINIT_UNIX_BACKEND=x11  # Force X11 backend
            # Uncomment the following line to use Wayland instead of X11
            # export WINIT_UNIX_BACKEND=wayland
          '';
        };
      });
}
