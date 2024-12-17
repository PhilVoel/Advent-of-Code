{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-24.11";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, self, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = nixpkgs.legacyPackages."${system}";
			in {
				devShells.default = pkgs.mkShell {
					name = "AoC24-Rust";
					packages = with pkgs; [
						bacon
						cargo
					];
				};
				packages = builtins.listToAttrs (
					map (day: rec {
							name = "Day_${day}";
							value = pkgs.rustPlatform.buildRustPackage rec {
								pname = name;
								version = "1";
								src = "${self}/${name}";
								cargoLock.lockFile = "${src}/Cargo.lock";
							};
					})
					(builtins.genList (x: toString (x+1)) 25)
				);
			}
		);
}
