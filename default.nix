{ lib, pkgs, pkg-config }:
pkgs.rustPlatform.buildRustPackage {
	pname = "discord-memleak-notifier";
	version = "0.1.0";
	cargoLock.lockFile = ./Cargo.lock;
	src = pkgs.lib.cleanSource ./.;

	nativeBuildInputs = [ pkg-config ];

	meta = {
		description = "A simple program for Linux and macOS to detect Discord resource leaks";
		homepage = "https://github.com/itsyunaya/discord-memleak-notifier";
		license = lib.licenses.gpl3;
		platforms = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
	};
}
