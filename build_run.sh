#!/bin/bash

echo "Building Rust guest code..." \
    && cd guest_rs \
    && cargo clean \
    && cargo build --release \
    && cd .. \
    && echo "Inlining binary into cairo module..." \
    && python3 convert.py \
    && echo "Building and running host code..." \
    && cd host_cairo \
    && scarb cairo-run \
    && cd .. \
    && echo "All runs completed successfully."
