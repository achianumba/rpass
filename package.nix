let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-25.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };

  rpassf = { pkgs }:
    pkgs.rustPlatform.buildRustPackage rec {
      pname = "rpass";
      version = "0.1.0";
      src = pkgs.lib.sources.cleanSource ./.;

      cargoLock = {
        lockFile = "${src.outPath}/Cargo.lock";
      };

      meta = with pkgs.lib; {
        description = "A password manager for the CLI";
        homepage = "https://github.com/achianumba/rpass";
        license = licenses.mit;
      };
    };
in
{
  rpass = pkgs.callPackage rpassf { pkgs = pkgs; };
}

