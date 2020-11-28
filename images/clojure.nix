let
  nixpkgs =
    builtins.fetchGit (builtins.fromJSON (builtins.readFile ./common/nixpkgs.json));

  pkgs =
    import nixpkgs {};

  build_image =
    import ./common/build_image.nix;

  deps =
    import ./clojure/deps.nix {};
in
build_image {
  pkgs = pkgs;
  name = "glot/clojure";
  tag = "latest";
  installedPackages = [
    pkgs.clojure
  ];
  runCommands = ''
    ## Install dependencies
    ${pkgs.gnutar}/bin/tar -xf ${deps}/deps.tar -C /home/glot
    ${pkgs.coreutils}/bin/chown -R glot:glot /home/glot
  '';
}
