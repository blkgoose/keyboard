{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;

          config.allowUnfreePredicate = pkg:
            builtins.elem (nixpkgs.lib.getName pkg) [ "adafruit-nrfutil" ];
        };
        xz = "${nixpkgs.lib.makeLibraryPath [ pkgs.xz.dev ]}/pkgconfig";
        libudev =
          "${nixpkgs.lib.makeLibraryPath [ pkgs.libudev-zero ]}/pkgconfig";

        elf2uf2 = pkgs.rustPlatform.buildRustPackage {
          pname = "elf2uf2-rs";
          version = "2.0.0";

          doCheck = false;

          src = pkgs.fetchFromGitHub {
            owner = "simmsb";
            repo = "elf2uf2-rs";
            rev = "034d4ed6fe4b8cf435436c73e98ffb1d64afcf92";
            sha256 = "sha256-EgExfWVsI14t+v6J5hYk8IUy4kaJLUK/Qsstc81INMs=";
          };

          nativeBuildInputs = [ pkgs.pkg-config ];

          buildInputs = [ pkgs.udev ];

          cargoHash = "sha256-tiOLxAKVrPrePVYs1KOc2vLnLjwjrhSQKL15yU3q/u8=";
        };

        runner = pkgs.writers.writeBashBin "runner" ''
          set -e

          output=$1
          input=$2

          if [ ! -e $output ]; then
            echo "ERROR: output not found"
            exit 1
          fi

          ${elf2uf2}/bin/elf2uf2-rs $input /tmp/runner-keyboard-build.uf2
          sudo cp /tmp/runner-keyboard-build.uf2 $output
          rm /tmp/runner-keyboard-build.uf2
        '';

      in {
        devShell = pkgs.mkShell {
          PKG_CONFIG_PATH = nixpkgs.lib.concatStringsSep ":" [ xz libudev ];

          buildInputs = with pkgs; [
            xz
            lld
            bacon
            cargo-make
            cargo-watch
            cargo-binutils
            usbutils
            runner
            adafruit-nrfutil
          ];
        };
      });
}
