{ lib, rustPlatform }:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  inherit (manifest) version;

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  # When building with nix, generated code is 1 folder deeper,
  # so we need to adjust the paths that refer to included files.
  patchPhase = ''
    substituteInPlace build.rs \
      --replace "../../../../../" "../../../../../../"
  '';

  meta = {
    inherit (manifest) description;
    homepage = manifest.repository;
    license = lib.licenses.mit;
    platforms = lib.platforms.all;
  };
}
