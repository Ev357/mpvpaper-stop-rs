{
  lib,
  craneLib,
}: let
  cargoToml = fromTOML (builtins.readFile ../Cargo.toml);

  pname = cargoToml.package.name;

  commonArgs = {
    inherit pname;
    version = cargoToml.package.version;

    src = craneLib.cleanCargoSource ../.;

    strictDeps = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
in
  craneLib.buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;

      meta = {
        description = cargoToml.package.description;
        homepage = cargoToml.package.homepage;
        platforms = lib.systems.flakeExposed;
        license = lib.licenses.mit;
        mainProgram = pname;
      };
    }
  )
