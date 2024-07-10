let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-unstable";
      rev = "1d7db1b9e4cf1ee075a9f52e5c36f7b9f4207502";
    };

  pkgs =
    import nixpkgs {};

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/swift";
  tag = "latest";
  installedPackages = [
    pkgs.binutils
    pkgs.swift
  ];
}
