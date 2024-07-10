#!/usr/bin/env bash
set -e

for image in result/*; do
  name="glot/${image}"

  echo "Pushing $name..."
  #docker push "$image"
done

