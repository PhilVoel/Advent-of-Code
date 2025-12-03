{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-25.11";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, self, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = nixpkgs.legacyPackages."${system}";
			in {
				devShells.default = pkgs.mkShell {
					name = "AoC_2025_09";
					packages = with pkgs; [
						bacon
						cargo
						clippy
						rustfmt
					];
				};
				packages.default = pkgs.rustPlatform.buildRustPackage {
					pname = "AoC_2025_09";
					version = "0.1.0";
					src = self;
					cargoLock.lockFile = ./Cargo.lock;
				};
			}
		);
}
