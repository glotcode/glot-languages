let
  nixpkgs =
    builtins.fetchGit (builtins.fromJSON (builtins.readFile ./common/nixpkgs.json));

  pkgs =
    import nixpkgs {};

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/assembly";
  tag = "latest";
  installedPackages = [
    pkgs.binutils
    pkgs.nasm
  ];
}
