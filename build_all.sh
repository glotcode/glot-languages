#!/usr/bin/env bash

mkdir -p result
nix build --out-link result/python --file images/python.nix
