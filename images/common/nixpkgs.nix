let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-unstable";
      rev = "24eb3f87fc610f18de7076aee7c5a84ac5591e3e";
    };

  pkgs =
    import nixpkgs {};
in
pkgs
