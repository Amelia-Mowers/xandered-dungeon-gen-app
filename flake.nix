
{
  description = "A Rust development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";  # Adjust the channel as necessary
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [];
        };
        buildInputs = with pkgs; [
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          libxkbcommon
          wayland
        ];
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            mold
            clang
            lld
          ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

          shellHook = ''
            export RUSTFLAGS="-C link-arg=-fuse-ld=$(which mold)"
            alias run='cargo run --features bevy/dynamic_linking'
          '';
        };
      }
    );
}

