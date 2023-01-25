#! /usr/bin/env bash

ALL_PACKAGES=(
    "blackbox_cast_codegen"
    "blackbox_cast"
    "blackbox_core_codegen"
    "blackbox_core"
    "blackbox_di"
)

# Publish all the packages.
for package in "${ALL_PACKAGES[@]}"; do
  echo ":::: Publishing '${package}'..."

  cargo publish --no-verify --allow-dirty -p ${package}

  # Give the index some time to update so the deps are there if we need them.
  sleep 5
done
