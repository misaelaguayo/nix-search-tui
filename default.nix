{ nixpkgs ? import <nixpkgs> {} }:

nixpkgs.rustPlatform.buildRustPackage {
    pname = "nix-search-tui";
    version = "0.0.1";
    src = nixpkgs.fetchFromGitHub {
        owner = "misaelaguayo";
        repo = "nix-search-tui";
        rev = "v0.0.1";
        hash = "sha256-j3IoWr/hcPCXv5O3SSSmYi8QZ3HOYwaJTkPIAscLaFQ=";
    };

    # Skip tests as they require the nix-search-cli binary, but we don't have that in pipelines
    doCheck = false;

    cargoHash = "sha256-/CkguHPmTl3DdQQl6S8H6mcb+cSrfU2Osv417cf6uUw=";
}
