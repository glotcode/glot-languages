let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;

  elmRunner =
    import ./elm/elm-runner/default.nix { pkgs = pkgs; };

  projectFiles =
    import ./elm/project.nix { pkgs = pkgs; };
in
build_image {
  pkgs = pkgs;
  name = "glot/elm";
  tag = "latest";
  installedPackages = [
    pkgs.elmPackages.elm
    pkgs.nodejs
    elmRunner
  ];
  run = ''
    # Prepare elm project
    ${pkgs.gnutar}/bin/tar -xf ${projectFiles}/project.tar -C /home/glot
    ${pkgs.coreutils}/bin/chown -R glot:glot /home/glot
  '';
}
