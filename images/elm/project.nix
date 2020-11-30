{ pkgs }:
let
  src =
    ./.;

  cmd =
    ''
    mkdir -p $out
    ${pkgs.gzip}/bin/zcat ${src}/project.tar.gz > $out/project.tar
    '';
in
pkgs.runCommand "glot-elm-project" {} cmd
