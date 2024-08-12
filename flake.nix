{
  description = "Nix flake for wired";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (self: super: rec { }) ];
        pkgs = import nixpkgs { inherit overlays system; };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            podman
            buildah
            bash
            python312Packages.cram
          ];

          shellHook = ''
            echo "Ready to wire"
          '';
        };
      }
    );
}
