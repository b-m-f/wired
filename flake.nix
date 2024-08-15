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
            python312Packages.cram
            just
            wireguard-tools
            pass
          ];

          shellHook = ''
            echo "Ready to wire"
          '';
        };
      }
    );
}
