{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "ito";
  inherit ((lib.importTOML ../Cargo.toml).package) version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = "A powerful string manipulation tool";
    homepage = "https://github.com/uncenter/ito";
    license = lib.licenses.mit;
    mainProgram = "ito";
  };
}
