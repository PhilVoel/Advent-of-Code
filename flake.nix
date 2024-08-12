{
	description = "Advent of Code 2023 Ocaml";
	inputs.nixpkgs.url = "github:nixos/nixpkgs/24.05";

	outputs = {nixpkgs, ...}:
	let
		system = "x86_64-linux";
		pkgs = nixpkgs.legacyPackages."${system}".pkgs;
	in {
		devShells."${system}".default = pkgs.mkShell {
			name = "Ocaml Shell";
			buildInputs = with pkgs; [
				dune_3
				ocaml
			] ++ (with ocamlPackages;
			[
				findlib
				ocamlformat
			]);
			SHELL_FLAKE_PATH="\\/home\\/philipp\\/Programming\\/Advent of Code\\/2023-OCaml";
			SHELL_FLAKE_PATH_NO_SPACES="file:\\/\\/\\/home\\/philipp\\/Programming\\/Advent%20of%20Code\\/2023-OCaml";
		};
	};
}
