#!/usr/bin/env bash
set -e

echo "Pruning old images..."
docker system prune -a

for image in result/*; do
  echo "Loading $image..."
  docker load -i "$image"
done
