let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/release-24.05";
      rev = "abd29679271a9fbcffe1dd640fc6c2a77957f5ed";
    };

  pkgs =
    import nixpkgs {};
in
pkgs
