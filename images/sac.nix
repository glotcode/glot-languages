let
  pkgs =
    import ./common/nixpkgs.nix;

  nix-sac =
    builtins.fetchGit {
      url = "https://github.com/hv15/sac-nix";
      ref = "refs/heads/sac-update";
      rev = "c8f7ab98cca5d6a628eb15c3dac71a42b6ccdfff";
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
