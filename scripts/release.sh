#!/bin/bash

"$PWD/scripts/build-all.sh"

VERSION="$(cargo read-manifest | jq .version -r)"
echo "Releasing $VERSION on GitHub"

# Hub requires the actual filenames to be different 
cp target/x86_64-apple-darwin/release/prompt target/x86_64-apple-darwin/release/prompt-darwin-x86_64
cp target/x86_64-unknown-linux-gnu/release/prompt target/x86_64-unknown-linux-gnu/release/prompt-linux-x86_64

hub release create \
    -a target/x86_64-apple-darwin/release/prompt-darwin-x86_64 \
    -a target/x86_64-unknown-linux-gnu/release/prompt-linux-x86_64 \
    -m "prompt_ben v$VERSION" -e \
    "v$VERSION"
