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
      in {
        devShell = pkgs.mkShell {
          PKG_CONFIG_PATH = nixpkgs.lib.concatStringsSep ":" [ xz libudev ];

          buildInputs = with pkgs; [ xz lld ];
        };
      });
}
