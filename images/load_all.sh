#!/usr/bin/env bash
set -e


for image in result/*; do
  echo "Loading $image..."
  docker load -i "$image"
done
