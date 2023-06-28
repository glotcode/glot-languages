let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;

  dartPackages =
    pkgs.dartPackages.withPackages(ps: [
      ps.build_runner
      ps.test
      ps.http
      ps.async
      ps.collection
    ]);
in
build_image {
  pkgs = pkgs;
  name = "glot/dart";
  tag = "latest";
  installedPackages = [
    pkgs.dart
    pkgs.dartPackages.sdk
    dartPackages
  ];
}
