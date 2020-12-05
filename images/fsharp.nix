let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/fsharp";
  tag = "latest";
  installedPackages = [
    pkgs.mono
    pkgs.fsharp
  ];
  env = [
    "MONO_PATH=${pkgs.fsharp}/lib/mono/4.5/"
  ];
}
