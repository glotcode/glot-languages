let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-unstable";
      rev = "ad47284f8b01f587e24a4f14e0f93126d8ebecda";
    };

  pkgs =
    import nixpkgs {};
in
pkgs
