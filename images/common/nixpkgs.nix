let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/release-24.05";
      rev = "8f4cb508c33212aa69ae22958d03c0ba9a906f5b";
    };

  pkgs =
    import nixpkgs {};
in
pkgs
