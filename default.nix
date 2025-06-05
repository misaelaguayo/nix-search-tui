{ nixpkgs ? import <nixpkgs> {} }:

nixpkgs.rustPlatform.buildRustPackage {
    pname = "nix-search-tui";
    version = "0.1.1";
    src = nixpkgs.fetchFromGitHub {
        owner = "misaelaguayo";
        repo = "nix-search-tui";
        rev = "v0.1.1";
        hash = "sha256-/B5n19FpYbbFtMVx3K7jBl6uBbesPNEf64Nxw3wvRmY";
    };

    nativeBuildInputs = with nixpkgs; [ nix-search-cli ];

    cargoHash = "sha256-IJeG30YZZNZNrlsewI/sHnQNobqvqmfcY/fP8/WAFKg=";
}
