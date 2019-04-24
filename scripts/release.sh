#!/bin/bash

"$PWD/scripts/build-all.sh"

VERSION="$(cargo read-manifest | jq .version -r)"
echo "Releasing $VERSION on GitHub"

hub release create \
    -a target/x86_64-apple-darwin/release/prompt#prompt-darwin-x86_64
    -a target/x86_64-unknown-linux-gnu/release/prompt#prompt-linux-x86_64
    "$VERSION"
