let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;

  elmRunner =
    import ./elm/elm-runner/default.nix { pkgs = pkgs; };

  bootstrap =
    import ./elm/bootstrap.nix { pkgs = pkgs; };
in
build_image {
  pkgs = pkgs;
  name = "glot/elm";
  tag = "latest";
  installedPackages = [
    pkgs.elmPackages.elm
    pkgs.nodejs
    pkgs.gnutar
    pkgs.gzip
    elmRunner
  ];
  run = ''
    # Prepare bootstrap file
    cp ${bootstrap}/bootstrap.tar.gz /
  '';
}
