let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;

  deps =
    import ./clojure/deps.nix { pkgs = pkgs; };
in
build_image {
  pkgs = pkgs;
  name = "glot/clojure";
  tag = "latest";
  installedPackages = [
    pkgs.clojure
  ];
  run = ''
    # Install dependencies
    ${pkgs.gnutar}/bin/tar -xf ${deps}/deps.tar -C /home/glot
    ${pkgs.coreutils}/bin/chown -R glot:glot /home/glot
  '';
}
