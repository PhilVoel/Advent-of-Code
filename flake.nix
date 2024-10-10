{
	description = "Advent of Code 2023 Ocaml";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/24.05";
		flake-utils.url = "github:numtide/flake-utils/v1.0.0";
	};

	outputs = {flake-utils, nixpkgs, ...}:
		flake-utils.lib.eachDefaultSystem (system:
			let pkgs = nixpkgs.legacyPackages."${system}";
			in {
				devShells.default = pkgs.mkShell {
					name = "Ocaml Shell";
					packages = with pkgs; [
						dune_3
						ocaml
					] ++ (with ocamlPackages;
					[
						findlib
						ocamlformat
					]);
				};
			}
		);
}
