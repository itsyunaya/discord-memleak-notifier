{
	description = "A simple program for Linux and macOS to detect Discord resource leaks";

	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

	outputs = { self, nixpkgs }: let
		systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
		forAllSystems = f: nixpkgs.lib.genAttrs systems f;
	in {
		packages = forAllSystems (sys: let
			pkgs = nixpkgs.legacyPackages.${sys};
		in {
			default = pkgs.callPackage ./. { };
		});
	};
}
