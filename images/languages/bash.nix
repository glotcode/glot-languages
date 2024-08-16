let
  pkgs =
    import ../common/nixpkgs.nix;

  build_image =
    import ../common/build_image.nix;
in
build_image {
  pkgs = pkgs;
  name = "glot/bash";
  tag = "latest";
  installedPackages = [
    pkgs.procps
    pkgs.utillinux
    pkgs.diffutils
    pkgs.gnugrep
    pkgs.time
    pkgs.file
    pkgs.gawk
    pkgs.gzip
    pkgs.unzip
    pkgs.lzma
    pkgs.gnutar
    pkgs.bzip2
    pkgs.findutils
    pkgs.git
    pkgs.patch
    pkgs.rsync
    pkgs.gnused
    pkgs.jq
    pkgs.lsof
  ];
}
