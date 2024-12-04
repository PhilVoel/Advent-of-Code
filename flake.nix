{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/24.05";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, self, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = nixpkgs.legacyPackages."${system}"; in
			{
				devShells.default = pkgs.mkShell {
					name = "AoC24-C";
				};
				packages = builtins.listToAttrs (
					map (day: rec {
							name = "Day_${day}";
							value = pkgs.stdenv.mkDerivation {
								inherit name;
								src = self;

								nativeBuildInputs = [pkgs.gcc];
								buildPhase = ''gcc src/Day_${day}.c -o ${name}'';

								installPhase = ''
									mkdir -p $out/bin
									install -t $out/bin ${name};
								'';
							};
					})
					(builtins.genList (x: toString (x+1)) 25)
				);
			}
		);
}
