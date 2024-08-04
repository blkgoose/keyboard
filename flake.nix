{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        xz = "${nixpkgs.lib.makeLibraryPath [ pkgs.xz.dev ]}/pkgconfig";
        libudev =
          "${nixpkgs.lib.makeLibraryPath [ pkgs.libudev-zero ]}/pkgconfig";

        runner = pkgs.writers.writeBashBin "runner" ''
          output=$1
          input=$2

          elf2uf2-rs $input /tmp/runner-keyboard-build.uf2
          cp /tmp/runner-keyboard-build.uf2 $output
          rm /tmp/runner-keyboard-build.uf2
        '';

      in {
        devShell = pkgs.mkShell {
          PKG_CONFIG_PATH = nixpkgs.lib.concatStringsSep ":" [ xz libudev ];

          buildInputs = with pkgs; [
            xz
            lld
            probe-rs
            runner
            bacon
            cargo-make
            cargo-watch
            cargo-binutils
          ];
        };
      });
}
