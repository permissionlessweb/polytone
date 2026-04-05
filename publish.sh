#!/bin/bash

PACKAGES="
  cw-orch-polytone
"
CONTRACTS="proxy note voice"

for lib in $CONTRACTS; do
  (
    cd "contracts/main/$lib"
    echo "Publishing $lib"
    cargo publish
  )
done

for pack in $PACKAGES; do
  (
    cd "packages/$pack"
    echo "Publishing $pack"
    cargo publish
  )
done