let
  nixpkgs =
    builtins.fetchGit (builtins.fromJSON (builtins.readFile ./common/nixpkgs.json));

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
