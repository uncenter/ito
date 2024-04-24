{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "sttr-rs";
  inherit ((lib.importTOML ../Cargo.toml).package) version;

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = "Easily change strings and convert string representations of data";
    homepage = "https://github.com/uncenter/sttr-rs";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [uncenter];
    mainProgram = "sttr";
  };
}
