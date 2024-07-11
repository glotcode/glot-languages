let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/release-22.11";
      rev = "0218941ea68b4c625533bead7bbb94ccce52dceb";
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
