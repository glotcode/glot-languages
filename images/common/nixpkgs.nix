let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-20.09";
      rev = "ffb3aab257e8851b558cdc6079241a7eb0c7239e";
    };

  pkgs =
    import nixpkgs {};
in
pkgs
