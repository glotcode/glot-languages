{ pkgs }:
let
  src =
    ./.;

  cmd =
    ''
    mkdir -p $out
    cp ${src}/deps.tar $out/
    '';
in
pkgs.runCommand "glot-clojure-deps" {} cmd
