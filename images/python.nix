let
  nixpkgs =
    builtins.fetchGit {
      url = "https://github.com/NixOS/nixpkgs";
      ref = "refs/heads/nixos-20.09";
      rev = "ffb3aab257e8851b558cdc6079241a7eb0c7239e";
    };

  pkgs =
    import nixpkgs {};

  pythonPackages =
    pkgs.python3.withPackages(ps: [
      ps.cachetools
      ps.nose
      ps.numpy
      ps.python-dateutil
      ps.pytz
      ps.six
    ]);

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/python";
  tag = "latest";
  installedPackages = [
    pythonPackages
  ];
}
