#!/usr/bin/env bash

mkdir -p result
nix build --out-link result/python --file images/python.nix
nix build --out-link result/haskell --file images/haskell.nix
