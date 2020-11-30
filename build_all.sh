#!/usr/bin/env bash

mkdir -p result
#nix build --out-link result/python --file images/python.nix
#nix build --out-link result/haskell --file images/haskell.nix
#nix build --out-link result/assembly --file images/assembly.nix
#nix build --out-link result/ats --file images/ats.nix
#nix build --out-link result/bash --file images/bash.nix
#nix build --out-link result/clang --file images/clang.nix
#nix-build --out-link result/clojure images/clojure.nix
nix-build --out-link result/elm images/elm.nix
