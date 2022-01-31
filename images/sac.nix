let
  pkgs =
    import ./common/nixpkgs.nix;

  nix-sac =
    builtins.fetchGit {
      url = "https://github.com/cxandru/sac-nix";
      ref = "refs/heads/main";
      rev = "b5c84342906aef3373d9a6e8b6b6933e1c4ea426";
    };

  sacpkgs =
    import nix-sac;

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/sac";
  tag = "latest";
  installedPackages = [
    pkgs.gcc
    pkgs.gnused
    sacpkgs.packages.x86_64-linux.sac2c
    sacpkgs.packages.x86_64-linux.stdlib
  ];
}
