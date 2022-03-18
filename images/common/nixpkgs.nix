let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-unstable";
      rev = "73ad5f9e147c0d2a2061f1d4bd91e05078dc0b58";
    };

  pkgs =
    import nixpkgs {};
in
pkgs
