{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-24.11";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, self, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = nixpkgs.legacyPackages."${system}"; in
			{
				devShells.default = pkgs.mkShell {
					name = "AoC24-Fortran";
				};
				packages = builtins.listToAttrs (
					map (day: rec {
							name = "Day_${day}";
							value = pkgs.stdenv.mkDerivation {
								inherit name;
								src = self;

								nativeBuildInputs = [pkgs.gfortran];
								buildPhase = ''gfortran src/${name}.f90 -o ${name}'';

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
