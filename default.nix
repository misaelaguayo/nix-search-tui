{ nixpkgs ? import <nixpkgs> {} }:

nixpkgs.rustPlatform.buildRustPackage {
    pname = "nix-search-tui";
    version = "0.1.0";
    src = nixpkgs.fetchFromGitHub {
        owner = "misaelaguayo";
        repo = "nix-search-tui";
        rev = "v0.1.0";
        hash = "sha256-Bnuk+28DZJV0M8Do37D58AJpEsleJf09X9x03T2dYVE=";
    };

    nativeBuildInputs = with nixpkgs; [ nix-search-cli ];

    cargoHash = "sha256-yp8+MWpyMv4ILvhuaZcHF/Q9NPmC+mwhcjXOmE9abnk=";
}
