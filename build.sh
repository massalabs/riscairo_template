#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

echo "Building rust guest..."
cd guest_rs
cargo clean
cargo build --release
cd ..
echo "Converting elf files..."
python3 convert.py
echo "Compiling cairo host..."
cd host_cairo
scarb clean
scarb --release build
cd ..
echo "Build successful."

