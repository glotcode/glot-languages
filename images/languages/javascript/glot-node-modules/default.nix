{ pkgs ? import <nixpkgs> {} }:

pkgs.mkYarnPackage {
  name = "glot-node-modules";
  src = ./.;
  packageJSON = ./package.json;
  yarnLock = ./yarn.lock;
  yarnNix = ./yarn.nix;
}
