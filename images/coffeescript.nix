let
  pkgs =
    import ../common/nixpkgs.nix;

  build_image =
    import ../common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/coffeescript";
  tag = "latest";
  installedPackages = [
    pkgs.nodePackages.coffee-script
  ];
  env = [
    "PATH=${pkgs.nodePackages.coffee-script}/bin/:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
  ];
}
