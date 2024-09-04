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
			packages = with pkgs; [
				dune_3
				ocaml
			] ++ (with ocamlPackages;
			[
				findlib
				ocamlformat
			]);
			shellHook = ''
				export NIX_LDFLAGS="$(echo $NIX_LDFLAGS | sed "s/\/home\/philipp\/Programming\/Advent of Code\/2023-OCaml/file:\/\/\/home\/philipp\/Programming\/Advent%20of%20Code\/2023-OCaml/g")"
			'';
			PROMPT="%F{cyan}%n%F{blue}%F{cyan}%m%F{blue}:%F{magenta}%~ ";
		};
	};
}
