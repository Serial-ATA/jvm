#!/usr/bin/env just --justfile

# -----------------------------------------------------------------------------
# TARGETS:
# -----------------------------------------------------------------------------

default: debug

# Build the entire project in debug
debug: cargo build

# Build the entire project in release
release: cargo build --release

# Build and run the java binary with the provided arguments
java +ARGS: debug
    cd tools/java &&\
    cargo run -- {{ ARGS }}
