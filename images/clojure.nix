let
  pkgs =
    import ./common/nixpkgs.nix;

  build_image =
    import ./common/build_image.nix;

  bootstrap =
    import ./clojure/bootstrap.nix { pkgs = pkgs; };
in
build_image {
  pkgs = pkgs;
  name = "glot/clojure";
  tag = "latest";
  installedPackages = [
    pkgs.clojure
    pkgs.gnutar
    pkgs.gzip
  ];
  run = ''
    # Prepare bootstrap file
    cp ${bootstrap}/bootstrap.tar.gz /
  '';
}
