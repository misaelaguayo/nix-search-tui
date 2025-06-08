{
    fetchFromGitHub,
    rustPlatform,
    nix-search-cli,
}:

rustPlatform.buildRustPackage {
    pname = "nix-search-tui";
    version = "0.1.1";
    src = fetchFromGitHub {
        owner = "misaelaguayo";
        repo = "nix-search-tui";
        rev = "v0.1.1";
        hash = "sha256-/B5n19FpYbbFtMVx3K7jBl6uBbesPNEf64Nxw3wvRmY";
    };

    nativeBuildInputs = [ nix-search-cli ];

    cargoHash = "sha256-IJeG30YZZNZNrlsewI/sHnQNobqvqmfcY/fP8/WAFKg=";
}
