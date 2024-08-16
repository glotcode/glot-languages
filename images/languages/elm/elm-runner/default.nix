{ pkgs ? import <nixpkgs> {} }:

pkgs.mkYarnPackage {
  name = "elm-runner";
  src = ./.;
  packageJSON = ./package.json;
  yarnLock = ./yarn.lock;
  yarnNix = ./yarn.nix;
}
