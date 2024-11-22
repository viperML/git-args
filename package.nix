{
  lib,
  rustPlatform,
  wrapWithGit ? true,
  git,
  makeWrapper,
}:
rustPlatform.buildRustPackage {
  name = "git-args";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
      lib.fileset.unions [
        ./src
        ./Cargo.toml
        ./Cargo.lock
      ]
    );
  };

  nativeBuildInputs = [
    makeWrapper
  ];

  doCheck = false; # faster builds

  postFixup = lib.optionalString wrapWithGit ''
    wrapProgram $out/bin/git \
      --set-default GIT_REAL '${git}/bin/git'
  '';

  cargoLock.lockFile = ./Cargo.lock;
}
