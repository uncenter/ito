{
  lib,
  rustPlatform,
}:
rustPlatform.buildRustPackage {
  pname = "sttr-rs";
  inherit ((lib.importTOML ../Cargo.toml).package) version;

  src = ../.;
  cargoLock = {
    lockFile = ../Cargo.lock;

    outputHashes = {
      "words-count-0.1.6" = "sha256-1oaeP419hnAbp283AB3f8bXi/VCE0CKMZiJvpTr1mWo=";
    };
  };

  meta = {
    description = "Easily change strings and convert string representations of data";
    homepage = "https://github.com/uncenter/sttr-rs";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [uncenter];
    mainProgram = "sttr";
  };
}
