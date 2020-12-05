let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/java";
  tag = "latest";
  installedPackages = [
    pkgs.jdk
  ];
  env = [
    "PATH=${pkgs.jdk}/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
    "JAVA_HOME=${pkgs.jdk}/lib/openjdk"
  ];
}
