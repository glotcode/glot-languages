{ pkgs }:
let
  src =
    ./.;

  cmd =
    ''
    mkdir -p $out
    cp ${src}/deps.tar.gz $out/
    '';
in
pkgs.runCommand "glot-clojure-deps" {} cmd
