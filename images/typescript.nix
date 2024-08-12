let
  pkgs =
    import ../common/nixpkgs.nix;

  build_image =
    import ../common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/typescript";
  tag = "latest";
  installedPackages = [
    pkgs.nodePackages.typescript
  ];
  env = [
    "PATH=${pkgs.nodePackages.typescript}/bin/:${pkgs.nodejs}/bin/:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
  ];
}
