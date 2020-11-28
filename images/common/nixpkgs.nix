let
  nixpkgs =
    builtins.fetchGit (builtins.fromJSON (builtins.readFile ./nixpkgs.json));

  pkgs =
    import nixpkgs {};
in
pkgs
