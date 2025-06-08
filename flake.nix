{

  description = "Tool for searching nix packages";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      packages = forAllSystems (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in rec {
          default = nix-search-tui;
          nix-search-tui = pkgs.callPackage ./default.nix { nix-search-cli = pkgs.nix-search-cli; };
        });

      apps = forAllSystems (system: rec {
        default = nix-search-tui;
        nix-search-tui = {
          type = "app";
          program = "${lib.getBin self.packages.${system}.nix-search-tui}/bin/nix-search-tui";
        };
      });
    };
}
