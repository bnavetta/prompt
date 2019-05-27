#!/bin/bash

# 1. Compile for macOS (assumes we're running on a macOS host)
echo "Compiling for macOS..."
cargo build --release

echo ""
echo ""

# 2. Compile for Linux
echo "Compiling for Linux..."
if ! docker volume inspect cargo-cache >/dev/null 2>&1; then
    docker volume create --driver local \
        --opt "o=uid=$(id -u),gid=$(id -g)" \
        --opt type=tmpfs --opt device=tmpfs \
        cargo-cache
fi

docker run \
    --rm \
    --user "$(id -u)":"$(id -g)" \
    -v "$PWD:/usr/src/prompt" \
    -v cargo-cache:/var/cargo \
    -e CARGO_HOME=/var/cargo \
    -w /usr/src/prompt \
    rust \
    cargo build --release --target x86_64-unknown-linux-gnu --color always
