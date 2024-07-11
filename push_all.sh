#!/usr/bin/env bash
set -e

for image in result/*; do
  name="glot/$(basename $image)"

  echo "Pushing $name..."
  docker push "$name"
done

