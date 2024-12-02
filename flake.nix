{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/24.05";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, self, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let
				ghc-with-libs = pkgs.haskellPackages.ghcWithPackages
					(haskellPackages: with haskellPackages; [
					]);
				pkgs = nixpkgs.legacyPackages."${system}";
			in {
				devShells.default = pkgs.mkShell {
					name = "AoC24-Haskell";
					packages = [ghc-with-libs];
				};
				packages = builtins.listToAttrs (
					map (day: rec {
							name = "Day_${day}";
							value = pkgs.stdenv.mkDerivation {
								inherit name;
								src = self;

								nativeBuildInputs = [ghc-with-libs];
								buildPhase = ''ghc -working-dir src -DDAY=\"${day}\" -o ${name} Main.hs ${name}.hs'';

								installPhase = ''
									mkdir -p $out/bin
									install -t $out/bin ${name};
								'';
							};
					})
					(builtins.genList (x: toString (x+1)) 2)
				);
			}
		);
}
