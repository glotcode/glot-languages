#!/usr/bin/env bash
set -e

mkdir -p result
nix-build --out-link result/assembly images/assembly.nix
nix-build --out-link result/ats images/ats.nix
nix-build --out-link result/bash images/bash.nix
nix-build --out-link result/cobol images/cobol.nix
nix-build --out-link result/coffeescript images/coffeescript.nix
nix-build --out-link result/clang images/clang.nix
nix-build --out-link result/clisp images/clisp.nix
nix-build --out-link result/clojure images/clojure.nix
nix-build --out-link result/crystal images/crystal.nix
nix-build --out-link result/dlang images/dlang.nix
nix-build --out-link result/dart images/dart.nix
nix-build --out-link result/elixir images/elixir.nix
nix-build --out-link result/elm images/elm.nix
nix-build --out-link result/erlang images/erlang.nix
nix-build --out-link result/golang images/golang.nix
nix-build --out-link result/groovy images/groovy.nix
nix-build --out-link result/guile images/guile.nix
nix-build --out-link result/hare images/hare.nix
nix-build --out-link result/haskell images/haskell.nix
nix-build --out-link result/idris images/idris.nix
nix-build --out-link result/java images/java.nix
nix-build --out-link result/javascript images/javascript.nix
nix-build --out-link result/julia images/julia.nix
nix-build --out-link result/kotlin images/kotlin.nix
nix-build --out-link result/lua images/lua.nix
nix-build --out-link result/mercury images/mercury.nix
nix-build --out-link result/csharp images/csharp.nix
nix-build --out-link result/fsharp images/fsharp.nix
nix-build --out-link result/nim images/nim.nix
nix-build --out-link result/nix images/nix.nix
nix-build --out-link result/ocaml images/ocaml.nix
nix-build --out-link result/pascal images/pascal.nix
nix-build --out-link result/perl images/perl.nix
nix-build --out-link result/php images/php.nix
nix-build --out-link result/python images/python.nix
nix-build --out-link result/raku images/raku.nix
nix-build --out-link result/ruby images/ruby.nix
nix-build --out-link result/rust images/rust.nix
nix-build --out-link result/sac images/sac.nix
nix-build --out-link result/scala images/scala.nix
nix-build --out-link result/swift images/swift.nix
nix-build --out-link result/mercury images/mercury.nix
nix-build --out-link result/typescript images/typescript.nix
nix-build --out-link result/zig images/zig.nix
