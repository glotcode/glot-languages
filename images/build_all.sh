#!/usr/bin/env bash
set -e

mkdir -p result
nix-build --out-link result/assembly languages/assembly.nix
nix-build --out-link result/ats languages/ats.nix
nix-build --out-link result/bash languages/bash.nix
nix-build --out-link result/cobol languages/cobol.nix
nix-build --out-link result/coffeescript languages/coffeescript.nix
nix-build --out-link result/clang languages/clang.nix
nix-build --out-link result/clisp languages/clisp.nix
nix-build --out-link result/clojure languages/clojure.nix
nix-build --out-link result/crystal languages/crystal.nix
nix-build --out-link result/dlang languages/dlang.nix
nix-build --out-link result/dart languages/dart.nix
nix-build --out-link result/elixir languages/elixir.nix
nix-build --out-link result/elm languages/elm.nix
nix-build --out-link result/erlang languages/erlang.nix
nix-build --out-link result/golang languages/golang.nix
nix-build --out-link result/groovy languages/groovy.nix
nix-build --out-link result/guile languages/guile.nix
nix-build --out-link result/hare languages/hare.nix
nix-build --out-link result/haskell languages/haskell.nix
nix-build --out-link result/idris languages/idris.nix
nix-build --out-link result/java languages/java.nix
nix-build --out-link result/javascript languages/javascript.nix
nix-build --out-link result/julia languages/julia.nix
nix-build --out-link result/kotlin languages/kotlin.nix
nix-build --out-link result/lua languages/lua.nix
nix-build --out-link result/mercury languages/mercury.nix
nix-build --out-link result/csharp languages/csharp.nix
nix-build --out-link result/fsharp languages/fsharp.nix
nix-build --out-link result/nim languages/nim.nix
nix-build --out-link result/nix languages/nix.nix
nix-build --out-link result/ocaml languages/ocaml.nix
nix-build --out-link result/pascal languages/pascal.nix
nix-build --out-link result/perl languages/perl.nix
nix-build --out-link result/php languages/php.nix
nix-build --out-link result/python languages/python.nix
nix-build --out-link result/raku languages/raku.nix
nix-build --out-link result/ruby languages/ruby.nix
nix-build --out-link result/rust languages/rust.nix
nix-build --out-link result/sac languages/sac.nix
nix-build --out-link result/scala languages/scala.nix
nix-build --out-link result/swift languages/swift.nix
nix-build --out-link result/typescript languages/typescript.nix
nix-build --out-link result/zig languages/zig.nix
