{
    fetchFromGitHub,
    rustPlatform,
    nix-search-cli,
}:

rustPlatform.buildRustPackage {
    pname = "nix-search-tui";
    version = "0.2.0";
    src = fetchFromGitHub {
        owner = "misaelaguayo";
        repo = "nix-search-tui";
        rev = "v0.2.0";
        hash = "sha256-Ksm9xZ0mFf5SVVzkHALnWCDT2aQl69IvWZgyR8dR1Mk=";
    };

    nativeBuildInputs = [ nix-search-cli ];

    cargoHash = "sha256-IJeG30YZZNZNrlsewI/sHnQNobqvqmfcY/fP8/WAFKg";
}
