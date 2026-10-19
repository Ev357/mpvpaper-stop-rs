{
  description = "mpvpaper-stop-rs";

  nixConfig = {
    extra-substituters = [
      "https://fenix.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "fenix.cachix.org-1:ecJhr+RdYEdcVgUkjruiYhjbBloIEGov7bos90cZi0Q="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    fenix,
    ...
  }: let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;

    mkPerSystem = perSystem:
      forAllSystems (system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            self.overlays.default
            fenix.overlays.default
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain fenix.packages.${system}.default.toolchain;
      in
        perSystem {inherit pkgs craneLib system;});
  in {
    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    packages = mkPerSystem ({
      pkgs,
      craneLib,
      system,
    }: {
      mpvpaper-stop-rs = pkgs.callPackage ./nix/mpvpaper-stop-rs.nix {inherit craneLib;};
      default = self.packages.${system}.mpvpaper-stop-rs;
    });

    devShells = mkPerSystem ({
      pkgs,
      craneLib,
      ...
    }: {
      default = pkgs.callPackage ./nix/shell.nix {inherit craneLib;};
    });

    overlays = {
      mpvpaper-stop-rs = final: _: let
        system = final.stdenv.hostPlatform.system;
      in {
        mpvpaper-stop-rs = self.packages.${system}.mpvpaper-stop-rs;
      };
      default = self.overlays.mpvpaper-stop-rs;
    };
  };
}
