#!/usr/bin/env bash
set -euo pipefail

# Ensure we are in the root of a Cargo project
if [ ! -f "Cargo.toml" ]; then
    echo "Error: No Cargo.toml found. Run this script from the root of your Cargo project."
    exit 1
fi

# Create builds folder if it doesn't exist
mkdir -p builds
mkdir -p builds/linux_x86-64
mkdir -p builds/windows_x86-64

# Get list of example names from Cargo
examples=$(cargo metadata --no-deps --format-version=1 \
    | jq -r '.packages[0].targets[] | select(.kind[] == "example") | .name')

if [ -z "$examples" ]; then
    echo "No examples found in this project."
    exit 0
fi

# Compile each example and copy executable to builds/
for example in $examples; do
    echo "Building example: $example"
    cargo build --release --example "$example"
    cargo build --release --target x86_64-pc-windows-gnu --example "$example"

    # Figure out the target directory (default target/release/examples)
    src="target/release/examples/$example"
    dest="builds/linux_x86-64/$example"

    if [ -f "$src" ]; then
        cp "$src" "$dest"
        echo " -> Copied to $dest"
    else
        echo "Error: Could not find built executable for $example"
    fi

  src="target/x86_64-pc-windows-gnu/release/examples/$example.exe"
  dest="builds/windows_x86-64/$example.exe"

    if [ -f "$src" ]; then
        cp "$src" "$dest"
        echo " -> Copied to $dest"
    else
        echo "Error: Could not find built executable for $example"
    fi
done

echo "All examples built and copied to ./builds/"
cd builds

tar -czf linux_x86-64.tar.gz linux_x86-64
zip -r windows_x86-64.zip windows_x86-64
