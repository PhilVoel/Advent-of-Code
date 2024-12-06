{
	inputs = {
		nixpkgs.url = "nixpkgs/nixos-24.11";
		jconsole = {
			url = "https://www.jsoftware.com/download/j9.5/install/j9.5_linux64.tar.gz";
			flake = false;
		};
	};

	outputs = {jconsole, nixpkgs, ...}:
		let
			pkgs = nixpkgs.legacyPackages.x86_64-linux;
			jconsolePkg = pkgs.stdenv.mkDerivation (finalAttrs: {
				pname = "jconsole";
				version = "9.5";

				src = jconsole; 

				installPhase = ''
					mkdir -p $out
					cp -r $src/* $out
				'';
			});
		in {
			devShells.x86_64-linux.default = pkgs.mkShell {
				packages = [jconsolePkg];
			};
		};
}
