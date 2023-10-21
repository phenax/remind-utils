{ nixpkgs ? import <nixpkgs> { }, haskellPackages ? nixpkgs.haskellPackages, compiler ? "default", doBenchmark ? false }:

let
  inherit (nixpkgs) pkgs;
  systemPackages = [
    haskellPackages.haskell-language-server
    haskellPackages.cabal-install
    pkgs.just
  ];

  commonHsPackages = with haskellPackages; [
    base
    bytestring
    containers
    random
    raw-strings-qq
  ];
in
with haskellPackages; mkDerivation {
  pname = "remind-stuff";
  version = "0.0.0.0";
  src = ./.;
  isLibrary = true;
  isExecutable = true;
  libraryHaskellDepends = commonHsPackages;
  executableHaskellDepends = commonHsPackages;
  executableSystemDepends = systemPackages;
  testHaskellDepends = commonHsPackages ++ [ hspec ];
  license = "MIT";
  hydraPlatforms = stdenv.lib.platforms.none;
}
