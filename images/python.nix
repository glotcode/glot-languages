let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;

  pythonPackages =
    pkgs.python3.withPackages(ps: [
      ps.cachetools
      ps.nose
      ps.numpy
      ps.python-dateutil
      ps.pytz
      ps.six
    ]);
in
build_image {
  pkgs = pkgs;
  name = "glot/python";
  tag = "latest";
  installedPackages = [
    pythonPackages
  ];
}
