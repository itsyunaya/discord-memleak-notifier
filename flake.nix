{
	description = "A simple program for Linux and macOS to detect Discord resource leaks";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
		crane.url = "github:ipetkov/crane";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, crane, flake-utils, ... }:
		flake-utils.lib.eachDefaultSystem (system:
		let
			pkgs = nixpkgs.legacyPackages.${system};
			craneLib = crane.mkLib pkgs;
		in {
			packages.dmn = craneLib.buildPackage {
				src = craneLib.cleanCargoSource ./.;
		};

		packages.default = self.packages.${system}.dmn;
	});
}