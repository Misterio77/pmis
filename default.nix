{ lib, rustPlatform, openssl, pkgconfig }:

let manifest = (lib.importTOML ./Cargo.toml).package;
in rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = [openssl pkgconfig];

  meta = with lib; {
    description = manifest.desciption;
    homepage = manifest.homepage;
    license = licenses.gpl3Plus;
    platforms = platforms.all;
  };
}
